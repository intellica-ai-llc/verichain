//! Strict primitive decoders: lowercase hex with length bounds, RFC 3339 date-time.

/// Decode strict lowercase hex (no `0x`, no uppercase, no whitespace, even length) of at most `max_bytes` bytes.
pub fn hex_strict(s: &str, max_bytes: usize) -> Result<Vec<u8>, String> {
    if s.len() > max_bytes.saturating_mul(2) {
        return Err(format!("hex string longer than {max_bytes} bytes"));
    }
    if s.len() % 2 != 0 {
        return Err("hex string has odd length".into());
    }
    if !s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')) {
        return Err("hex must be lowercase [0-9a-f] only".into());
    }
    hex::decode(s).map_err(|_| "invalid hex".to_string())
}

/// Strict lowercase hex of exactly 32 bytes.
pub fn hex32(s: &str) -> Result<[u8; 32], String> {
    let v = hex_strict(s, 32)?;
    v.try_into()
        .map_err(|_| "expected exactly 32 bytes (64 hex characters)".to_string())
}

pub fn is_hex64(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

fn digits(b: &[u8]) -> Option<u32> {
    if b.is_empty() || !b.iter().all(u8::is_ascii_digit) {
        return None;
    }
    b.iter().try_fold(0u32, |a, d| a.checked_mul(10)?.checked_add((d - b'0') as u32))
}

/// RFC 3339 `date-time` with uppercase `T` and `Z`, calendar-valid, seconds 00..=60, optional fraction.
pub fn is_rfc3339(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() < 20 || b.len() > 64 {
        return false;
    }
    let (Some(y), Some(mo), Some(d), Some(h), Some(mi), Some(se)) = (
        digits(&b[0..4]),
        digits(&b[5..7]),
        digits(&b[8..10]),
        digits(&b[11..13]),
        digits(&b[14..16]),
        digits(&b[17..19]),
    ) else {
        return false;
    };
    if b[4] != b'-' || b[7] != b'-' || b[10] != b'T' || b[13] != b':' || b[16] != b':' {
        return false;
    }
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let dim = match mo {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    if d < 1 || d > dim || h > 23 || mi > 59 || se > 60 {
        return false;
    }
    let mut i = 19;
    if b[i] == b'.' {
        let st = i + 1;
        i = st;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == st {
            return false;
        }
    }
    match &b[i..] {
        b"Z" => true,
        [sign, rest @ ..] if (*sign == b'+' || *sign == b'-') && rest.len() == 5 => {
            rest[2] == b':'
                && matches!((digits(&rest[0..2]), digits(&rest[3..5])), (Some(h), Some(m)) if h <= 23 && m <= 59)
        }
        _ => false,
    }
}
