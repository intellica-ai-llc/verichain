//! Append-only log over tiles, with O(log n) proofs read from tiles.
use crate::store::TileStore;
use crate::tile::*;
use vc_core::hash::{leaf_hash, node_hash, sha256, Hash};

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
}

type Res<T> = Result<T, StoreError>;

const MAX_LEVELS: usize = 8; // 256^8 leaves is far beyond u64

pub struct TileLog<S: TileStore> {
    store: S,
    size: u64,
    /// For each stored level: hashes of the current (possibly partial) tile.
    edge: Vec<Vec<Hash>>,
    /// Entry bundle bytes and entry count of the current (possibly partial) bundle.
    entry_edge: Vec<u8>,
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
    /// Open (or create) a log; recovers hash tiles from entry bundles if they are behind.
    pub fn open(store: S) -> Res<Self> {
        let mut log = TileLog {
            store,
            size: 0,
            edge: vec![Vec::new(); MAX_LEVELS],
            entry_edge: Vec::new(),
        };
        log.size = log.scan_entries()?;
        if !log.load_edges()? {
            log.recover()?;
        }
        Ok(log)
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    /// Count entries by probing bundles (full tiles, then the widest partial).
    fn scan_entries(&mut self) -> Res<u64> {
        let mut n = 0u64;
        while self.store.get(&entry_tile_path(n, TILE_WIDTH))?.is_some() {
            n += 1;
        }
        let mut count = n * TILE_WIDTH;
        for w in (1..TILE_WIDTH).rev() {
            if let Some(b) = self.store.get(&entry_tile_path(n, w))? {
                let got = Self::parse_bundle(&b)?;
                if got.len() as u64 != w {
                    return Err(StoreError::Corrupt("partial entry bundle width".into()));
                }
                self.entry_edge = b;
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

    /// Load the current partial tile of every stored level; false if any expected tile is missing.
    fn load_edges(&mut self) -> Res<bool> {
        for l in 0..MAX_LEVELS {
            let count = self.size >> (HEIGHT as usize * l);
            let (full, w) = (count / TILE_WIDTH, count % TILE_WIDTH);
            self.edge[l].clear();
            if count == 0 {
                continue;
            }
            // all full tiles must exist
            if full > 0
                && self
                    .store
                    .get(&hash_tile_path(l as u32, full - 1, TILE_WIDTH))?
                    .is_none()
            {
                return Ok(false);
            }
            if w > 0 {
                match self.store.get(&hash_tile_path(l as u32, full, w))? {
                    Some(b) => {
                        let h = split_hashes(&b)?;
                        if h.len() as u64 != w {
                            return Ok(false);
                        }
                        self.edge[l] = h;
                    }
                    None => return Ok(false),
                }
            }
        }
        Ok(true)
    }

    /// Rebuild every hash tile from the entry bundles (entries are the source of truth).
    fn recover(&mut self) -> Res<()> {
        let size = self.size;
        let mut edge: Vec<Vec<Hash>> = vec![Vec::new(); MAX_LEVELS];
        let mut counts = [0u64; MAX_LEVELS];
        for i in 0..size {
            let e = self.entry(i)?;
            Self::push_hash(&self.store, &mut edge, &mut counts, leaf_hash(&e), true)?;
        }
        // write partial edge tiles
        for (l, e) in edge.iter().enumerate() {
            if !e.is_empty() {
                let n = (self.size >> (HEIGHT as usize * l)) / TILE_WIDTH;
                self.store.put(
                    &hash_tile_path(l as u32, n, e.len() as u64),
                    &join_hashes(e),
                )?;
            }
        }
        self.edge = edge;
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

    /// Append a batch durably (group commit). Returns (first index, new size).
    pub fn append_batch(&mut self, entries: &[&[u8]]) -> Res<(u64, u64)> {
        if entries.iter().any(|e| e.len() > u16::MAX as usize) {
            return Err(StoreError::TooLarge);
        }
        let first = self.size;
        if entries.is_empty() {
            return Ok((first, first));
        }
        // 1. entry bundles first (source of truth)
        for e in entries {
            self.entry_edge
                .extend_from_slice(&(e.len() as u16).to_be_bytes());
            self.entry_edge.extend_from_slice(e);
            self.size += 1;
            let w = self.size % TILE_WIDTH;
            if w == 0 {
                let n = self.size / TILE_WIDTH - 1;
                self.store
                    .put(&entry_tile_path(n, TILE_WIDTH), &self.entry_edge)?;
                self.entry_edge.clear();
            }
        }
        let w = self.size % TILE_WIDTH;
        if w != 0 {
            let n = self.size / TILE_WIDTH;
            self.store.put(&entry_tile_path(n, w), &self.entry_edge)?;
        }
        // 2. hash tiles (derived)
        let mut edge = std::mem::take(&mut self.edge);
        let mut counts = [0u64; MAX_LEVELS];
        for (l, c) in counts.iter_mut().enumerate() {
            *c = first >> (HEIGHT as usize * l);
        }
        for e in entries {
            Self::push_hash(&self.store, &mut edge, &mut counts, leaf_hash(e), true)?;
        }
        for (l, e) in edge.iter().enumerate() {
            if !e.is_empty() {
                let n = (self.size >> (HEIGHT as usize * l)) / TILE_WIDTH;
                self.store.put(
                    &hash_tile_path(l as u32, n, e.len() as u64),
                    &join_hashes(e),
                )?;
            }
        }
        self.edge = edge;
        Ok((first, self.size))
    }

    /// Entry at `index`.
    pub fn entry(&self, index: u64) -> Res<Vec<u8>> {
        if index >= self.size {
            return Err(StoreError::Range);
        }
        let (n, off) = (index / TILE_WIDTH, (index % TILE_WIDTH) as usize);
        let full = self.size / TILE_WIDTH;
        let b = if n < full {
            self.store.get(&entry_tile_path(n, TILE_WIDTH))?
        } else {
            Some(self.entry_edge.clone())
        }
        .ok_or_else(|| StoreError::Corrupt("missing entry bundle".into()))?;
        Self::parse_bundle(&b)?
            .get(off)
            .map(|e| e.to_vec())
            .ok_or_else(|| StoreError::Corrupt("entry offset".into()))
    }

    /// Hash at stored level `l`, index `idx`.
    fn stored(&self, l: u32, idx: u64) -> Res<Hash> {
        Ok(self.stored_range(l, idx, 1)?[0])
    }

    /// `len` consecutive stored hashes at level `l` starting at `idx` (must lie within one tile).
    fn stored_range(&self, l: u32, idx: u64, len: u64) -> Res<Vec<Hash>> {
        let count = self.size >> (HEIGHT * l);
        if idx + len > count {
            return Err(StoreError::Range);
        }
        let (n, off) = (idx / TILE_WIDTH, (idx % TILE_WIDTH) as usize);
        if off as u64 + len > TILE_WIDTH {
            return Err(StoreError::Range);
        }
        let full = count / TILE_WIDTH;
        let hashes = if n < full {
            split_hashes(
                &self
                    .store
                    .get(&hash_tile_path(l, n, TILE_WIDTH))?
                    .ok_or_else(|| StoreError::Corrupt("missing hash tile".into()))?,
            )?
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
