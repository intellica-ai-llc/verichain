//! Append-only log over tiles, with O(log n) proofs read from tiles.
use crate::store::TileStore;
use crate::tile::*;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Mutex;
use vc_core::hash::{leaf_hash, node_hash, sha256, to_hex, Hash};

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("corrupt store: {0}")]
    Corrupt(String),
    #[error("index or size out of range")]
    Range,
    #[error("entry too large (max 65535 bytes)")]
    TooLarge,
    #[error("conflict: {0}")]
    Conflict(String),
}

type Res<T> = Result<T, StoreError>;

const MAX_LEVELS: usize = 8; // 256^8 leaves is far beyond u64

/// The commit record is the single linearisation point of an append: entry bundles and hash tiles are written
/// first, the commit record (`size` + Merkle root) last. Anything in the store beyond the committed size is garbage
/// from a failed or crashed append and is ignored (and overwritten by the next append).
const COMMIT_PATH: &str = "meta/commit";
const COMMIT_MAGIC: &str = "vc-tilelog-commit-v1";

pub struct TileLog<S: TileStore> {
    store: S,
    size: u64,
    /// For each stored level: hashes of the current (possibly partial) tile.
    edge: Vec<Vec<Hash>>,
    /// Entry bundle bytes of the current (possibly partial) bundle.
    entry_edge: Vec<u8>,
    /// Bytes of the commit record this instance last read or wrote (None: no record, legacy/new store).
    commit: Option<Vec<u8>>,
    /// A commit write failed with an indeterminate outcome; verify the store before the next append.
    dirty: bool,
    /// Full hash tiles already checked against their parent hash, keyed to the digest of the verified bytes (a
    /// re-read with different bytes is verified again).
    verified: Mutex<HashMap<(u32, u64), Hash>>,
}

fn commit_bytes(size: u64, root: &Hash) -> Vec<u8> {
    format!("{COMMIT_MAGIC}\n{size}\n{}\n", to_hex(root)).into_bytes()
}

fn parse_commit(b: &[u8]) -> Res<(u64, Hash)> {
    let bad = || StoreError::Corrupt("commit record".into());
    let t = std::str::from_utf8(b).map_err(|_| bad())?;
    let mut it = t.split('\n');
    if it.next() != Some(COMMIT_MAGIC) {
        return Err(bad());
    }
    let size_s = it.next().ok_or_else(bad)?;
    let size: u64 = size_s.parse().map_err(|_| bad())?;
    if size.to_string() != size_s {
        return Err(bad());
    }
    let hex = it.next().ok_or_else(bad)?;
    if hex.len() != 64 || it.next() != Some("") || it.next().is_some() {
        return Err(bad());
    }
    let mut root = [0u8; 32];
    for (i, c) in hex.as_bytes().chunks(2).enumerate() {
        let d = |x: u8| match x {
            b'0'..=b'9' => Ok(x - b'0'),
            b'a'..=b'f' => Ok(x - b'a' + 10),
            _ => Err(bad()),
        };
        root[i] = d(c[0])? * 16 + d(c[1])?;
    }
    Ok((size, root))
}

fn bundle_nth(b: &[u8], n: usize) -> Res<&[u8]> {
    let (mut i, mut k) = (0usize, 0usize);
    while i < b.len() {
        let len = u16::from_be_bytes(
            b.get(i..i + 2)
                .ok_or_else(|| StoreError::Corrupt("bundle length".into()))?
                .try_into()
                .unwrap(),
        ) as usize;
        let e = b
            .get(i + 2..i + 2 + len)
            .ok_or_else(|| StoreError::Corrupt("bundle entry".into()))?;
        if k == n {
            return Ok(e);
        }
        i += 2 + len;
        k += 1;
    }
    Err(StoreError::Corrupt("entry offset".into()))
}

fn largest_pow2_below(n: u64) -> u64 {
    1u64 << (63 - (n - 1).leading_zeros())
}

fn mth_slice(h: &[Hash]) -> Hash {
    match h.len() {
        0 => sha256(b""),
        1 => h[0],
        n => {
            let k = largest_pow2_below(n as u64) as usize;
            node_hash(&mth_slice(&h[..k]), &mth_slice(&h[k..]))
        }
    }
}

fn split_hashes(b: &[u8]) -> Res<Vec<Hash>> {
    if !b.len().is_multiple_of(32) {
        return Err(StoreError::Corrupt("hash tile length".into()));
    }
    Ok(b.chunks_exact(32).map(|c| c.try_into().unwrap()).collect())
}

fn join_hashes(h: &[Hash]) -> Vec<u8> {
    h.iter().flatten().copied().collect()
}

impl<S: TileStore> TileLog<S> {
    /// Open (or create) a log. With a commit record the size is authoritative and the derived hash tiles are
    /// checked against the committed root (rebuilt from the entry bundles if they disagree). Without one (new or
    /// pre-commit store) the size is inferred from the entry bundles.
    pub fn open(store: S) -> Res<Self> {
        let mut log = TileLog {
            store,
            size: 0,
            edge: vec![Vec::new(); MAX_LEVELS],
            entry_edge: Vec::new(),
            commit: None,
            dirty: false,
            verified: Mutex::new(HashMap::new()),
        };
        match log.store.get(COMMIT_PATH)? {
            Some(b) => {
                let (size, root) = parse_commit(&b)?;
                log.commit = Some(b);
                log.size = size;
                log.load_partial_bundle()?;
                if size >= TILE_WIDTH
                    && !log
                        .store
                        .exists(&entry_tile_path(size / TILE_WIDTH - 1, TILE_WIDTH))?
                {
                    return Err(StoreError::Corrupt("missing full entry bundle".into()));
                }
                let ok = log.load_edges()?
                    && log.edge0_matches_bundle()
                    && log.root().map(|r| r == root).unwrap_or(false);
                if !ok {
                    log.recover()?;
                    if log.root()? != root {
                        return Err(StoreError::Corrupt(
                            "entries do not match the committed root".into(),
                        ));
                    }
                }
            }
            None => {
                log.size = log.scan_size()?;
                if !(log.load_edges()? && log.edge0_matches_bundle()) {
                    log.recover()?;
                }
            }
        }
        Ok(log)
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    /// Load the current partial entry bundle (exactly `size % 256` entries) from the store.
    fn load_partial_bundle(&mut self) -> Res<()> {
        self.entry_edge.clear();
        let w = self.size % TILE_WIDTH;
        if w == 0 {
            return Ok(());
        }
        let b = self
            .store
            .get(&entry_tile_path(self.size / TILE_WIDTH, w))?
            .ok_or_else(|| StoreError::Corrupt("missing partial entry bundle".into()))?;
        if Self::parse_bundle(&b)?.len() as u64 != w {
            return Err(StoreError::Corrupt("partial entry bundle width".into()));
        }
        self.entry_edge = b;
        Ok(())
    }

    /// Infer the size of a store without commit record: number of full bundles (exponential + binary search on
    /// existence, no content transfer) plus the widest partial bundle.
    fn scan_size(&mut self) -> Res<u64> {
        let full_exists = |s: &S, n: u64| s.exists(&entry_tile_path(n, TILE_WIDTH));
        let mut n = 0u64;
        if full_exists(&self.store, 0)? {
            let (mut lo, mut hi) = (0u64, 1u64);
            while full_exists(&self.store, hi)? {
                lo = hi;
                hi = hi
                    .checked_mul(2)
                    .ok_or_else(|| StoreError::Corrupt("too many bundles".into()))?;
            }
            while hi - lo > 1 {
                let mid = lo + (hi - lo) / 2;
                if full_exists(&self.store, mid)? {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            n = hi;
        }
        let mut count = n * TILE_WIDTH;
        self.entry_edge.clear();
        for w in (1..TILE_WIDTH).rev() {
            if self.store.exists(&entry_tile_path(n, w))? {
                self.size = count + w;
                self.load_partial_bundle()?;
                count += w;
                break;
            }
        }
        Ok(count)
    }

    fn parse_bundle(b: &[u8]) -> Res<Vec<&[u8]>> {
        let mut out = Vec::new();
        let mut i = 0usize;
        while i < b.len() {
            let len = u16::from_be_bytes(
                b.get(i..i + 2)
                    .ok_or_else(|| StoreError::Corrupt("bundle length".into()))?
                    .try_into()
                    .unwrap(),
            ) as usize;
            let e = b
                .get(i + 2..i + 2 + len)
                .ok_or_else(|| StoreError::Corrupt("bundle entry".into()))?;
            out.push(e);
            i += 2 + len;
        }
        Ok(out)
    }

    /// Level-0 edge hashes must be the leaf hashes of the partial bundle.
    fn edge0_matches_bundle(&self) -> bool {
        match Self::parse_bundle(&self.entry_edge) {
            Ok(es) => {
                es.len() == self.edge[0].len()
                    && es
                        .iter()
                        .zip(&self.edge[0])
                        .all(|(e, h)| leaf_hash(e) == *h)
            }
            Err(_) => false,
        }
    }

    /// Load the current partial tile of every stored level; false if anything derived is missing or malformed.
    fn load_edges(&mut self) -> Res<bool> {
        self.verified.lock().unwrap().clear();
        for l in 0..MAX_LEVELS {
            let count = self.size >> (HEIGHT as usize * l);
            let (full, w) = (count / TILE_WIDTH, count % TILE_WIDTH);
            self.edge[l].clear();
            if count == 0 {
                continue;
            }
            if full > 0
                && !self
                    .store
                    .exists(&hash_tile_path(l as u32, full - 1, TILE_WIDTH))?
            {
                return Ok(false);
            }
            if w > 0 {
                match self.store.get(&hash_tile_path(l as u32, full, w))? {
                    Some(b) => match split_hashes(&b) {
                        Ok(h) if h.len() as u64 == w => self.edge[l] = h,
                        _ => return Ok(false),
                    },
                    None => return Ok(false),
                }
            }
        }
        Ok(true)
    }

    /// Rebuild every hash tile from the entry bundles (entries are the source of truth), reading each bundle once.
    fn recover(&mut self) -> Res<()> {
        let full = self.size / TILE_WIDTH;
        let mut edge: Vec<Vec<Hash>> = vec![Vec::new(); MAX_LEVELS];
        let mut counts = [0u64; MAX_LEVELS];
        for n in 0..full {
            let b = self
                .store
                .get(&entry_tile_path(n, TILE_WIDTH))?
                .ok_or_else(|| StoreError::Corrupt("missing entry bundle".into()))?;
            let es = Self::parse_bundle(&b)?;
            if es.len() as u64 != TILE_WIDTH {
                return Err(StoreError::Corrupt("full entry bundle width".into()));
            }
            for e in es {
                Self::push_hash(&self.store, &mut edge, &mut counts, leaf_hash(e), true)?;
            }
        }
        for e in Self::parse_bundle(&self.entry_edge)? {
            Self::push_hash(&self.store, &mut edge, &mut counts, leaf_hash(e), true)?;
        }
        Self::put_edges(&self.store, &edge, self.size)?;
        self.edge = edge;
        self.verified.lock().unwrap().clear();
        Ok(())
    }

    fn put_edges(store: &S, edge: &[Vec<Hash>], size: u64) -> Res<()> {
        for (l, e) in edge.iter().enumerate() {
            if !e.is_empty() {
                let n = (size >> (HEIGHT as usize * l)) / TILE_WIDTH;
                store.put(
                    &hash_tile_path(l as u32, n, e.len() as u64),
                    &join_hashes(e),
                )?;
            }
        }
        Ok(())
    }

    /// Push a leaf hash into the level structure; completed tiles are written as full tiles.
    /// `counts[l]` is the number of hashes ever pushed at level `l` (before this push).
    fn push_hash(
        store: &S,
        edge: &mut [Vec<Hash>],
        counts: &mut [u64],
        h: Hash,
        write: bool,
    ) -> Res<()> {
        let mut level = 0usize;
        let mut cur = h;
        loop {
            edge[level].push(cur);
            counts[level] += 1;
            if edge[level].len() as u64 != TILE_WIDTH {
                return Ok(());
            }
            let tile_hashes = std::mem::take(&mut edge[level]);
            let root = mth_slice(&tile_hashes);
            if write {
                let n = counts[level] / TILE_WIDTH - 1;
                store.put(
                    &hash_tile_path(level as u32, n, TILE_WIDTH),
                    &join_hashes(&tile_hashes),
                )?;
            }
            level += 1;
            if level >= MAX_LEVELS {
                return Err(StoreError::Range);
            }
            cur = root;
        }
    }

    /// Append one entry; returns its index.
    pub fn append(&mut self, entry: &[u8]) -> Res<u64> {
        Ok(self.append_batch(&[entry])?.0)
    }

    /// Append a batch durably (group commit). Returns (first index, new size). Transactional: on error the
    /// instance is unchanged (store garbage beyond the committed size is ignored and overwritten later).
    pub fn append_batch(&mut self, entries: &[&[u8]]) -> Res<(u64, u64)> {
        if entries.iter().any(|e| e.len() > u16::MAX as usize) {
            return Err(StoreError::TooLarge);
        }
        let first = self.size;
        if entries.is_empty() {
            return Ok((first, first));
        }
        self.store.lock_writer()?;
        // fence: the store must still be at the commit this instance last saw, checked BEFORE any write so a stale
        // writer cannot clobber same-path tiles of a newer committed state
        if self.store.get(COMMIT_PATH)? != self.commit {
            return Err(StoreError::Conflict(if self.dirty {
                "indeterminate commit after a failed append; reopen the log".into()
            } else {
                "commit record changed underneath this writer (stale or second writer); reopen"
                    .into()
            }));
        }
        self.dirty = false;
        if self.commit.is_none() {
            // new or pre-commit store: pin the current state BEFORE any entry lands, so a failed or crashed
            // append can never be mistaken for content by size inference
            let bytes = commit_bytes(self.size, &self.root()?);
            self.store.put(COMMIT_PATH, &bytes)?;
            self.commit = Some(bytes);
        }
        // 1. entry bundles (source of truth), built on local state
        let mut bundle = self.entry_edge.clone();
        let mut size = first;
        for e in entries {
            bundle.extend_from_slice(&(e.len() as u16).to_be_bytes());
            bundle.extend_from_slice(e);
            size += 1;
            if size.is_multiple_of(TILE_WIDTH) {
                let n = size / TILE_WIDTH - 1;
                self.store.put(&entry_tile_path(n, TILE_WIDTH), &bundle)?;
                bundle.clear();
            }
        }
        if !size.is_multiple_of(TILE_WIDTH) {
            self.store.put(
                &entry_tile_path(size / TILE_WIDTH, size % TILE_WIDTH),
                &bundle,
            )?;
        }
        // 2. hash tiles (derived)
        let mut edge = self.edge.clone();
        let mut counts = [0u64; MAX_LEVELS];
        for (l, c) in counts.iter_mut().enumerate() {
            *c = first >> (HEIGHT as usize * l);
        }
        for e in entries {
            Self::push_hash(&self.store, &mut edge, &mut counts, leaf_hash(e), true)?;
        }
        Self::put_edges(&self.store, &edge, size)?;
        // 3. commit record last; swap in the new state, restore the old one if the commit fails
        let old_size = std::mem::replace(&mut self.size, size);
        let old_edge = std::mem::replace(&mut self.edge, edge);
        let old_bundle = std::mem::replace(&mut self.entry_edge, bundle);
        self.verified.lock().unwrap().clear();
        match self.write_commit() {
            Ok(()) => Ok((first, size)),
            Err(e) => {
                self.size = old_size;
                self.edge = old_edge;
                self.entry_edge = old_bundle;
                self.verified.lock().unwrap().clear();
                self.dirty = true;
                Err(e)
            }
        }
    }

    fn write_commit(&mut self) -> Res<()> {
        let bytes = commit_bytes(self.size, &self.root()?);
        if self.store.get(COMMIT_PATH)? != self.commit {
            return Err(StoreError::Conflict(
                "commit record changed underneath this writer (second writer?)".into(),
            ));
        }
        self.store.put(COMMIT_PATH, &bytes)?;
        self.commit = Some(bytes);
        Ok(())
    }

    /// Bytes of entry bundle `n` (borrowed from memory for the current partial bundle) and its expected count.
    fn bundle(&self, n: u64) -> Res<(Cow<'_, [u8]>, u64)> {
        let full = self.size / TILE_WIDTH;
        if n < full {
            let b = self
                .store
                .get(&entry_tile_path(n, TILE_WIDTH))?
                .ok_or_else(|| StoreError::Corrupt("missing entry bundle".into()))?;
            Ok((Cow::Owned(b), TILE_WIDTH))
        } else {
            Ok((Cow::Borrowed(&self.entry_edge[..]), self.size % TILE_WIDTH))
        }
    }

    /// Entry at `index`, checked against its leaf hash in the tree.
    pub fn entry(&self, index: u64) -> Res<Vec<u8>> {
        if index >= self.size {
            return Err(StoreError::Range);
        }
        let n = index / TILE_WIDTH;
        let (b, _) = self.bundle(n)?;
        let e = bundle_nth(&b, (index % TILE_WIDTH) as usize)?;
        // the in-memory partial bundle was checked against the edge hashes when loaded; stored bundles are not
        if n < self.size / TILE_WIDTH && leaf_hash(e) != self.leaf_hash_at(index)? {
            return Err(StoreError::Corrupt(
                "entry does not match its leaf hash".into(),
            ));
        }
        Ok(e.to_vec())
    }

    /// Visit entries `from..size` in order, reading each bundle once and verifying every entry against the hash
    /// tiles (which are in turn verified against their parents). A truncated or altered bundle is an error.
    pub fn scan<E: From<StoreError>>(
        &self,
        from: u64,
        mut f: impl FnMut(u64, &[u8]) -> Result<(), E>,
    ) -> Result<(), E> {
        let mut i = from;
        while i < self.size {
            let n = i / TILE_WIDTH;
            let (b, want) = self.bundle(n)?;
            let es = Self::parse_bundle(&b)?;
            if es.len() as u64 != want {
                return Err(StoreError::Corrupt("entry bundle width".into()).into());
            }
            let stored = n < self.size / TILE_WIDTH;
            let hashes: Cow<'_, [Hash]> = if stored {
                Cow::Owned(self.load_full_tile(0, n)?)
            } else {
                Cow::Borrowed(&self.edge[0][..])
            };
            for (off, e) in es.iter().enumerate().skip((i % TILE_WIDTH) as usize) {
                if stored && hashes.get(off) != Some(&leaf_hash(e)) {
                    return Err(
                        StoreError::Corrupt("entry does not match its leaf hash".into()).into(),
                    );
                }
                f(n * TILE_WIDTH + off as u64, e)?;
            }
            i = (n + 1) * TILE_WIDTH;
        }
        Ok(())
    }

    /// Deep audit: every entry bundle is complete and matches the hash tiles, which match each other.
    pub fn verify_integrity(&self) -> Res<()> {
        self.scan(0, |_, _| Ok::<(), StoreError>(()))
    }

    /// Load a full hash tile and check it against its parent hash (once per instance).
    fn load_full_tile(&self, l: u32, n: u64) -> Res<Vec<Hash>> {
        let b = self
            .store
            .get(&hash_tile_path(l, n, TILE_WIDTH))?
            .ok_or_else(|| StoreError::Corrupt("missing hash tile".into()))?;
        let h = split_hashes(&b)?;
        if h.len() as u64 != TILE_WIDTH {
            return Err(StoreError::Corrupt("hash tile width".into()));
        }
        let digest = sha256(&b);
        if self.verified.lock().unwrap().get(&(l, n)) != Some(&digest) {
            if l as usize + 1 >= MAX_LEVELS {
                return Err(StoreError::Corrupt("hash tile level".into()));
            }
            let parent = self.stored_range(l + 1, n, 1)?[0];
            if mth_slice(&h) != parent {
                return Err(StoreError::Corrupt(
                    "hash tile does not match its parent hash".into(),
                ));
            }
            self.verified.lock().unwrap().insert((l, n), digest);
        }
        Ok(h)
    }

    /// Hash at stored level `l`, index `idx`.
    fn stored(&self, l: u32, idx: u64) -> Res<Hash> {
        Ok(self.stored_range(l, idx, 1)?[0])
    }

    /// `len` consecutive stored hashes at level `l` starting at `idx` (must lie within one tile).
    fn stored_range(&self, l: u32, idx: u64, len: u64) -> Res<Vec<Hash>> {
        if l as usize >= MAX_LEVELS {
            return Err(StoreError::Range);
        }
        let count = self.size >> (HEIGHT * l);
        if idx.checked_add(len).is_none_or(|e| e > count) {
            return Err(StoreError::Range);
        }
        let (n, off) = (idx / TILE_WIDTH, (idx % TILE_WIDTH) as usize);
        if off as u64 + len > TILE_WIDTH {
            return Err(StoreError::Range);
        }
        let full = count / TILE_WIDTH;
        let hashes = if n < full {
            self.load_full_tile(l, n)?
        } else {
            self.edge[l as usize].clone()
        };
        Ok(hashes
            .get(off..off + len as usize)
            .ok_or_else(|| StoreError::Corrupt("hash tile short".into()))?
            .to_vec())
    }

    /// Hash of the perfect subtree at tree level `k`, index `i` (covers leaves [i<<k, (i+1)<<k)).
    fn subtree(&self, k: u32, i: u64) -> Res<Hash> {
        let l = k / HEIGHT;
        let r = k % HEIGHT;
        if r == 0 {
            return self.stored(l, i);
        }
        let hs = self.stored_range(l, i << r, 1u64 << r)?;
        Ok(mth_slice(&hs))
    }

    /// MTH over leaves [a, b), b <= size.
    fn range_hash(&self, a: u64, b: u64) -> Res<Hash> {
        let n = b - a;
        if n == 0 {
            return Ok(sha256(b""));
        }
        if n.is_power_of_two() && a.is_multiple_of(n) {
            return self.subtree(n.trailing_zeros(), a / n);
        }
        let k = largest_pow2_below(n);
        Ok(node_hash(
            &self.range_hash(a, a + k)?,
            &self.range_hash(a + k, b)?,
        ))
    }

    /// Merkle root of the first `size` leaves.
    pub fn root_at(&self, size: u64) -> Res<Hash> {
        if size > self.size {
            return Err(StoreError::Range);
        }
        self.range_hash(0, size)
    }

    pub fn root(&self) -> Res<Hash> {
        self.root_at(self.size)
    }

    pub fn leaf_hash_at(&self, index: u64) -> Res<Hash> {
        if index >= self.size {
            return Err(StoreError::Range);
        }
        self.subtree(0, index)
    }

    /// Inclusion proof of leaf `index` in the tree of the first `size` leaves (RFC 9162 §2.1.3.1).
    pub fn inclusion_proof(&self, index: u64, size: u64) -> Res<Vec<Hash>> {
        if size > self.size || index >= size {
            return Err(StoreError::Range);
        }
        let mut path = Vec::new();
        let (mut m, mut a, mut b) = (index, 0u64, size);
        // iterative form of PATH(m, D[a:b]); siblings are collected top-down and reversed
        while b - a > 1 {
            let k = largest_pow2_below(b - a);
            if m < k {
                path.push(self.range_hash(a + k, b)?);
                b = a + k;
            } else {
                path.push(self.range_hash(a, a + k)?);
                a += k;
                m -= k;
            }
        }
        path.reverse();
        Ok(path)
    }

    /// Consistency proof between the first `first` leaves and the first `second` leaves (RFC 9162 §2.1.4.1).
    pub fn consistency_proof(&self, first: u64, second: u64) -> Res<Vec<Hash>> {
        if first == 0 || first > second || second > self.size {
            return Err(StoreError::Range);
        }
        let mut path = Vec::new();
        let (mut m, mut a, mut b, mut complete) = (first, 0u64, second, true);
        loop {
            let n = b - a;
            if m == n {
                if !complete {
                    path.push(self.range_hash(a, b)?);
                }
                break;
            }
            let k = largest_pow2_below(n);
            if m <= k {
                path.push(self.range_hash(a + k, b)?);
                b = a + k;
            } else {
                path.push(self.range_hash(a, a + k)?);
                a += k;
                m -= k;
                complete = false;
            }
        }
        path.reverse();
        Ok(path)
    }
}
