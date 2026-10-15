//! R object model produced by the parser.

/// SEXPTYPE codes as written in R's serialization stream (see R's `serialize.c`).
pub mod sexptype {
    pub const NIL: u8 = 0;
    pub const SYM: u8 = 1;
    pub const LIST: u8 = 2;
    pub const CLO: u8 = 3;
    pub const ENV: u8 = 4;
    pub const PROM: u8 = 5;
    pub const LANG: u8 = 6;
    pub const SPECIAL: u8 = 7;
    pub const BUILTIN: u8 = 8;
    pub const CHAR: u8 = 9;
    pub const LGL: u8 = 10;
    pub const INT: u8 = 13;
    pub const REAL: u8 = 14;
    pub const CPLX: u8 = 15;
    pub const STR: u8 = 16;
    pub const DOT: u8 = 17;
    pub const VEC: u8 = 19;
    pub const EXPR: u8 = 20;
    pub const BCODE: u8 = 21;
    pub const EXTPTR: u8 = 22;
    pub const WEAKREF: u8 = 23;
    pub const RAW: u8 = 24;
    pub const S4: u8 = 25;
    pub const ALTREP: u8 = 238;
    /// Inside byte code constant pools only.
    pub const BCREPREF: u8 = 243;
    pub const BCREPDEF: u8 = 244;
    pub const ATTRLIST: u8 = 239;
    pub const ATTRLANG: u8 = 240;
    pub const BASEENV: u8 = 241;
    pub const EMPTYENV: u8 = 242;
    pub const PERSIST: u8 = 247;
    pub const PACKAGE: u8 = 248;
    pub const NAMESPACE: u8 = 249;
    pub const BASENAMESPACE: u8 = 250;
    pub const MISSINGARG: u8 = 251;
    pub const UNBOUNDVALUE: u8 = 252;
    pub const GLOBALENV: u8 = 253;
    pub const NILVALUE: u8 = 254;
    pub const REF: u8 = 255;
}

use sexptype::*;

/// Short R-style name for a SEXPTYPE.
pub fn type_name(t: u8) -> &'static str {
    match t {
        NIL | NILVALUE => "NULL",
        SYM => "sym",
        LIST => "pairlist",
        CLO => "closure",
        ENV => "env",
        PROM => "promise",
        LANG => "call",
        SPECIAL => "special",
        BUILTIN => "builtin",
        CHAR => "char",
        LGL => "lgl",
        INT => "int",
        REAL => "dbl",
        CPLX => "cplx",
        STR => "chr",
        DOT => "dots",
        VEC => "list",
        EXPR => "expr",
        BCODE => "bytecode",
        EXTPTR => "extptr",
        WEAKREF => "weakref",
        RAW => "raw",
        S4 => "S4",
        BASEENV => "baseenv",
        EMPTYENV => "emptyenv",
        GLOBALENV => "globalenv",
        BASENAMESPACE => "basenamespace",
        MISSINGARG => "missingarg",
        UNBOUNDVALUE => "unbound",
        PACKAGE => "package",
        NAMESPACE => "namespace",
        PERSIST => "persist",
        _ => "?",
    }
}

/// R's integer/logical NA.
pub const NA_INTEGER: i32 = i32::MIN;
/// R's `NA_real_`: a NaN whose low word is 1954.
pub const NA_REAL: f64 = f64::from_bits(0x7ff0_0000_0000_07a2);

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Symbol(String),
    /// Pairlist-like nodes: pairlists, calls, closures, promises, `...`.
    Pairlist {
        sexptype: u8,
        items: Vec<(Option<String>, RObject)>,
    },
    Logical(Vec<i32>),
    Integer(Vec<i32>),
    Real(Vec<f64>),
    Complex(Vec<[f64; 2]>),
    Raw(Vec<u8>),
    /// `None` is `NA_character_`.
    Character(Vec<Option<String>>),
    List(Vec<RObject>),
    Expression(Vec<RObject>),
    /// S4 object; its slots are the attributes.
    S4,
    /// Index into [`Document::environments`].
    Environment(usize),
    Builtin {
        special: bool,
        name: String,
    },
    ExternalPointer,
    WeakReference,
    /// Singleton environments and markers (globalenv, baseenv, missing arg, ...).
    Singleton(u8),
    /// Byte-compiled code: the instruction vector and the constant pool (which
    /// holds the original expressions).
    Bytecode {
        code: Box<RObject>,
        consts: Vec<RObject>,
    },
    /// Package/namespace/persistent references, identified by a character vector.
    Reference {
        sexptype: u8,
        info: Vec<Option<String>>,
    },
    /// An ALTREP class this crate does not expand; `state` is its serialized state.
    AltRep {
        class: String,
        state: Box<RObject>,
    },
    /// A vector the policy chose not to load.
    Skipped {
        sexptype: u8,
        len: u64,
    },
    /// A vector streamed to a [`crate::policy::Sink`]; `target` is the label
    /// the sink returned.
    Streamed {
        sexptype: u8,
        len: u64,
        target: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct RObject {
    pub value: Value,
    pub attributes: Vec<(String, RObject)>,
    pub is_object: bool,
}

impl RObject {
    pub fn new(value: Value) -> Self {
        RObject {
            value,
            attributes: Vec::new(),
            is_object: false,
        }
    }

    pub fn null() -> Self {
        Self::new(Value::Null)
    }

    pub fn attr(&self, name: &str) -> Option<&RObject> {
        self.attributes
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v)
    }

    /// Value of the `class` attribute, if any.
    pub fn classes(&self) -> Vec<&str> {
        match self.attr("class").map(|c| &c.value) {
            Some(Value::Character(v)) => v.iter().flatten().map(String::as_str).collect(),
            _ => Vec::new(),
        }
    }

    pub fn inherits(&self, class: &str) -> bool {
        self.classes().contains(&class)
    }

    /// Value of the `names` attribute, if it is a character vector.
    pub fn names(&self) -> Option<&[Option<String>]> {
        match self.attr("names").map(|c| &c.value) {
            Some(Value::Character(v)) => Some(v),
            _ => None,
        }
    }

    /// Element of a named list (or tagged pairlist) by name.
    pub fn get(&self, name: &str) -> Option<&RObject> {
        match &self.value {
            Value::List(items) | Value::Expression(items) => {
                let names = self.names()?;
                names
                    .iter()
                    .position(|n| n.as_deref() == Some(name))
                    .and_then(|i| items.get(i))
            }
            Value::Pairlist { items, .. } => items
                .iter()
                .find(|(k, _)| k.as_deref() == Some(name))
                .map(|(_, v)| v),
            _ => None,
        }
    }

    /// SEXPTYPE this value was serialized as (best effort for expanded ALTREP).
    pub fn sexptype(&self) -> u8 {
        match &self.value {
            Value::Null => NILVALUE,
            Value::Symbol(_) => SYM,
            Value::Pairlist { sexptype, .. } => *sexptype,
            Value::Logical(_) => LGL,
            Value::Integer(_) => INT,
            Value::Real(_) => REAL,
            Value::Complex(_) => CPLX,
            Value::Raw(_) => RAW,
            Value::Character(_) => STR,
            Value::List(_) => VEC,
            Value::Expression(_) => EXPR,
            Value::S4 => S4,
            Value::Environment(_) => ENV,
            Value::Builtin { special: true, .. } => SPECIAL,
            Value::Builtin { special: false, .. } => BUILTIN,
            Value::Bytecode { .. } => BCODE,
            Value::ExternalPointer => EXTPTR,
            Value::WeakReference => WEAKREF,
            Value::Singleton(t) => *t,
            Value::Reference { sexptype, .. } => *sexptype,
            Value::AltRep { .. } => ALTREP,
            Value::Skipped { sexptype, .. } | Value::Streamed { sexptype, .. } => *sexptype,
        }
    }

    /// True for zero-length vectors and NULL.
    pub fn is_empty(&self) -> bool {
        self.len() == Some(0)
    }

    /// Length of a vector-like value (`None` for non-vectors).
    pub fn len(&self) -> Option<u64> {
        Some(match &self.value {
            Value::Null => 0,
            Value::Logical(v) | Value::Integer(v) => v.len() as u64,
            Value::Real(v) => v.len() as u64,
            Value::Complex(v) => v.len() as u64,
            Value::Raw(v) => v.len() as u64,
            Value::Character(v) => v.len() as u64,
            Value::List(v) | Value::Expression(v) => v.len() as u64,
            Value::Pairlist { items, .. } => items.len() as u64,
            Value::Skipped { len, .. } | Value::Streamed { len, .. } => *len,
            _ => return None,
        })
    }
}

/// An environment read from the stream. Environments can reference themselves,
/// so objects point to them by index instead of owning them.
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub locked: bool,
    pub enclosure: RObject,
    pub frame: RObject,
    pub hashtab: RObject,
    pub attributes: RObject,
}

/// How the stream encodes numbers and strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// Big-endian binary, R's default.
    Xdr,
    /// `ascii = TRUE` (decimal) or `ascii = NA` (hexadecimal doubles).
    Ascii,
    /// `xdr = FALSE`: the writer's native byte order, read as little-endian.
    Binary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// `saveRDS()`: a single object.
    Rds,
    /// `save()` (`.RData`, `.rda`, `.Robj`): a tagged pairlist of named objects.
    RData,
}

/// Result of parsing one stream.
#[derive(Debug, Clone)]
pub struct Document {
    pub format: Format,
    pub encoding: Encoding,
    /// Serialization format version (2 or 3).
    pub version: i32,
    /// R version that wrote the stream, as (major, minor, patch).
    pub writer_version: (u32, u32, u32),
    pub native_encoding: Option<String>,
    pub root: RObject,
    pub environments: Vec<Environment>,
    /// Decompressed bytes consumed.
    pub bytes_read: u64,
}

impl Document {
    /// Named objects of an RData file, or `("", root)` for an RDS file.
    pub fn objects(&self) -> Vec<(&str, &RObject)> {
        match (&self.format, &self.root.value) {
            (Format::RData, Value::Pairlist { items, .. }) => items
                .iter()
                .map(|(k, v)| (k.as_deref().unwrap_or(""), v))
                .collect(),
            _ => vec![("", &self.root)],
        }
    }
}
