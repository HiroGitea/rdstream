//! Decide which vectors to materialize.
//!
//! Paths name a position in the object tree:
//! `$name` is a tagged pairlist entry (e.g. an object in an RData file),
//! `@name` an attribute or S4 slot, `[k]` the k-th element of a list.
//! List names are serialized after the elements, so list elements are addressed
//! by index; `rdstream describe` prints both.
//!
//! Besides keeping or skipping a vector, a policy can hand the parser a [`Sink`]
//! that receives the vector's elements in chunks as they are read, so selected
//! vectors can be written out without ever being held in memory.

use std::io;

/// Consulted for every atomic or character vector before its data is read.
pub trait Policy {
    /// Load this vector into memory (`true`) or read past it (`false`).
    fn keep(&mut self, path: &str, sexptype: u8, len: u64) -> bool;

    /// Stream this vector to a [`Sink`] instead of keeping or skipping it.
    /// Asked before [`Policy::keep`]; the default declines.
    fn sink(&mut self, _path: &str, _sexptype: u8, _len: u64) -> io::Result<Option<Box<dyn Sink>>> {
        Ok(None)
    }
}

/// Receives a streamed vector's elements, in order, in bounded chunks.
pub trait Sink {
    /// Logical and integer vectors arrive as [`Chunk::Int`], doubles as
    /// [`Chunk::Real`], complex vectors as [`Chunk::Real`] with interleaved
    /// real and imaginary parts, raw vectors as [`Chunk::Raw`] and character
    /// vectors one element at a time as [`Chunk::Str`].
    fn write(&mut self, chunk: Chunk<'_>) -> io::Result<()>;

    /// Called after the last element. The returned label is stored in the
    /// document as [`crate::Value::Streamed`].
    fn finish(self: Box<Self>) -> io::Result<String>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Chunk<'a> {
    Int(&'a [i32]),
    Real(&'a [f64]),
    Raw(&'a [u8]),
    /// One element of a character vector; `None` is `NA_character_`.
    Str(Option<&'a str>),
}

/// Keep vectors up to `threshold` elements, plus anything matching a `keep` glob;
/// `skip` globs win over both.
#[derive(Debug, Clone)]
pub struct Rules {
    pub threshold: u64,
    pub keep: Vec<String>,
    pub skip: Vec<String>,
}

impl Default for Rules {
    fn default() -> Self {
        Rules {
            threshold: 1_000_000,
            keep: Vec::new(),
            skip: Vec::new(),
        }
    }
}

impl Rules {
    /// Keep everything.
    pub fn all() -> Self {
        Rules {
            threshold: u64::MAX,
            ..Default::default()
        }
    }
}

impl Policy for Rules {
    fn keep(&mut self, path: &str, _sexptype: u8, len: u64) -> bool {
        if self.skip.iter().any(|g| glob_match(g, path)) {
            return false;
        }
        len <= self.threshold || self.keep.iter().any(|g| glob_match(g, path))
    }
}

impl<F: FnMut(&str, u8, u64) -> bool> Policy for F {
    fn keep(&mut self, path: &str, sexptype: u8, len: u64) -> bool {
        self(path, sexptype, len)
    }
}

/// Glob match where `*` matches any run of characters (including separators)
/// and every other character is literal.
pub fn glob_match(pattern: &str, text: &str) -> bool {
    let (p, t) = (pattern.as_bytes(), text.as_bytes());
    let (mut pi, mut ti) = (0, 0);
    let mut star: Option<(usize, usize)> = None;
    while ti < t.len() {
        if pi < p.len() && p[pi] == b'*' {
            star = Some((pi, ti));
            pi += 1;
        } else if pi < p.len() && p[pi] == t[ti] {
            pi += 1;
            ti += 1;
        } else if let Some((sp, st)) = star {
            pi = sp + 1;
            ti = st + 1;
            star = Some((sp, st + 1));
        } else {
            return false;
        }
    }
    p[pi..].iter().all(|&c| c == b'*')
}

#[cfg(test)]
mod tests {
    use super::glob_match;

    #[test]
    fn globs() {
        assert!(glob_match("$x@assays[0]@counts*", "$x@assays[0]@counts@x"));
        assert!(glob_match("*@counts@x", "$obj@assays[3]@counts@x"));
        assert!(!glob_match("*@counts@x", "$obj@assays[3]@counts@i"));
        assert!(glob_match("*", ""));
        assert!(glob_match("a*b*c", "aXXbYYc"));
        assert!(!glob_match("a*b*c", "aXXbYY"));
        assert!(glob_match("[0]", "[0]"));
    }
}
