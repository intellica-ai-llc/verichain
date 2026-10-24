//! RFC 8785 (JCS) canonicalization with I-JSON strictness:
//! - object keys sorted by UTF-16 code units,
//! - numbers are IEEE-754 doubles serialized per ECMAScript (so integers above 2^53 are reduced),
//! - duplicate object keys, lone surrogates, non-finite numbers and over-deep nesting are rejected.
use serde_json::Value;

const MAX_DEPTH: usize = 128;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum CanonError {
    #[error("invalid JSON: {0}")]
    Syntax(&'static str),
    #[error("duplicate object key")]
    DuplicateKey,
    #[error("number not representable as finite IEEE-754 double")]
    Number,
    #[error("nesting too deep")]
    Depth,
    #[error("lone surrogate in string")]
    Surrogate,
}

enum J {
    Null,
    Bool(bool),
    Num(f64),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}

/// Canonical bytes of an already-parsed value. (Duplicates cannot occur in a `Value`.)
pub fn canonicalize(v: &Value) -> Result<Vec<u8>, CanonError> {
    let j = from_value(v, 0)?;
    let mut out = Vec::new();
    write(&j, &mut out)?;
    Ok(out)
}

/// Parse strict JSON text (rejecting duplicates etc.) and canonicalize it.
pub fn canonicalize_str(s: &str) -> Result<Vec<u8>, CanonError> {
    let mut p = Parser {
        b: s.as_bytes(),
        i: 0,
    };
    p.ws();
    let j = p.value(0)?;
    p.ws();
    if p.i != p.b.len() {
        return Err(CanonError::Syntax("trailing data"));
    }
    let mut out = Vec::new();
    write(&j, &mut out)?;
    Ok(out)
}

fn from_value(v: &Value, depth: usize) -> Result<J, CanonError> {
    if depth > MAX_DEPTH {
        return Err(CanonError::Depth);
    }
    Ok(match v {
        Value::Null => J::Null,
        Value::Bool(b) => J::Bool(*b),
        Value::Number(n) => {
            let f = n.as_f64().ok_or(CanonError::Number)?;
            if !f.is_finite() {
                return Err(CanonError::Number);
            }
            J::Num(f)
        }
        Value::String(s) => J::Str(s.clone()),
        Value::Array(a) => J::Arr(
            a.iter()
                .map(|x| from_value(x, depth + 1))
                .collect::<Result<_, _>>()?,
        ),
        Value::Object(m) => J::Obj(
            m.iter()
                .map(|(k, x)| Ok((k.clone(), from_value(x, depth + 1)?)))
                .collect::<Result<_, CanonError>>()?,
        ),
    })
}

fn write(j: &J, out: &mut Vec<u8>) -> Result<(), CanonError> {
    match j {
        J::Null => out.extend_from_slice(b"null"),
        J::Bool(true) => out.extend_from_slice(b"true"),
        J::Bool(false) => out.extend_from_slice(b"false"),
        J::Num(f) => out.extend_from_slice(ryu_js::Buffer::new().format_finite(*f).as_bytes()),
        J::Str(s) => write_str(s, out),
        J::Arr(a) => {
            out.push(b'[');
            for (i, x) in a.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                write(x, out)?;
            }
            out.push(b']');
        }
        J::Obj(m) => {
            let mut idx: Vec<usize> = (0..m.len()).collect();
            idx.sort_by(|&a, &b| m[a].0.encode_utf16().cmp(m[b].0.encode_utf16()));
            for w in idx.windows(2) {
                if m[w[0]].0 == m[w[1]].0 {
                    return Err(CanonError::DuplicateKey);
                }
            }
            out.push(b'{');
            for (n, &i) in idx.iter().enumerate() {
                if n > 0 {
                    out.push(b',');
                }
                write_str(&m[i].0, out);
                out.push(b':');
                write(&m[i].1, out)?;
            }
            out.push(b'}');
        }
    }
    Ok(())
}

fn write_str(s: &str, out: &mut Vec<u8>) {
    out.push(b'"');
    for c in s.chars() {
        match c {
            '"' => out.extend_from_slice(b"\\\""),
            '\\' => out.extend_from_slice(b"\\\\"),
            '\u{8}' => out.extend_from_slice(b"\\b"),
            '\u{c}' => out.extend_from_slice(b"\\f"),
            '\n' => out.extend_from_slice(b"\\n"),
            '\r' => out.extend_from_slice(b"\\r"),
            '\t' => out.extend_from_slice(b"\\t"),
            c if (c as u32) < 0x20 => {
                out.extend_from_slice(format!("\\u{:04x}", c as u32).as_bytes())
            }
            c => {
                let mut buf = [0u8; 4];
                out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            }
        }
    }
    out.push(b'"');
}

struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    fn ws(&mut self) {
        while matches!(self.b.get(self.i), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.i += 1;
        }
    }
    fn peek(&self) -> Option<u8> {
        self.b.get(self.i).copied()
    }
    fn lit(&mut self, s: &[u8], v: J) -> Result<J, CanonError> {
        if self.b[self.i..].starts_with(s) {
            self.i += s.len();
            Ok(v)
        } else {
            Err(CanonError::Syntax("bad literal"))
        }
    }
    fn value(&mut self, depth: usize) -> Result<J, CanonError> {
        if depth > MAX_DEPTH {
            return Err(CanonError::Depth);
        }
        match self.peek().ok_or(CanonError::Syntax("unexpected end"))? {
            b'n' => self.lit(b"null", J::Null),
            b't' => self.lit(b"true", J::Bool(true)),
            b'f' => self.lit(b"false", J::Bool(false)),
            b'"' => Ok(J::Str(self.string()?)),
            b'[' => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.peek() == Some(b']') {
                    self.i += 1;
                    return Ok(J::Arr(v));
                }
                loop {
                    self.ws();
                    v.push(self.value(depth + 1)?);
                    self.ws();
                    match self.peek() {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(J::Arr(v));
                        }
                        _ => return Err(CanonError::Syntax("expected , or ]")),
                    }
                }
            }
            b'{' => {
                self.i += 1;
                let mut m: Vec<(String, J)> = Vec::new();
                self.ws();
                if self.peek() == Some(b'}') {
                    self.i += 1;
                    return Ok(J::Obj(m));
                }
                loop {
                    self.ws();
                    if self.peek() != Some(b'"') {
                        return Err(CanonError::Syntax("expected key"));
                    }
                    let k = self.string()?;
                    if m.iter().any(|(e, _)| *e == k) {
                        return Err(CanonError::DuplicateKey);
                    }
                    self.ws();
                    if self.peek() != Some(b':') {
                        return Err(CanonError::Syntax("expected :"));
                    }
                    self.i += 1;
                    self.ws();
                    let v = self.value(depth + 1)?;
                    m.push((k, v));
                    self.ws();
                    match self.peek() {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(J::Obj(m));
                        }
                        _ => return Err(CanonError::Syntax("expected , or }")),
                    }
                }
            }
            b'-' | b'0'..=b'9' => self.number(),
            _ => Err(CanonError::Syntax("unexpected character")),
        }
    }
    fn number(&mut self) -> Result<J, CanonError> {
        let s = self.i;
        if self.peek() == Some(b'-') {
            self.i += 1;
        }
        match self.peek() {
            Some(b'0') => self.i += 1,
            Some(b'1'..=b'9') => {
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.i += 1;
                }
            }
            _ => return Err(CanonError::Syntax("bad number")),
        }
        if self.peek() == Some(b'.') {
            self.i += 1;
            let d = self.i;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.i += 1;
            }
            if self.i == d {
                return Err(CanonError::Syntax("bad fraction"));
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.i += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.i += 1;
            }
            let d = self.i;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.i += 1;
            }
            if self.i == d {
                return Err(CanonError::Syntax("bad exponent"));
            }
        }
        let t = std::str::from_utf8(&self.b[s..self.i]).map_err(|_| CanonError::Number)?;
        let f: f64 = t.parse().map_err(|_| CanonError::Number)?;
        if !f.is_finite() {
            return Err(CanonError::Number);
        }
        Ok(J::Num(f))
    }
    fn hex4(&mut self) -> Result<u32, CanonError> {
        let h = self
            .b
            .get(self.i..self.i + 4)
            .ok_or(CanonError::Syntax("short \\u escape"))?;
        let mut v = 0u32;
        for &c in h {
            v = v * 16
                + (c as char)
                    .to_digit(16)
                    .ok_or(CanonError::Syntax("bad \\u escape"))?;
        }
        self.i += 4;
        Ok(v)
    }
    fn string(&mut self) -> Result<String, CanonError> {
        self.i += 1; // opening quote
        let mut out = String::new();
        loop {
            let c = *self
                .b
                .get(self.i)
                .ok_or(CanonError::Syntax("unterminated string"))?;
            match c {
                b'"' => {
                    self.i += 1;
                    return Ok(out);
                }
                b'\\' => {
                    self.i += 1;
                    let e = *self.b.get(self.i).ok_or(CanonError::Syntax("bad escape"))?;
                    self.i += 1;
                    match e {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{8}'),
                        b'f' => out.push('\u{c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let u = self.hex4()?;
                            let ch = if (0xD800..0xDC00).contains(&u) {
                                if self.b.get(self.i..self.i + 2) != Some(b"\\u") {
                                    return Err(CanonError::Surrogate);
                                }
                                self.i += 2;
                                let lo = self.hex4()?;
                                if !(0xDC00..0xE000).contains(&lo) {
                                    return Err(CanonError::Surrogate);
                                }
                                char::from_u32(0x10000 + ((u - 0xD800) << 10) + (lo - 0xDC00))
                                    .ok_or(CanonError::Surrogate)?
                            } else if (0xDC00..0xE000).contains(&u) {
                                return Err(CanonError::Surrogate);
                            } else {
                                char::from_u32(u).ok_or(CanonError::Surrogate)?
                            };
                            out.push(ch);
                        }
                        _ => return Err(CanonError::Syntax("bad escape")),
                    }
                }
                0x00..=0x1f => return Err(CanonError::Syntax("control character in string")),
                _ => {
                    // copy one UTF-8 scalar (input is &str so it is valid UTF-8)
                    let rest = std::str::from_utf8(&self.b[self.i..])
                        .map_err(|_| CanonError::Syntax("utf8"))?;
                    let ch = rest.chars().next().ok_or(CanonError::Syntax("utf8"))?;
                    out.push(ch);
                    self.i += ch.len_utf8();
                }
            }
        }
    }
}
