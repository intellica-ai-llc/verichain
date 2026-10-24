//! Tile addressing.

/// Tile height: a tile holds 2^8 = 256 hashes (or entries).
pub const HEIGHT: u32 = 8;
pub const TILE_WIDTH: u64 = 1 << HEIGHT;

/// Encode a tile index as 3-digit groups, all but the last prefixed with `x`.
pub fn index_path(mut n: u64) -> String {
    let mut s = format!("{:03}", n % 1000);
    while n >= 1000 {
        n /= 1000;
        s = format!("x{:03}/{}", n % 1000, s);
    }
    s
}

/// Path of a hash tile at tile level `level`, index `n`; `width` < 256 means a partial tile.
pub fn hash_tile_path(level: u32, n: u64, width: u64) -> String {
    let base = format!("tile/{}/{}", level, index_path(n));
    if width < TILE_WIDTH {
        format!("{base}.p/{width}")
    } else {
        base
    }
}

/// Path of an entry bundle.
pub fn entry_tile_path(n: u64, width: u64) -> String {
    let base = format!("tile/entries/{}", index_path(n));
    if width < TILE_WIDTH {
        format!("{base}.p/{width}")
    } else {
        base
    }
}
