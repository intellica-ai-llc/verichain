//! Blob storage abstraction with atomic writes.
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

pub trait TileStore {
    fn get(&self, path: &str) -> io::Result<Option<Vec<u8>>>;
    /// Durably and atomically write `data` at `path` (replacing any previous content).
    fn put(&self, path: &str, data: &[u8]) -> io::Result<()>;
    /// Cheap existence probe (no content transfer where the backend allows it).
    fn exists(&self, path: &str) -> io::Result<bool> {
        Ok(self.get(path)?.is_some())
    }
    /// Acquire exclusive writer ownership of the store (idempotent). Backends that cannot fence writers keep the
    /// default; `TileLog` additionally does a compare-and-swap on its commit record.
    fn lock_writer(&self) -> io::Result<()> {
        Ok(())
    }
}

/// In-memory store for tests.
#[derive(Default)]
pub struct MemStore {
    m: Mutex<HashMap<String, Vec<u8>>>,
}

impl MemStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn remove_prefix(&self, prefix: &str) {
        self.m.lock().unwrap().retain(|k, _| !k.starts_with(prefix));
    }
    pub fn paths(&self) -> Vec<String> {
        let mut v: Vec<String> = self.m.lock().unwrap().keys().cloned().collect();
        v.sort();
        v
    }
}

impl TileStore for MemStore {
    fn get(&self, path: &str) -> io::Result<Option<Vec<u8>>> {
        Ok(self.m.lock().unwrap().get(path).cloned())
    }
    fn put(&self, path: &str, data: &[u8]) -> io::Result<()> {
        self.m
            .lock()
            .unwrap()
            .insert(path.to_string(), data.to_vec());
        Ok(())
    }
    fn exists(&self, path: &str) -> io::Result<bool> {
        Ok(self.m.lock().unwrap().contains_key(path))
    }
}

static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Filesystem store: write to a unique temp file in the same directory, fsync, rename, fsync the directory
/// (and any directories created on the way). `lock_writer` takes an exclusive advisory lock on `<root>/LOCK`.
pub struct FsStore {
    root: PathBuf,
    lock: Mutex<Option<std::fs::File>>,
}

fn sync_dir(d: &Path) -> io::Result<()> {
    std::fs::File::open(d)?.sync_all()
}

impl FsStore {
    pub fn new(root: impl Into<PathBuf>) -> io::Result<Self> {
        let root = root.into();
        std::fs::create_dir_all(&root)?;
        Ok(Self {
            root,
            lock: Mutex::new(None),
        })
    }

    fn safe(&self, path: &str) -> io::Result<PathBuf> {
        if path
            .split('/')
            .any(|c| c.is_empty() || c == ".." || c == "." || c.contains('\0'))
            || path.starts_with('/')
        {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "unsafe path"));
        }
        Ok(self.root.join(path))
    }
}

impl TileStore for FsStore {
    fn get(&self, path: &str) -> io::Result<Option<Vec<u8>>> {
        match std::fs::read(self.safe(path)?) {
            Ok(b) => Ok(Some(b)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn exists(&self, path: &str) -> io::Result<bool> {
        match std::fs::metadata(self.safe(path)?) {
            Ok(m) => Ok(m.is_file()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e),
        }
    }

    fn put(&self, path: &str, data: &[u8]) -> io::Result<()> {
        use std::io::Write;
        let dest = self.safe(path)?;
        let dir = dest.parent().ok_or_else(|| io::Error::other("no parent"))?;
        // directories that do not exist yet must be made durable in their parents
        let mut created = Vec::new();
        let mut d = dir;
        while d != self.root && !d.exists() {
            created.push(d.to_path_buf());
            match d.parent() {
                Some(p) => d = p,
                None => break,
            }
        }
        std::fs::create_dir_all(dir)?;
        let tmp = dir.join(format!(
            ".tmp-{}-{}-{}",
            std::process::id(),
            TMP_COUNTER.fetch_add(1, Ordering::SeqCst),
            dest.file_name().and_then(|n| n.to_str()).unwrap_or("x")
        ));
        let res = (|| {
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&tmp)?;
            f.write_all(data)?;
            f.sync_all()?;
            drop(f);
            std::fs::rename(&tmp, &dest)
        })();
        if let Err(e) = res {
            let _ = std::fs::remove_file(&tmp);
            return Err(e);
        }
        sync_dir(dir)?;
        for c in created.iter() {
            if let Some(p) = c.parent() {
                sync_dir(p)?;
            }
        }
        Ok(())
    }

    fn lock_writer(&self) -> io::Result<()> {
        let mut g = self.lock.lock().unwrap();
        if g.is_some() {
            return Ok(());
        }
        let f = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(self.root.join("LOCK"))?;
        match f.try_lock() {
            Ok(()) => {
                *g = Some(f);
                Ok(())
            }
            Err(std::fs::TryLockError::WouldBlock) => Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "another writer holds this log",
            )),
            Err(std::fs::TryLockError::Error(e)) => Err(e),
        }
    }
}
