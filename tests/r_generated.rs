//! Checks files written by `tests/r/generate.R` against what R reported about
//! them in `expected.json`. CI runs R to create them; locally, run
//! `Rscript tests/r/generate.R target/r-fixtures` and set
//! `RDSTREAM_R_FIXTURES=target/r-fixtures`, or this test does nothing.

use std::collections::BTreeMap;
use std::path::PathBuf;

use rdstream::sexp::{NA_INTEGER, NA_REAL, sexptype};
use rdstream::{DecodeOptions, Encoding, Format, RObject, Rules, Source, Value};
use serde_json::Value as Json;

fn fixtures() -> Option<PathBuf> {
    let dir = PathBuf::from(std::env::var_os("RDSTREAM_R_FIXTURES")?);
    Some(if dir.is_absolute() {
        dir
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(dir)
    })
}

/// R's `typeof()` for a parsed object.
fn r_type(o: &RObject) -> &'static str {
    match o.sexptype() {
        sexptype::NIL | sexptype::NILVALUE => "NULL",
        sexptype::INT => "integer",
        sexptype::REAL => "double",
        sexptype::LGL => "logical",
        sexptype::STR => "character",
        sexptype::CPLX => "complex",
        sexptype::RAW => "raw",
        sexptype::VEC => "list",
        sexptype::EXPR => "expression",
        sexptype::S4 => "S4",
        sexptype::ENV | sexptype::GLOBALENV | sexptype::BASEENV | sexptype::EMPTYENV => {
            "environment"
        }
        sexptype::CLO => "closure",
        sexptype::SYM => "symbol",
        sexptype::LANG => "language",
        sexptype::LIST => "pairlist",
        _ => "?",
    }
}

struct Check<'a> {
    file: &'a str,
    /// ASCII files hold doubles to 16 significant digits only.
    ascii: bool,
}

impl Check<'_> {
    fn double(&self, expected: &Json, x: f64, at: &str) {
        let e = expected.as_str().unwrap();
        let ok = match e {
            "NA" => x.to_bits() == NA_REAL.to_bits(),
            "NaN" => x.is_nan() && x.to_bits() != NA_REAL.to_bits(),
            _ => {
                let v: f64 = e
                    .parse()
                    .unwrap_or_else(|_| panic!("{at}: bad expected double {e}"));
                if self.ascii && v.is_finite() && v != 0.0 {
                    ((x - v) / v).abs() < 1e-15
                } else {
                    x.to_bits() == v.to_bits()
                }
            }
        };
        assert!(ok, "{}: {at}: expected {e}, got {x:?}", self.file);
    }

    fn values(&self, expected: &[Json], o: &RObject, at: &str) {
        let file = self.file;
        match &o.value {
            Value::Integer(v) | Value::Logical(v) => {
                let got: Vec<String> = v
                    .iter()
                    .take(expected.len())
                    .map(|&x| match (x, o.sexptype() == sexptype::LGL) {
                        (NA_INTEGER, _) => "NA".into(),
                        (0, true) => "FALSE".into(),
                        (_, true) => "TRUE".into(),
                        (x, false) => x.to_string(),
                    })
                    .collect();
                let want: Vec<&str> = expected.iter().map(|e| e.as_str().unwrap()).collect();
                assert_eq!(got, want, "{file}: {at}");
            }
            Value::Real(v) => {
                for (k, (e, &x)) in expected.iter().zip(v).enumerate() {
                    self.double(e, x, &format!("{at}[{k}]"));
                }
            }
            Value::Complex(v) => {
                for (k, (e, &[re, im])) in expected.iter().zip(v).enumerate() {
                    self.double(&e[0], re, &format!("{at}[{k}].re"));
                    self.double(&e[1], im, &format!("{at}[{k}].im"));
                }
            }
            Value::Raw(v) => {
                let want: Vec<u8> = expected.iter().map(|e| e.as_u64().unwrap() as u8).collect();
                assert_eq!(&v[..want.len()], want, "{file}: {at}");
            }
            Value::Character(v) => {
                let want: Vec<Option<&str>> = expected.iter().map(|e| e.as_str()).collect();
                let got: Vec<Option<&str>> =
                    v.iter().take(want.len()).map(|s| s.as_deref()).collect();
                assert_eq!(got, want, "{file}: {at}");
            }
            other => panic!("{file}: {at}: expected values, got {other:?}"),
        }
    }

    fn object(&self, expected: &Json, o: &RObject, at: &str) {
        let file = self.file;
        assert_eq!(r_type(o), expected["type"], "{file}: {at}: {o:?}");
        if let Some(n) = expected.get("length").and_then(Json::as_u64) {
            assert_eq!(o.len(), Some(n), "{file}: {at}: length");
        }
        for key in ["values", "head"] {
            if let Some(v) = expected.get(key).and_then(Json::as_array) {
                self.values(v, o, &format!("{at} {key}"));
            }
        }
        if let Some(elements) = expected.get("elements").and_then(Json::as_array) {
            let Value::List(items) = &o.value else {
                panic!("{file}: {at}: expected a list, got {o:?}")
            };
            for (k, (e, item)) in elements.iter().zip(items).enumerate() {
                self.object(e, item, &format!("{at}[{k}]"));
            }
        }
        let empty = serde_json::Map::new();
        let attrs = expected
            .get("attributes")
            .and_then(Json::as_object)
            .unwrap_or(&empty);
        let got: BTreeMap<&str, &RObject> =
            o.attributes.iter().map(|(k, v)| (k.as_str(), v)).collect();
        let want: Vec<&str> = attrs.keys().map(String::as_str).collect();
        assert_eq!(
            got.keys().copied().collect::<Vec<_>>(),
            want,
            "{file}: {at}: attribute names"
        );
        for (name, e) in attrs {
            let a = got[name.as_str()];
            // attributes() expands compact row names c(NA, -n) to 1:n.
            if let (Value::Integer(v), "row.names") = (&a.value, name.as_str())
                && v.len() == 2
                && v[0] == NA_INTEGER
            {
                let expanded = RObject::new(Value::Integer((1..=v[1].abs()).collect()));
                self.object(e, &expanded, &format!("{at}@{name}"));
                continue;
            }
            self.object(e, a, &format!("{at}@{name}"));
        }
    }
}

#[test]
fn files_written_by_r() {
    let Some(dir) = fixtures() else {
        eprintln!("RDSTREAM_R_FIXTURES not set; skipping (see tests/r/generate.R)");
        return;
    };
    let expected: Json =
        serde_json::from_str(&std::fs::read_to_string(dir.join("expected.json")).unwrap()).unwrap();
    let objects = expected["objects"].as_object().unwrap();
    let files = expected["files"].as_array().unwrap();
    assert!(files.len() >= 20, "{}", files.len());
    for f in files {
        let file = f["file"].as_str().unwrap();
        let opts = DecodeOptions {
            threads: 2,
            ..Default::default()
        };
        let p = rdstream::parse_source(Source::File(dir.join(file)), &mut Rules::all(), &opts)
            .unwrap_or_else(|e| panic!("{file}: {e}"));
        let d = &p.document;
        assert_eq!(
            d.format,
            if f["format"] == "rds" {
                Format::Rds
            } else {
                Format::RData
            },
            "{file}"
        );
        let encoding = match f["encoding"].as_str().unwrap() {
            "ascii" => Encoding::Ascii,
            "binary" => Encoding::Binary,
            _ => Encoding::Xdr,
        };
        assert_eq!(d.encoding, encoding, "{file}");
        assert_eq!(d.version as u64, f["version"].as_u64().unwrap(), "{file}");
        let layers = match f["compression"].as_str().unwrap() {
            "none" => 0,
            "gzip+gzip" => 2,
            _ => 1,
        };
        assert_eq!(p.layers.len(), layers, "{file}");
        let check = Check {
            file,
            ascii: encoding == Encoding::Ascii,
        };
        let found: BTreeMap<&str, &RObject> = match d.format {
            Format::RData => d.objects().into_iter().collect(),
            Format::Rds => {
                let names = d.root.names().unwrap();
                let Value::List(items) = &d.root.value else {
                    panic!("{file}")
                };
                names
                    .iter()
                    .map(|n| n.as_deref().unwrap())
                    .zip(items)
                    .collect()
            }
        };
        assert_eq!(found.len(), objects.len(), "{file}");
        for (name, e) in objects {
            check.object(e, found[name.as_str()], name);
        }
    }
}
