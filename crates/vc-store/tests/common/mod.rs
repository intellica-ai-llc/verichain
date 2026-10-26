//! Shared helpers for the W1 verifier tests (failure-injecting store, independent roots).
#![allow(dead_code)]
use std::collections::BTreeMap;
use std::io;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use vc_core::hash::{leaf_hash, Hash};
use vc_store::TileStore;

/// A shareable in-memory store whose `put` can be made to fail after N successful puts.
#[derive(Clone)]
pub struct Shared {
    pub m: Arc<Mutex<BTreeMap<String, Vec<u8>>>>,
    /// remaining successful puts; negative = unlimited
    pub budget: Arc<AtomicI64>,
    pub puts: Arc<AtomicU64>,
    pub fail_get: Arc<AtomicBool>,
}

impl Shared {
    pub fn new() -> Self {
        Shared {
            m: Arc::new(Mutex::new(BTreeMap::new())),
            budget: Arc::new(AtomicI64::new(-1)),
            puts: Arc::new(AtomicU64::new(0)),
            fail_get: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn unlimited(&self) {
        self.budget.store(-1, Ordering::SeqCst);
    }
    pub fn fail_after(&self, n: i64) {
        self.budget.store(n, Ordering::SeqCst);
    }
    pub fn snapshot(&self) -> BTreeMap<String, Vec<u8>> {
        self.m.lock().unwrap().clone()
    }
    pub fn restore(&self, s: &BTreeMap<String, Vec<u8>>) {
        *self.m.lock().unwrap() = s.clone();
    }
    pub fn raw_put(&self, path: &str, data: &[u8]) {
        self.m.lock().unwrap().insert(path.to_string(), data.to_vec());
    }
    pub fn raw_get(&self, path: &str) -> Option<Vec<u8>> {
        self.m.lock().unwrap().get(path).cloned()
    }
    pub fn raw_remove(&self, path: &str) {
        self.m.lock().unwrap().remove(path);
    }
    pub fn paths(&self) -> Vec<String> {
        self.m.lock().unwrap().keys().cloned().collect()
    }
}

impl TileStore for Shared {
    fn get(&self, path: &str) -> io::Result<Option<Vec<u8>>> {
        if self.fail_get.load(Ordering::SeqCst) {
            return Err(io::Error::other("injected get failure"));
        }
        Ok(self.m.lock().unwrap().get(path).cloned())
    }
    fn put(&self, path: &str, data: &[u8]) -> io::Result<()> {
        let b = self.budget.load(Ordering::SeqCst);
        if b == 0 {
            return Err(io::Error::other("injected put failure"));
        }
        if b > 0 {
            self.budget.store(b - 1, Ordering::SeqCst);
        }
        self.puts.fetch_add(1, Ordering::SeqCst);
        self.m.lock().unwrap().insert(path.to_string(), data.to_vec());
        Ok(())
    }
}

pub fn entry(i: usize) -> Vec<u8> {
    format!("e{i}").into_bytes()
}

pub fn leaves(n: usize) -> Vec<Hash> {
    (0..n).map(|i| leaf_hash(&entry(i))).collect()
}

pub fn refs(es: &[Vec<u8>]) -> Vec<&[u8]> {
    es.iter().map(|e| e.as_slice()).collect()
}
