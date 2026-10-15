//! Serialization streams assembled byte by byte from the format in R's
//! serialize.c, shared by the integration tests.
#![allow(dead_code)]

use rdstream::sexp::NA_REAL;

pub const LGL: u8 = 10;
pub const INT: u8 = 13;
pub const REAL: u8 = 14;
pub const STR: u8 = 16;
pub const VEC: u8 = 19;
pub const LIST: u8 = 2;
pub const S4: u8 = 25;
pub const ENV: u8 = 4;
pub const ALTREP: u8 = 238;
pub const CPLX: u8 = 15;
pub const RAW: u8 = 24;

#[derive(Clone, Copy, PartialEq)]
pub enum Enc {
    Xdr,
    Ascii,
    Binary,
}

/// Serialization stream writer, mirroring `OutInteger`/`OutReal`/`OutString`.
pub struct W(pub Vec<u8>, pub Enc);

impl W {
    pub fn rds() -> W {
        W::rds_in(Enc::Xdr)
    }
    pub fn rds_in(enc: Enc) -> W {
        let mut w = W(
            match enc {
                Enc::Xdr => b"X\n",
                Enc::Ascii => b"A\n",
                Enc::Binary => b"B\n",
            }
            .to_vec(),
            enc,
        );
        w.i(3).i(0x040301).i(0x030500).i(5).text(b"UTF-8");
        w
    }
    pub fn rdata() -> W {
        let mut w = W(b"RDX2\nX\n".to_vec(), Enc::Xdr);
        w.i(2).i(0x040301).i(0x020300);
        w
    }
    pub fn i(&mut self, v: i32) -> &mut Self {
        match self.1 {
            Enc::Xdr => self.0.extend_from_slice(&v.to_be_bytes()),
            Enc::Binary => self.0.extend_from_slice(&v.to_le_bytes()),
            Enc::Ascii if v == i32::MIN => self.0.extend_from_slice(b"NA\n"),
            Enc::Ascii => self.0.extend_from_slice(format!("{v}\n").as_bytes()),
        }
        self
    }
    pub fn real(&mut self, v: f64) -> &mut Self {
        match self.1 {
            Enc::Xdr => self.0.extend_from_slice(&v.to_be_bytes()),
            Enc::Binary => self.0.extend_from_slice(&v.to_le_bytes()),
            Enc::Ascii => {
                let s = match v {
                    _ if v.to_bits() == NA_REAL.to_bits() => "NA".into(),
                    _ if v.is_nan() => "NaN".into(),
                    f64::INFINITY => "Inf".into(),
                    f64::NEG_INFINITY => "-Inf".into(),
                    _ => format!("{v:?}"),
                };
                self.0.extend_from_slice(s.as_bytes());
                self.0.push(b'\n');
            }
        }
        self
    }
    pub fn byte(&mut self, b: u8) -> &mut Self {
        match self.1 {
            Enc::Ascii => self.0.extend_from_slice(format!("{b:02x}\n").as_bytes()),
            _ => self.0.push(b),
        }
        self
    }
    /// String bytes (after their length), escaped like R's `OutString` in ASCII.
    pub fn text(&mut self, s: &[u8]) -> &mut Self {
        if self.1 != Enc::Ascii {
            self.0.extend_from_slice(s);
            return self;
        }
        for &c in s {
            let esc = match c {
                b'\n' => "\\n".into(),
                b'\t' => "\\t".into(),
                b'\\' => "\\\\".into(),
                b'"' => "\\\"".into(),
                b'\'' => "\\'".into(),
                b'?' => "\\?".into(),
                c if c <= 32 || c > 126 => format!("\\{c:03o}"),
                c => (c as char).to_string(),
            };
            self.0.extend_from_slice(esc.as_bytes());
        }
        self.0.push(b'\n');
        self
    }
    pub fn flags(&mut self, t: u8, obj: bool, attr: bool, tag: bool) -> &mut Self {
        self.i(t as i32 | (obj as i32) << 8 | (attr as i32) << 9 | (tag as i32) << 10)
    }
    pub fn chars(&mut self, s: Option<&str>) -> &mut Self {
        match s {
            None => self.i(9).i(-1),
            Some(s) => self
                .i(9 | (1 << 3) << 12)
                .i(s.len() as i32)
                .text(s.as_bytes()),
        }
    }
    pub fn sym(&mut self, name: &str) -> &mut Self {
        self.i(1).chars(Some(name))
    }
    pub fn reference(&mut self, idx: i32) -> &mut Self {
        self.i(idx << 8 | 255)
    }
    pub fn nil(&mut self) -> &mut Self {
        self.i(254)
    }
    pub fn ints(&mut self, v: &[i32], attr: bool) -> &mut Self {
        self.flags(INT, false, attr, false).i(v.len() as i32);
        v.iter().for_each(|&x| {
            self.i(x);
        });
        self
    }
    pub fn reals(&mut self, v: &[f64], attr: bool) -> &mut Self {
        self.flags(REAL, false, attr, false).i(v.len() as i32);
        v.iter().for_each(|&x| {
            self.real(x);
        });
        self
    }
    pub fn strs(&mut self, v: &[Option<&str>], attr: bool) -> &mut Self {
        self.flags(STR, false, attr, false).i(v.len() as i32);
        v.iter().for_each(|s| {
            self.chars(*s);
        });
        self
    }
    /// Start a tagged pairlist node whose tag is a new symbol.
    pub fn node(&mut self, tag: &str) -> &mut Self {
        self.flags(LIST, false, false, true).sym(tag)
    }
    pub fn node_ref(&mut self, idx: i32) -> &mut Self {
        self.flags(LIST, false, false, true).reference(idx)
    }
}

/// data.frame(g = factor(c("u", "v", "u"))) with compact row names c(NA, -3L).
pub fn data_frame() -> W {
    let mut w = W::rds();
    w.flags(VEC, true, true, false).i(1);
    w.ints(&[1, 2, 1], true);
    w.node("levels").strs(&[Some("u"), Some("v")], false);
    w.node("class").strs(&[Some("factor")], false).nil();
    w.node("names").strs(&[Some("g")], false);
    w.node_ref(2).strs(&[Some("data.frame")], false); // #2 = sym class
    w.node("row.names").ints(&[i32::MIN, -3], false).nil();
    w
}

/// `1:n` as R serializes it: an ALTREP compact_intseq with state (n, 1, 1).
pub fn altrep_compact_intseq(n: f64) -> W {
    let mut w = W::rds();
    w.i(ALTREP as i32);
    w.flags(LIST, false, false, false).sym("compact_intseq");
    w.flags(LIST, false, false, false).sym("base");
    w.flags(LIST, false, false, false)
        .ints(&[INT as i32], false)
        .nil();
    w.reals(&[n, 1.0, 1.0], false);
    w.nil();
    w
}
