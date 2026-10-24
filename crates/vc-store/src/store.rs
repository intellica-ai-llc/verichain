//! Blob storage abstraction with atomic writes.
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::sync::Mutex;

pub trait TileStore {
    fn get(&self, path: &str) -> io::Result<Option<Vec<u8>>>;
    /// Durably and atomically write `data` at `path` (replacing any previous content).
    fn put(&self, path: &str, data: &[u8]) -> io::Result<()>;
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
}

/// Filesystem store: write to a temp file in the same directory, fsync, rename, fsync the directory.
pub struct FsStore {
    root: PathBuf,
}

impl FsStore {
    pub fn new(root: impl Into<PathBuf>) -> io::Result<Self> {
        let root = root.into();
        std::fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    fn safe(&self, path: &str) -> io::Result<PathBuf> {
        if path
            .split('/')
            .any(|c| c.is_empty() || c == ".." || c == ".")
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

    fn put(&self, path: &str, data: &[u8]) -> io::Result<()> {
        use std::io::Write;
        let dest = self.safe(path)?;
        let dir = dest.parent().ok_or_else(|| io::Error::other("no parent"))?;
        std::fs::create_dir_all(dir)?;
        let tmp = dir.join(format!(
            ".tmp-{}-{}",
            std::process::id(),
            dest.file_name().and_then(|n| n.to_str()).unwrap_or("x")
        ));
        {
            let mut f = std::fs::File::create(&tmp)?;
            f.write_all(data)?;
            f.sync_all()?;
        }
        std::fs::rename(&tmp, &dest)?;
        if let Ok(d) = std::fs::File::open(dir) {
            let _ = d.sync_all();
        }
        Ok(())
    }
}
