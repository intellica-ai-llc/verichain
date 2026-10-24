//! vc-store: tile-based append-only storage for the VeriChain log.
//!
//! Layout (tlog-tiles style, height 8): hash tiles at `tile/<L>/<N>` hold up to 256 consecutive hashes of tree
//! level 8*L; partial tiles are `tile/<L>/<N>.p/<W>`; entry bundles at `tile/entries/<N>` hold up to 256
//! entries, each prefixed by a big-endian u16 length. `<N>` is split into 3-digit groups, all but the last
//! prefixed with `x` (e.g. 1234067 -> `x001/x234/067`). Entries are the source of truth; hash tiles can be
//! recomputed from them (crash recovery). NOTE: layout is implemented from the tlog-tiles design as understood
//! from the Go reference (`sumdb/tlog`); conformance with the C2SP text is still to be verified (docs/SOURCES.md).
pub mod store;
pub mod tile;
pub mod tilelog;

pub use store::{FsStore, MemStore, TileStore};
pub use tilelog::{StoreError, TileLog};
