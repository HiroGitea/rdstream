//! Forward-only input: a buffered byte reader with cheap skipping, and the wire
//! encodings R serializes with (XDR, native binary and ASCII; see `InInteger`,
//! `InReal`, `InString` and friends in R's `serialize.c`).

use std::io::{self, Read};

use crate::error::{Error, Result};
use crate::sexp::{NA_INTEGER, NA_REAL};

const BUF_SIZE: usize = 1 << 20;

pub struct ByteReader<R> {
    inner: R,
    buf: Box<[u8]>,
    pos: usize,
    end: usize,
    consumed: u64,
}

impl<R: Read> ByteReader<R> {
    pub fn new(inner: R) -> Self {
        ByteReader {
            inner,
            buf: vec![0; BUF_SIZE].into_boxed_slice(),
            pos: 0,
            end: 0,
            consumed: 0,
        }
    }

    /// Bytes handed out so far (read or skipped).
    pub fn consumed(&self) -> u64 {
        self.consumed
    }

    fn eof(&self) -> Error {
        Error::UnexpectedEof {
            offset: self.consumed,
        }
    }

    /// Refill so that at least `n` (<= BUF_SIZE) bytes are buffered; fewer only at EOF.
    fn fill_upto(&mut self, n: usize) -> Result<usize> {
        if self.end - self.pos >= n {
            return Ok(self.end - self.pos);
        }
        self.buf.copy_within(self.pos..self.end, 0);
        self.end -= self.pos;
        self.pos = 0;
        while self.end < n {
            match self.inner.read(&mut self.buf[self.end..]) {
                Ok(0) => break,
                Ok(k) => self.end += k,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e.into()),
            }
        }
        Ok(self.end)
    }

    fn fill(&mut self, n: usize) -> Result<()> {
        if self.fill_upto(n)? < n {
            Err(self.eof())
        } else {
            Ok(())
        }
    }

    /// Fill `out` completely; large reads bypass the internal buffer.
    pub fn read_exact(&mut self, out: &mut [u8]) -> Result<()> {
        let k = out.len().min(self.end - self.pos);
        out[..k].copy_from_slice(&self.buf[self.pos..self.pos + k]);
        self.pos += k;
        if k < out.len() {
            let rest = &mut out[k..];
            if rest.len() >= BUF_SIZE / 2 {
                self.inner.read_exact(rest).map_err(|e| match e.kind() {
                    io::ErrorKind::UnexpectedEof => Error::UnexpectedEof {
                        offset: self.consumed + k as u64,
                    },
                    _ => e.into(),
                })?;
            } else {
                self.fill(rest.len())?;
                rest.copy_from_slice(&self.buf[self.pos..self.pos + rest.len()]);
                self.pos += rest.len();
            }
        }
        self.consumed += out.len() as u64;
        Ok(())
    }

    pub fn bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut v = vec![0; n];
        self.read_exact(&mut v)?;
        Ok(v)
    }

    /// Discard `n` bytes.
    pub fn skip(&mut self, n: u64) -> Result<()> {
        let avail = (self.end - self.pos) as u64;
        if n <= avail {
            self.pos += n as usize;
            self.consumed += n;
            return Ok(());
        }
        let mut left = n - avail;
        self.pos = 0;
        self.end = 0;
        while left > 0 {
            let want = left.min(BUF_SIZE as u64) as usize;
            match self.inner.read(&mut self.buf[..want]) {
                Ok(0) => {
                    return Err(Error::UnexpectedEof {
                        offset: self.consumed + n - left,
                    });
                }
                Ok(r) => left -= r as u64,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e.into()),
            }
        }
        self.consumed += n;
        Ok(())
    }

    /// Next byte without consuming it; `None` at EOF.
    pub fn peek(&mut self) -> Result<Option<u8>> {
        Ok(if self.fill_upto(1)? == 0 {
            None
        } else {
            Some(self.buf[self.pos])
        })
    }

    pub fn byte(&mut self) -> Result<u8> {
        self.fill(1)?;
        let b = self.buf[self.pos];
        self.pos += 1;
        self.consumed += 1;
        Ok(b)
    }

    /// Skip ASCII whitespace, then replace `out` with the following run of
    /// non-whitespace bytes (`InWord`).
    pub fn word(&mut self, out: &mut Vec<u8>) -> Result<()> {
        out.clear();
        loop {
            if self.pos == self.end && self.fill_upto(1)? == 0 {
                return if out.is_empty() {
                    Err(self.eof())
                } else {
                    Ok(())
                };
            }
            let avail = &self.buf[self.pos..self.end];
            let start = if out.is_empty() {
                avail.iter().position(|b| !b.is_ascii_whitespace())
            } else {
                Some(0)
            };
            let Some(start) = start else {
                self.consumed += avail.len() as u64;
                self.pos = self.end;
                continue;
            };
            let len = avail[start..].iter().position(u8::is_ascii_whitespace);
            let stop = start + len.unwrap_or(avail.len() - start);
            out.extend_from_slice(&avail[start..stop]);
            self.consumed += stop as u64;
            self.pos += stop;
            if len.is_some() {
                return Ok(());
            }
        }
    }
}

/// One of R's serialization encodings. Every vector element and every string
/// goes through these methods, so the parser is independent of the encoding.
pub trait Wire {
    fn consumed(&self) -> u64;
    fn int(&mut self) -> Result<i32>;
    fn ints(&mut self, out: &mut [i32]) -> Result<()>;
    fn reals(&mut self, out: &mut [f64]) -> Result<()>;
    fn raw(&mut self, out: &mut [u8]) -> Result<()>;
    /// `len` bytes of string data (CHARSXP contents, encoding or builtin names).
    fn string(&mut self, len: usize) -> Result<Vec<u8>>;
    fn skip_ints(&mut self, n: u64) -> Result<()>;
    fn skip_reals(&mut self, n: u64) -> Result<()>;
    fn skip_raw(&mut self, n: u64) -> Result<()>;
    fn skip_string(&mut self, len: usize) -> Result<()>;
}

/// XDR (big-endian, R's default) or native binary. R writes native binary in
/// the writer's byte order without recording it; little-endian is assumed.
pub struct Binary<R> {
    r: ByteReader<R>,
    big_endian: bool,
}

impl<R: Read> Binary<R> {
    pub fn xdr(r: ByteReader<R>) -> Self {
        Binary {
            r,
            big_endian: true,
        }
    }

    pub fn native(r: ByteReader<R>) -> Self {
        Binary {
            r,
            big_endian: false,
        }
    }
}

impl<R: Read> Wire for Binary<R> {
    fn consumed(&self) -> u64 {
        self.r.consumed()
    }

    fn int(&mut self) -> Result<i32> {
        let mut b = [0u8; 4];
        if self.r.end - self.r.pos >= 4 {
            b.copy_from_slice(&self.r.buf[self.r.pos..self.r.pos + 4]);
            self.r.pos += 4;
            self.r.consumed += 4;
        } else {
            self.r.read_exact(&mut b)?;
        }
        Ok(if self.big_endian {
            i32::from_be_bytes(b)
        } else {
            i32::from_le_bytes(b)
        })
    }

    fn ints(&mut self, out: &mut [i32]) -> Result<()> {
        self.r.read_exact(bytemuck::cast_slice_mut(out))?;
        if self.big_endian {
            out.iter_mut().for_each(|x| *x = i32::from_be(*x));
        } else {
            out.iter_mut().for_each(|x| *x = i32::from_le(*x));
        }
        Ok(())
    }

    fn reals(&mut self, out: &mut [f64]) -> Result<()> {
        self.r.read_exact(bytemuck::cast_slice_mut(out))?;
        if self.big_endian {
            out.iter_mut()
                .for_each(|x| *x = f64::from_bits(u64::from_be(x.to_bits())));
        } else {
            out.iter_mut()
                .for_each(|x| *x = f64::from_bits(u64::from_le(x.to_bits())));
        }
        Ok(())
    }

    fn raw(&mut self, out: &mut [u8]) -> Result<()> {
        self.r.read_exact(out)
    }

    fn string(&mut self, len: usize) -> Result<Vec<u8>> {
        self.r.bytes(len)
    }

    fn skip_ints(&mut self, n: u64) -> Result<()> {
        self.r.skip(n * 4)
    }

    fn skip_reals(&mut self, n: u64) -> Result<()> {
        self.r.skip(n * 8)
    }

    fn skip_raw(&mut self, n: u64) -> Result<()> {
        self.r.skip(n)
    }

    fn skip_string(&mut self, len: usize) -> Result<()> {
        self.r.skip(len as u64)
    }
}

/// `saveRDS(ascii = TRUE)` / `save(ascii = TRUE)`: one whitespace-separated word
/// per number, doubles as `%.16g` (or C99 hex floats for `save(ascii = NA)`),
/// raw bytes as two hex digits, strings with C-style escapes.
pub struct Ascii<R> {
    r: ByteReader<R>,
    word: Vec<u8>,
}

impl<R: Read> Ascii<R> {
    pub fn new(r: ByteReader<R>) -> Self {
        Ascii {
            r,
            word: Vec::new(),
        }
    }

    fn malformed(&self, what: &str) -> Error {
        Error::Malformed {
            msg: format!("bad ASCII {what} {:?}", String::from_utf8_lossy(&self.word)),
            path: String::new(),
            offset: self.r.consumed(),
        }
    }

    fn real(&mut self) -> Result<f64> {
        self.r.word(&mut self.word)?;
        let w = &self.word[..];
        Ok(match w {
            b"NA" => NA_REAL,
            b"NaN" => f64::NAN,
            b"Inf" => f64::INFINITY,
            b"-Inf" => f64::NEG_INFINITY,
            _ => std::str::from_utf8(w)
                .ok()
                .and_then(parse_c_double)
                .ok_or_else(|| self.malformed("double"))?,
        })
    }

    fn byte(&mut self) -> Result<u8> {
        self.r.word(&mut self.word)?;
        std::str::from_utf8(&self.word)
            .ok()
            .and_then(|w| u8::from_str_radix(w, 16).ok())
            .ok_or_else(|| self.malformed("byte"))
    }

    /// `InString`: skip leading whitespace, then decode `len` (escaped) bytes.
    fn decode_string(&mut self, len: usize, mut out: Option<&mut Vec<u8>>) -> Result<()> {
        if len == 0 {
            return Ok(());
        }
        while self.r.peek()?.is_some_and(|b| b.is_ascii_whitespace()) {
            self.r.byte()?;
        }
        for _ in 0..len {
            let mut c = self.r.byte()?;
            if c == b'\\' {
                c = match self.r.byte()? {
                    b'n' => b'\n',
                    b't' => b'\t',
                    b'v' => 0x0b,
                    b'b' => 0x08,
                    b'r' => b'\r',
                    b'f' => 0x0c,
                    b'a' => 0x07,
                    d @ b'0'..=b'7' => {
                        let mut v = (d - b'0') as u32;
                        for _ in 0..2 {
                            match self.r.peek()? {
                                Some(d @ b'0'..=b'7') => {
                                    self.r.byte()?;
                                    v = v * 8 + (d - b'0') as u32;
                                }
                                _ => break,
                            }
                        }
                        v as u8
                    }
                    // `\\`, `\?`, `\'`, `\"` and anything unknown stand for themselves.
                    other => other,
                };
            }
            if let Some(out) = out.as_deref_mut() {
                out.push(c);
            }
        }
        Ok(())
    }
}

impl<R: Read> Wire for Ascii<R> {
    fn consumed(&self) -> u64 {
        self.r.consumed()
    }

    fn int(&mut self) -> Result<i32> {
        self.r.word(&mut self.word)?;
        if self.word == b"NA" {
            return Ok(NA_INTEGER);
        }
        std::str::from_utf8(&self.word)
            .ok()
            .and_then(|w| w.parse().ok())
            .ok_or_else(|| self.malformed("integer"))
    }

    fn ints(&mut self, out: &mut [i32]) -> Result<()> {
        for x in out {
            *x = self.int()?;
        }
        Ok(())
    }

    fn reals(&mut self, out: &mut [f64]) -> Result<()> {
        for x in out {
            *x = self.real()?;
        }
        Ok(())
    }

    fn raw(&mut self, out: &mut [u8]) -> Result<()> {
        for x in out {
            *x = self.byte()?;
        }
        Ok(())
    }

    fn string(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut v = Vec::with_capacity(len);
        self.decode_string(len, Some(&mut v))?;
        Ok(v)
    }

    fn skip_ints(&mut self, n: u64) -> Result<()> {
        (0..n).try_for_each(|_| self.r.word(&mut self.word))
    }

    fn skip_reals(&mut self, n: u64) -> Result<()> {
        self.skip_ints(n)
    }

    fn skip_raw(&mut self, n: u64) -> Result<()> {
        self.skip_ints(n)
    }

    fn skip_string(&mut self, len: usize) -> Result<()> {
        self.decode_string(len, None)
    }
}

/// Parse a double as C's `strtod` would for R's output: decimal (`%.16g`) or
/// hexadecimal floating point (`%a`, e.g. `-0x1.8p+3`).
fn parse_c_double(s: &str) -> Option<f64> {
    let (neg, body) = match s.as_bytes().first()? {
        b'-' => (true, &s[1..]),
        b'+' => (false, &s[1..]),
        _ => (false, s),
    };
    let Some(hex) = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X")) else {
        // Rust also accepts words such as "inf" that C prints differently; R never writes them.
        return s.parse().ok();
    };
    let (mant, exp) = hex.split_once(['p', 'P']).unwrap_or((hex, "0"));
    let mut exp: i64 = exp.parse().ok()?;
    let (int, frac) = mant.split_once('.').unwrap_or((mant, ""));
    if int.is_empty() && frac.is_empty() {
        return None;
    }
    let mut m: u64 = 0;
    for (i, c) in int.chars().chain(frac.chars()).enumerate() {
        let d = c.to_digit(16)? as u64;
        if m >> 60 == 0 {
            m = m * 16 + d;
            if i >= int.len() {
                exp -= 4;
            }
        } else if i < int.len() {
            exp += 4; // further integer digits only scale; R never writes this many
        }
    }
    // m * 2^exp with a single rounding: m is exact, and scaling by an exactly
    // representable power of two rounds once (twice only for absurd exponents).
    let mut v = m as f64;
    while exp > 1000 {
        v *= 2f64.powi(1000);
        exp -= 1000;
    }
    while exp < -1000 {
        v *= 2f64.powi(-1000);
        exp += 1000;
    }
    v *= 2f64.powi(exp as i32);
    Some(if neg { -v } else { v })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_doubles() {
        assert_eq!(
            parse_c_double("3.141592653589793"),
            Some(std::f64::consts::PI)
        );
        assert_eq!(parse_c_double("1e+05"), Some(1e5));
        assert_eq!(parse_c_double("-0"), Some(-0.0));
        assert_eq!(parse_c_double("0x1.8p+1"), Some(3.0));
        assert_eq!(parse_c_double("-0x1.999999999999ap-4"), Some(-0.1));
        assert_eq!(parse_c_double("0x0p+0"), Some(0.0));
        assert_eq!(parse_c_double("0x1p-1074"), Some(f64::from_bits(1)));
        assert_eq!(
            parse_c_double("0x0.0000000000001p-1022"),
            Some(f64::from_bits(1))
        );
        assert_eq!(parse_c_double("0x1.fffffffffffffp+1023"), Some(f64::MAX));
        assert_eq!(parse_c_double("0x"), None);
        assert_eq!(parse_c_double("abc"), None);
    }

    /// Hands out one byte per read, so every word straddles a refill.
    struct Trickle(std::io::Cursor<Vec<u8>>);

    impl Read for Trickle {
        fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
            let n = out.len().min(1);
            self.0.read(&mut out[..n])
        }
    }

    #[test]
    fn words_span_buffer_refills() {
        let text = format!("  12\n{}\n-7", "9".repeat(10));
        let mut r = ByteReader::new(Trickle(std::io::Cursor::new(text.clone().into_bytes())));
        let mut w = Vec::new();
        r.word(&mut w).unwrap();
        assert_eq!(w, b"12");
        r.word(&mut w).unwrap();
        assert_eq!(w, "9".repeat(10).as_bytes());
        r.word(&mut w).unwrap();
        assert_eq!(w, b"-7");
        assert_eq!(r.consumed(), text.len() as u64);
        assert!(r.word(&mut w).is_err());
    }

    #[test]
    fn ascii_strings_unescape() {
        let mut a = Ascii::new(ByteReader::new(std::io::Cursor::new(
            b"\n a\\040b\\n\\\\\\303\\251\\0x\n".to_vec(),
        )));
        assert_eq!(a.string(9).unwrap(), b"a b\n\\\xc3\xa9\0x");
    }
}
