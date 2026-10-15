//! Single-pass parser for R's serialization format, mirroring `ReadItem` in
//! R's `src/main/serialize.c`.

use std::fmt::{self, Write as _};
use std::io::Read;

use crate::error::{Error, Result};
use crate::policy::{Chunk, Policy, Sink};
use crate::reader::{Ascii, Binary, ByteReader, Wire};
use crate::sexp::sexptype::*;
use crate::sexp::{Document, Encoding, Environment, Format, NA_INTEGER, RObject, Value};

const IS_OBJECT: i32 = 1 << 8;
const HAS_ATTR: i32 = 1 << 9;
const HAS_TAG: i32 = 1 << 10;
const LATIN1_MASK: i32 = 1 << 2;
const UTF8_MASK: i32 = 1 << 3;
const ASCII_MASK: i32 = 1 << 6;
/// Elements per chunk handed to a [`Sink`].
const SINK_CHUNK: usize = 1 << 16;

fn is_pairlike(t: u8) -> bool {
    matches!(t, LIST | LANG | CLO | PROM | DOT | ATTRLIST | ATTRLANG)
}

/// Reads one serialized document from an already decompressed stream (see
/// [`crate::decompress`]), in any of R's encodings.
pub struct Parser<'p, R, P: ?Sized> {
    input: R,
    policy: &'p mut P,
}

impl<'p, R: Read, P: Policy + ?Sized> Parser<'p, R, P> {
    pub fn new(input: R, policy: &'p mut P) -> Self {
        Parser { input, policy }
    }

    pub fn parse_document(self) -> Result<Document> {
        let mut r = ByteReader::new(self.input);
        let mut head = [0u8; 2];
        r.read_exact(&mut head)?;
        let format = if &head == b"RD" {
            // save(): "RDX3\n", "RDA2\n", "RDB3\n", ... then the serialization header.
            let mut rest = [0u8; 3];
            r.read_exact(&mut rest)?;
            if !matches!(rest, [b'A' | b'B' | b'X', b'2' | b'3', b'\n']) {
                return Err(Error::NotSerialized([&head[..], &rest[..]].concat()));
            }
            r.read_exact(&mut head)?;
            Format::RData
        } else {
            Format::Rds
        };
        // Like R's `InFormat`, only the first byte counts ("A\r\n" from Windows is
        // fine), and a newline may precede ASCII's format byte.
        let encoding = match head {
            [b'X', _] => Encoding::Xdr,
            [b'A', _] => Encoding::Ascii,
            [b'B', _] => Encoding::Binary,
            [b'\n', b'A'] => {
                r.byte()?;
                Encoding::Ascii
            }
            _ => return Err(Error::NotSerialized(head.to_vec())),
        };
        match encoding {
            Encoding::Xdr => Items::new(Binary::xdr(r), self.policy).document(format, encoding),
            Encoding::Binary => {
                Items::new(Binary::native(r), self.policy).document(format, encoding)
            }
            Encoding::Ascii => Items::new(Ascii::new(r), self.policy).document(format, encoding),
        }
    }
}

enum Decision {
    Keep,
    Skip,
    Sink(Box<dyn Sink>),
}

struct Items<'p, W, P: ?Sized> {
    w: W,
    policy: &'p mut P,
    refs: Vec<RObject>,
    envs: Vec<Environment>,
    path: String,
    /// Set while reading internal state that must be loaded regardless of policy.
    force_keep: bool,
    /// Set while reading state whose vectors do not become the value the policy
    /// expects (a deferred string's numbers), so they are not offered to sinks.
    no_sink: bool,
    /// The writer's single-byte native encoding, for strings without an
    /// encoding flag that are not valid UTF-8.
    native: Option<SingleByte>,
}

#[derive(Clone, Copy)]
enum SingleByte {
    Latin1,
    Cp1252,
}

impl SingleByte {
    fn from_name(name: &str) -> Option<Self> {
        match name.to_ascii_uppercase().replace(['-', '_'], "").as_str() {
            "LATIN1" | "ISO88591" => Some(SingleByte::Latin1),
            "CP1252" | "WINDOWS1252" => Some(SingleByte::Cp1252),
            _ => None,
        }
    }

    fn decode(self, b: &[u8]) -> String {
        // Windows-1252 differs from Latin-1 only in 0x80..=0x9f.
        const CP1252: [char; 32] = [
            '€', '\u{81}', '‚', 'ƒ', '„', '…', '†', '‡', 'ˆ', '‰', 'Š', '‹', 'Œ', '\u{8d}', 'Ž',
            '\u{8f}', '\u{90}', '‘', '’', '“', '”', '•', '–', '—', '˜', '™', 'š', '›', 'œ',
            '\u{9d}', 'ž', 'Ÿ',
        ];
        b.iter()
            .map(|&c| match (self, c) {
                (SingleByte::Cp1252, 0x80..=0x9f) => CP1252[(c - 0x80) as usize],
                _ => c as char,
            })
            .collect()
    }
}

impl<'p, W: Wire, P: Policy + ?Sized> Items<'p, W, P> {
    fn new(w: W, policy: &'p mut P) -> Self {
        Items {
            w,
            policy,
            refs: Vec::new(),
            envs: Vec::new(),
            path: String::new(),
            force_keep: false,
            no_sink: false,
            native: None,
        }
    }

    fn document(mut self, format: Format, encoding: Encoding) -> Result<Document> {
        self.body(format, encoding).map_err(|e| match e {
            // The wire reports malformed numbers without knowing where they are.
            Error::Malformed { msg, path, offset } if path.is_empty() => Error::Malformed {
                msg,
                path: self.path.clone(),
                offset,
            },
            e => e,
        })
    }

    fn body(&mut self, format: Format, encoding: Encoding) -> Result<Document> {
        let version = self.w.int()?;
        let writer = self.w.int()?;
        let _min_reader = self.w.int()?;
        let native_encoding = if version == 3 {
            let n = self.w.int()?;
            let n = usize::try_from(n)
                .map_err(|_| self.malformed(format!("bad encoding length {n}")))?;
            Some(String::from_utf8_lossy(&self.w.string(n)?).into_owned())
        } else {
            None
        };
        self.native = native_encoding.as_deref().and_then(SingleByte::from_name);
        let root = self.item()?;
        let w = writer as u32;
        Ok(Document {
            format,
            encoding,
            version,
            writer_version: (w >> 16, (w >> 8) & 0xff, w & 0xff),
            native_encoding,
            root,
            environments: std::mem::take(&mut self.envs),
            bytes_read: self.w.consumed(),
        })
    }

    fn malformed(&self, msg: impl Into<String>) -> Error {
        Error::Malformed {
            msg: msg.into(),
            path: self.path.clone(),
            offset: self.w.consumed(),
        }
    }

    fn unsupported(&self, what: impl Into<String>) -> Error {
        Error::Unsupported {
            what: what.into(),
            path: self.path.clone(),
            offset: self.w.consumed(),
        }
    }

    fn decide(&mut self, t: u8, n: u64) -> Result<Decision> {
        if self.force_keep {
            return Ok(Decision::Keep);
        }
        if !self.no_sink
            && let Some(sink) = self.policy.sink(&self.path, t, n)?
        {
            return Ok(Decision::Sink(sink));
        }
        Ok(if self.policy.keep(&self.path, t, n) {
            Decision::Keep
        } else {
            Decision::Skip
        })
    }

    fn item(&mut self) -> Result<RObject> {
        let flags = self.w.int()?;
        self.item_flags(flags)
    }

    /// Parse an item with `seg` appended to the current path. On error the path
    /// is left pointing at the failure.
    fn item_at(&mut self, seg: fmt::Arguments) -> Result<RObject> {
        let mark = self.path.len();
        self.path.write_fmt(seg).unwrap();
        let res = self.item()?;
        self.path.truncate(mark);
        Ok(res)
    }

    fn length(&mut self) -> Result<u64> {
        match self.w.int()? {
            -1 => {
                let hi = self.w.int()? as u32 as u64;
                let lo = self.w.int()? as u32 as u64;
                Ok((hi << 32) | lo)
            }
            n if n < -1 => Err(self.malformed(format!("negative length {n}"))),
            n => Ok(n as u64),
        }
    }

    fn usize_len(&self, n: u64) -> Result<usize> {
        usize::try_from(n).map_err(|_| self.malformed(format!("length {n} exceeds address space")))
    }

    fn charsxp(&mut self, flags: i32) -> Result<Option<String>> {
        let n = self.w.int()?;
        if n == -1 {
            return Ok(None);
        }
        if n < 0 {
            return Err(self.malformed(format!("bad string length {n}")));
        }
        let b = self.w.string(n as usize)?;
        let levels = flags >> 12;
        Ok(Some(if levels & LATIN1_MASK != 0 {
            SingleByte::Latin1.decode(&b)
        } else {
            String::from_utf8(b).unwrap_or_else(|e| match self.native {
                // Unflagged strings are in the writer's native encoding.
                Some(native) if levels & (UTF8_MASK | ASCII_MASK) == 0 => {
                    native.decode(e.as_bytes())
                }
                _ => String::from_utf8_lossy(e.as_bytes()).into_owned(),
            })
        }))
    }

    /// `InStringVec`: a leading 0, a length, then CHARSXP items.
    fn string_vec(&mut self) -> Result<Vec<Option<String>>> {
        if self.w.int()? != 0 {
            return Err(self.malformed("bad string vector"));
        }
        let n = self.w.int()?;
        (0..n)
            .map(|_| {
                let f = self.w.int()?;
                self.charsxp(f)
            })
            .collect()
    }

    fn read_vec<T: Clone + Default>(
        &mut self,
        n: u64,
        read: fn(&mut W, &mut [T]) -> Result<()>,
    ) -> Result<Vec<T>> {
        let mut v = vec![T::default(); self.usize_len(n)?];
        read(&mut self.w, &mut v)?;
        Ok(v)
    }

    /// Pass `elems` wire elements to `sink` in chunks.
    fn stream<T: Clone + Default>(
        &mut self,
        (t, len): (u8, u64),
        elems: u64,
        mut sink: Box<dyn Sink>,
        read: fn(&mut W, &mut [T]) -> Result<()>,
        wrap: for<'a> fn(&'a [T]) -> Chunk<'a>,
    ) -> Result<Value> {
        let mut buf = vec![T::default(); elems.min(SINK_CHUNK as u64) as usize];
        let mut left = elems;
        while left > 0 {
            let k = left.min(SINK_CHUNK as u64) as usize;
            read(&mut self.w, &mut buf[..k])?;
            sink.write(wrap(&buf[..k]))?;
            left -= k as u64;
        }
        Ok(Value::Streamed {
            sexptype: t,
            len,
            target: sink.finish()?,
        })
    }

    fn item_flags(&mut self, flags: i32) -> Result<RObject> {
        let t = (flags & 0xff) as u8;
        let is_object = flags & IS_OBJECT != 0;
        let value = match t {
            NILVALUE => return Ok(RObject::null()),
            BASEENV | EMPTYENV | GLOBALENV | UNBOUNDVALUE | MISSINGARG | BASENAMESPACE => {
                return Ok(RObject::new(Value::Singleton(t)));
            }
            REF => {
                let packed = (flags >> 8) as usize;
                let i = if packed == 0 {
                    self.w.int()? as usize
                } else {
                    packed
                };
                return match i.checked_sub(1).and_then(|k| self.refs.get(k)) {
                    Some(o) => Ok(o.clone()),
                    None => Err(self.malformed(format!("reference #{i} out of range"))),
                };
            }
            PERSIST | PACKAGE | NAMESPACE => {
                let info = self.string_vec()?;
                let o = RObject::new(Value::Reference { sexptype: t, info });
                self.refs.push(o.clone());
                return Ok(o);
            }
            SYM => {
                let f = self.w.int()?;
                let o = RObject::new(Value::Symbol(self.charsxp(f)?.unwrap_or_default()));
                self.refs.push(o.clone());
                return Ok(o);
            }
            ENV => return self.environment(),
            t if is_pairlike(t) => return self.pairlist(flags, '$'),
            ALTREP => return self.altrep(is_object),
            CHAR => Value::Character(vec![self.charsxp(flags)?]),
            LGL | INT => {
                let n = self.length()?;
                match self.decide(t, n)? {
                    Decision::Keep => {
                        let v = self.read_vec(n, W::ints)?;
                        if t == LGL {
                            Value::Logical(v)
                        } else {
                            Value::Integer(v)
                        }
                    }
                    Decision::Skip => {
                        self.w.skip_ints(n)?;
                        Value::Skipped {
                            sexptype: t,
                            len: n,
                        }
                    }
                    Decision::Sink(s) => self.stream((t, n), n, s, W::ints, |c| Chunk::Int(c))?,
                }
            }
            REAL => {
                let n = self.length()?;
                match self.decide(t, n)? {
                    Decision::Keep => Value::Real(self.read_vec(n, W::reals)?),
                    Decision::Skip => {
                        self.w.skip_reals(n)?;
                        Value::Skipped {
                            sexptype: t,
                            len: n,
                        }
                    }
                    Decision::Sink(s) => self.stream((t, n), n, s, W::reals, |c| Chunk::Real(c))?,
                }
            }
            CPLX => {
                let n = self.length()?;
                let parts = n
                    .checked_mul(2)
                    .ok_or_else(|| self.malformed(format!("complex length {n}")))?;
                match self.decide(t, n)? {
                    Decision::Keep => {
                        let flat = self.read_vec(parts, W::reals)?;
                        Value::Complex(flat.chunks_exact(2).map(|c| [c[0], c[1]]).collect())
                    }
                    Decision::Skip => {
                        self.w.skip_reals(parts)?;
                        Value::Skipped {
                            sexptype: t,
                            len: n,
                        }
                    }
                    Decision::Sink(s) => {
                        self.stream((t, n), parts, s, W::reals, |c| Chunk::Real(c))?
                    }
                }
            }
            RAW => {
                let n = self.length()?;
                match self.decide(t, n)? {
                    Decision::Keep => Value::Raw(self.read_vec(n, W::raw)?),
                    Decision::Skip => {
                        self.w.skip_raw(n)?;
                        Value::Skipped {
                            sexptype: t,
                            len: n,
                        }
                    }
                    Decision::Sink(s) => self.stream((t, n), n, s, W::raw, |c| Chunk::Raw(c))?,
                }
            }
            STR => {
                let n = self.length()?;
                match self.decide(t, n)? {
                    Decision::Keep => {
                        let mut v = Vec::with_capacity(self.usize_len(n)?.min(1 << 20));
                        for _ in 0..n {
                            let f = self.w.int()?;
                            v.push(self.charsxp(f)?);
                        }
                        Value::Character(v)
                    }
                    Decision::Skip => {
                        for _ in 0..n {
                            self.w.int()?;
                            let k = self.w.int()?;
                            if k > 0 {
                                self.w.skip_string(k as usize)?;
                            }
                        }
                        Value::Skipped {
                            sexptype: t,
                            len: n,
                        }
                    }
                    Decision::Sink(mut sink) => {
                        for _ in 0..n {
                            let f = self.w.int()?;
                            let s = self.charsxp(f)?;
                            sink.write(Chunk::Str(s.as_deref()))?;
                        }
                        Value::Streamed {
                            sexptype: t,
                            len: n,
                            target: sink.finish()?,
                        }
                    }
                }
            }
            VEC | EXPR => {
                let n = self.length()?;
                let mut v = Vec::with_capacity(self.usize_len(n)?.min(1 << 16));
                for k in 0..n {
                    v.push(self.item_at(format_args!("[{k}]"))?);
                }
                if t == VEC {
                    Value::List(v)
                } else {
                    Value::Expression(v)
                }
            }
            S4 => Value::S4,
            EXTPTR => {
                self.refs.push(RObject::new(Value::ExternalPointer));
                self.item_at(format_args!("<prot>"))?;
                self.item_at(format_args!("<tag>"))?;
                Value::ExternalPointer
            }
            WEAKREF => {
                self.refs.push(RObject::new(Value::WeakReference));
                Value::WeakReference
            }
            SPECIAL | BUILTIN => {
                let n = self.w.int()?;
                let n = usize::try_from(n)
                    .map_err(|_| self.malformed(format!("bad builtin name length {n}")))?;
                let name = String::from_utf8_lossy(&self.w.string(n)?).into_owned();
                Value::Builtin {
                    special: t == SPECIAL,
                    name,
                }
            }
            BCODE => {
                let n = self.w.int()?;
                let mut reps = vec![
                    None;
                    usize::try_from(n).map_err(
                        |_| self.malformed(format!("bad bytecode reps {n}"))
                    )?
                ];
                self.bytecode(&mut reps)?
            }
            _ => return Err(self.unsupported(format!("SEXPTYPE {t}"))),
        };
        let attributes = if flags & HAS_ATTR != 0 {
            self.attributes()?
        } else {
            Vec::new()
        };
        Ok(RObject {
            value,
            attributes,
            is_object,
        })
    }

    /// `ReadBC1`: the code vector, then the constant pool, whose language
    /// objects use a compact encoding that can share repeated subtrees.
    fn bytecode(&mut self, reps: &mut Vec<Option<RObject>>) -> Result<Value> {
        let code = self.item_at(format_args!("<code>"))?;
        let n = self.w.int()?;
        let mut consts = Vec::with_capacity(usize::try_from(n).unwrap_or(0).min(1 << 16));
        for k in 0..n {
            let t = self.w.int()?;
            let mark = self.path.len();
            write!(self.path, "<consts>[{k}]").unwrap();
            let c = match u8::try_from(t) {
                Ok(BCODE) => RObject::new(self.bytecode(reps)?),
                Ok(LANG | LIST | BCREPDEF | BCREPREF | ATTRLANG | ATTRLIST) => {
                    self.bytecode_lang(t, reps)?
                }
                _ => self.item()?,
            };
            self.path.truncate(mark);
            consts.push(c);
        }
        Ok(Value::Bytecode {
            code: Box::new(code),
            consts,
        })
    }

    /// `ReadBCLang`: a call or pairlist node of a constant pool.
    fn bytecode_lang(&mut self, t: i32, reps: &mut Vec<Option<RObject>>) -> Result<RObject> {
        let t = u8::try_from(t).unwrap_or(0);
        if t == BCREPREF {
            let i = self.w.int()?;
            return match usize::try_from(i).ok().and_then(|i| reps.get(i)) {
                // A reference into a node still being read would be a cycle; R
                // never writes those for code, so a placeholder suffices.
                Some(o) => Ok(o.clone().unwrap_or_else(RObject::null)),
                None => Err(self.malformed(format!("bytecode reference #{i} out of range"))),
            };
        }
        if !matches!(t, BCREPDEF | LANG | LIST | ATTRLANG | ATTRLIST) {
            return self.item(); // after a padding 0
        }
        let (pos, t) = if t == BCREPDEF {
            (Some(self.w.int()?), self.w.int()? as u8)
        } else {
            (None, t)
        };
        let (sexptype, has_attr) = match t {
            ATTRLANG => (LANG, true),
            ATTRLIST => (LIST, true),
            t => (t, false),
        };
        let attributes = if has_attr {
            self.attributes()?
        } else {
            Vec::new()
        };
        let tag = match self.item()?.value {
            Value::Symbol(s) => Some(s),
            _ => None,
        };
        let car_type = self.w.int()?;
        let car = self.bytecode_lang(car_type, reps)?;
        let cdr_type = self.w.int()?;
        let cdr = self.bytecode_lang(cdr_type, reps)?;
        let mut items = vec![(tag, car)];
        match cdr.value {
            Value::Null => {}
            Value::Pairlist {
                sexptype: LIST,
                items: rest,
            } if cdr.attributes.is_empty() => items.extend(rest),
            _ => items.push((Some("<cdr>".into()), cdr)),
        }
        let node = RObject {
            value: Value::Pairlist { sexptype, items },
            attributes,
            is_object: false,
        };
        if let Some(pos) = pos {
            match usize::try_from(pos).ok().and_then(|p| reps.get_mut(p)) {
                Some(slot) => *slot = Some(node.clone()),
                None => {
                    return Err(self.malformed(format!("bytecode definition #{pos} out of range")));
                }
            }
        }
        Ok(node)
    }

    fn environment(&mut self) -> Result<RObject> {
        let locked = self.w.int()? != 0;
        let idx = self.envs.len();
        // Registered before its contents: environments may contain references to themselves.
        self.envs.push(Environment {
            locked,
            enclosure: RObject::null(),
            frame: RObject::null(),
            hashtab: RObject::null(),
            attributes: RObject::null(),
        });
        self.refs.push(RObject::new(Value::Environment(idx)));
        let enclosure = self.item_at(format_args!("<enclos>"))?;
        let frame = self.item_at(format_args!("<frame>"))?;
        let hashtab = self.item_at(format_args!("<hashtab>"))?;
        let attributes = self.item_at(format_args!("<attr>"))?;
        self.envs[idx] = Environment {
            locked,
            enclosure,
            frame,
            hashtab,
            attributes,
        };
        Ok(RObject::new(Value::Environment(idx)))
    }

    fn attributes(&mut self) -> Result<Vec<(String, RObject)>> {
        let flags = self.w.int()?;
        self.attributes_flags(flags)
    }

    fn attributes_flags(&mut self, flags: i32) -> Result<Vec<(String, RObject)>> {
        let t = (flags & 0xff) as u8;
        if t == NILVALUE {
            return Ok(Vec::new());
        }
        if !is_pairlike(t) {
            return Err(self.malformed(format!("attributes are SEXPTYPE {t}, expected a pairlist")));
        }
        match self.pairlist(flags, '@')?.value {
            Value::Pairlist { items, .. } => Ok(items
                .into_iter()
                .map(|(k, v)| {
                    // S4 slots cannot hold NULL, so R stores the symbol `\001NULL\001` instead.
                    let v = match &v.value {
                        Value::Symbol(s) if s == "\u{1}NULL\u{1}" => RObject::null(),
                        _ => v,
                    };
                    (k.unwrap_or_default(), v)
                })
                .collect()),
            _ => unreachable!(),
        }
    }

    /// Pairlist-like nodes, walked along CDR iteratively so long lists don't recurse.
    fn pairlist(&mut self, mut flags: i32, sep: char) -> Result<RObject> {
        let sexptype = (flags & 0xff) as u8;
        let is_object = flags & IS_OBJECT != 0;
        let mut items: Vec<(Option<String>, RObject)> = Vec::new();
        let mut attributes = Vec::new();
        loop {
            if flags & HAS_ATTR != 0 {
                let a = self.attributes()?;
                if attributes.is_empty() {
                    attributes = a;
                }
            }
            let mut tag = None;
            if flags & HAS_TAG != 0 {
                let t = self.item()?;
                match t.value {
                    Value::Symbol(s) => tag = Some(s),
                    // e.g. a closure's environment is serialized in the tag position
                    _ => items.push((Some("<tag>".into()), t)),
                }
            }
            let car = match &tag {
                Some(name) => self.item_at(format_args!("{sep}{name}"))?,
                None => {
                    let k = items.len();
                    self.item_at(format_args!("[{k}]"))?
                }
            };
            items.push((tag, car));
            flags = self.w.int()?;
            let t = (flags & 0xff) as u8;
            if t == NILVALUE {
                break;
            }
            if !is_pairlike(t) {
                let cdr = self.item_flags(flags)?;
                items.push((Some("<cdr>".into()), cdr));
                break;
            }
        }
        Ok(RObject {
            value: Value::Pairlist { sexptype, items },
            attributes,
            is_object,
        })
    }

    fn altrep(&mut self, is_object: bool) -> Result<RObject> {
        let info = self.item_at(format_args!("<altinfo>"))?;
        let class = match &info.value {
            Value::Pairlist { items, .. } => match items.first().map(|(_, v)| &v.value) {
                Some(Value::Symbol(s)) => s.clone(),
                _ => return Err(self.malformed("ALTREP info without class symbol")),
            },
            _ => return Err(self.malformed("ALTREP info is not a pairlist")),
        };
        // A compact sequence's state is just (length, start, step); the policy is
        // applied to the expanded length below instead.
        let saved = (self.force_keep, self.no_sink);
        self.force_keep |= class.starts_with("compact_");
        self.no_sink |= class == "deferred_string";
        let state = self.item()?;
        (self.force_keep, self.no_sink) = saved;
        let attributes = self.attributes()?;
        let first = |state: &RObject| match &state.value {
            Value::Pairlist { items, .. } => items.first().map(|(_, v)| v.clone()),
            _ => None,
        };
        let value = match class.as_str() {
            "compact_intseq" | "compact_realseq" => {
                let (n, start, step) = match &state.value {
                    Value::Real(v) if v.len() == 3 => (v[0] as u64, v[1], v[2]),
                    Value::Integer(v) if v.len() == 3 => (v[0] as u64, v[1] as f64, v[2] as f64),
                    _ => return Err(self.malformed(format!("bad {class} state"))),
                };
                let int = class == "compact_intseq";
                let t = if int { INT } else { REAL };
                let at = |k: u64| start + step * k as f64;
                match self.decide(t, n)? {
                    Decision::Skip => Value::Skipped {
                        sexptype: t,
                        len: n,
                    },
                    Decision::Keep if int => Value::Integer((0..n).map(|k| at(k) as i32).collect()),
                    Decision::Keep => Value::Real((0..n).map(at).collect()),
                    Decision::Sink(mut sink) => {
                        for lo in (0..n).step_by(SINK_CHUNK) {
                            let ks = lo..n.min(lo + SINK_CHUNK as u64);
                            if int {
                                sink.write(Chunk::Int(
                                    &ks.map(|k| at(k) as i32).collect::<Vec<_>>(),
                                ))?;
                            } else {
                                sink.write(Chunk::Real(&ks.map(at).collect::<Vec<_>>()))?;
                            }
                        }
                        Value::Streamed {
                            sexptype: t,
                            len: n,
                            target: sink.finish()?,
                        }
                    }
                }
            }
            "deferred_string" => match first(&state).map(|o| o.value) {
                Some(Value::Integer(v)) => Value::Character(
                    v.iter()
                        .map(|&x| (x != NA_INTEGER).then(|| x.to_string()))
                        .collect(),
                ),
                Some(Value::Real(v)) => {
                    Value::Character(v.iter().map(|&x| format_r_double(x)).collect())
                }
                Some(Value::Skipped { len, .. }) => Value::Skipped { sexptype: STR, len },
                _ => return Err(self.malformed("bad deferred_string state")),
            },
            c if c.starts_with("wrap_") => match first(&state) {
                Some(inner) => {
                    let attributes = if attributes.is_empty() {
                        inner.attributes
                    } else {
                        attributes
                    };
                    return Ok(RObject {
                        value: inner.value,
                        attributes,
                        is_object,
                    });
                }
                None => return Err(self.malformed(format!("bad {class} state"))),
            },
            _ => Value::AltRep {
                class,
                state: Box::new(state),
            },
        };
        Ok(RObject {
            value,
            attributes,
            is_object,
        })
    }
}

/// `as.character()` of a double: up to 15 significant digits, choosing fixed or
/// scientific notation by width the way R does with `scipen = 0`.
fn format_r_double(x: f64) -> Option<String> {
    if x.is_nan() {
        // R's NA_real_ is a NaN with payload 1954; other NaNs print as "NaN".
        return if x.to_bits() & 0xffff_ffff == 1954 {
            None
        } else {
            Some("NaN".into())
        };
    }
    if x.is_infinite() {
        return Some(if x > 0.0 { "Inf" } else { "-Inf" }.into());
    }
    let sci = {
        let s = format!("{:.14e}", x);
        let (mant, exp) = s.split_once('e').unwrap();
        let mant = if mant.contains('.') {
            mant.trim_end_matches('0').trim_end_matches('.')
        } else {
            mant
        };
        let exp: i32 = exp.parse().unwrap();
        format!("{mant}e{}{:02}", if exp < 0 { '-' } else { '+' }, exp.abs())
    };
    let sig = sci
        .split_once('e')
        .unwrap()
        .0
        .chars()
        .filter(char::is_ascii_digit)
        .count();
    let exp: i32 = sci.split_once('e').unwrap().1.parse().unwrap();
    let decimals = (sig as i32 - 1 - exp).max(0) as usize;
    let fixed = format!("{:.*}", decimals, x);
    Some(if fixed.len() <= sci.len() { fixed } else { sci })
}

#[cfg(test)]
mod tests {
    use super::format_r_double;

    #[test]
    fn r_double_formatting() {
        let f = |x| format_r_double(x).unwrap();
        assert_eq!(f(1.0), "1");
        assert_eq!(f(100000.0), "1e+05");
        assert_eq!(f(123456.0), "123456");
        assert_eq!(f(0.1), "0.1");
        assert_eq!(f(1.5), "1.5");
        assert_eq!(f(-2.25), "-2.25");
        assert_eq!(f(1e-20), "1e-20");
        assert_eq!(f(f64::INFINITY), "Inf");
    }
}
