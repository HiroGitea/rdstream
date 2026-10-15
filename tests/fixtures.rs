//! Files written by R itself, collected from open-source projects; provenance,
//! licenses and the R code behind each file are in tests/fixtures/MANIFEST.md.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rdstream::sexp::{NA_INTEGER, NA_REAL};
use rdstream::{DecodeOptions, Encoding, Format, Parsed, RObject, Rules, Source, Value};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

fn data_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            out.extend(data_files(&path));
        } else if matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("rds" | "rda" | "RData")
        ) {
            out.push(path);
        }
    }
    out.sort();
    out
}

fn parse(path: &Path) -> rdstream::Result<Parsed> {
    let opts = DecodeOptions {
        threads: 1,
        ..Default::default()
    };
    rdstream::parse_source(Source::File(path.to_path_buf()), &mut Rules::all(), &opts)
}

fn load(rel: &str) -> Parsed {
    parse(&fixtures().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

/// The object saved in a fixture: the root of an `.rds`, or the object named
/// `name` in an `.rda`.
fn object<'a>(p: &'a Parsed, name: &str) -> &'a RObject {
    match p.document.format {
        Format::Rds => &p.document.root,
        Format::RData => {
            p.document
                .objects()
                .into_iter()
                .find(|(n, _)| *n == name)
                .unwrap()
                .1
        }
    }
}

/// `o` with every double rounded to 15 significant digits (NaN payloads kept).
fn round_doubles(o: &RObject) -> RObject {
    let r = |x: f64| {
        if x.is_finite() {
            format!("{x:.14e}").parse().unwrap()
        } else {
            x
        }
    };
    let value = match &o.value {
        Value::Real(v) => Value::Real(v.iter().map(|&x| r(x)).collect()),
        Value::Complex(v) => Value::Complex(v.iter().map(|&[a, b]| [r(a), r(b)]).collect()),
        Value::List(v) => Value::List(v.iter().map(round_doubles).collect()),
        Value::Expression(v) => Value::Expression(v.iter().map(round_doubles).collect()),
        Value::Pairlist { sexptype, items } => Value::Pairlist {
            sexptype: *sexptype,
            items: items
                .iter()
                .map(|(k, v)| (k.clone(), round_doubles(v)))
                .collect(),
        },
        v => v.clone(),
    };
    let attributes = o
        .attributes
        .iter()
        .map(|(k, v)| (k.clone(), round_doubles(v)))
        .collect();
    RObject {
        value,
        attributes,
        is_object: o.is_object,
    }
}

fn chr(v: &[&str]) -> Value {
    Value::Character(v.iter().map(|s| Some(s.to_string())).collect())
}

#[test]
fn every_fixture_parses() {
    let mut unexpected = Vec::new();
    let mut ok = 0;
    for path in data_files(&fixtures()) {
        match parse(&path) {
            Ok(p) => {
                ok += 1;
                assert!(matches!(p.document.version, 2 | 3), "{}", path.display());
            }
            Err(e) => unexpected.push(format!("{}: {e}", path.display())),
        }
    }
    assert!(unexpected.is_empty(), "{}", unexpected.join("\n"));
    assert_eq!(ok, 796);
}

/// Every object of rdata's format matrix (`<test>__{ascii,xdr,binary}__version_{2,3}.{rds,rda}`)
/// parses to the same tree from every variant.
#[test]
fn rdata_variants_agree_across_encodings_versions_and_containers() {
    let mut groups: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();
    for path in data_files(&fixtures().join("rdata/generated")) {
        let name = path.file_name().unwrap().to_str().unwrap();
        groups
            .entry(name.split("__").next().unwrap().to_string())
            .or_default()
            .push(path);
    }
    assert_eq!(groups.len(), 58);
    let mut compared = 0;
    for (name, files) in &groups {
        let parsed: Vec<_> = files.iter().map(|f| (f, parse(f))).collect();
        let mut reference: Option<(String, &PathBuf)> = None;
        for (file, p) in &parsed {
            let p = p.as_ref().unwrap();
            let fname = file.file_name().unwrap().to_str().unwrap();
            let expected = if fname.contains("__ascii__") {
                Encoding::Ascii
            } else if fname.contains("__binary__") {
                Encoding::Binary
            } else {
                Encoding::Xdr
            };
            assert_eq!(p.document.encoding, expected, "{fname}");
            // R writes ASCII doubles with 16 significant digits, so compare 15.
            // Debug, because NaN != NaN.
            let tree = format!("{:?}", round_doubles(object(p, name)));
            match &reference {
                None => reference = Some((tree, file)),
                Some((r, rfile)) => {
                    assert_eq!(&tree, r, "{} vs {}", file.display(), rfile.display())
                }
            }
            compared += 1;
        }
    }
    assert_eq!(compared, 580);
}

#[test]
fn altrep_objects_expand_like_r() {
    for (version, ext) in [(2, "rds"), (3, "rds"), (3, "rda")] {
        let get = |name: &str| {
            let p = load(&format!(
                "rdata/generated/{name}__xdr__version_{version}.{ext}"
            ));
            object(&p, name).clone()
        };
        assert_eq!(
            get("test_altrep_compact_intseq").value,
            Value::Integer((0..1000).collect())
        );
        assert_eq!(
            get("test_altrep_compact_intseq_asymmetric").value,
            Value::Integer((-5..=5).collect())
        );
        assert_eq!(
            get("test_altrep_deferred_string").value,
            chr(&[
                "1", "2.3", "10000", "1e+05", "-10000", "-1e+05", "0.001", "1e-04", "1e-05"
            ])
        );
        let wrapped = get("test_altrep_wrap_real_attributes");
        assert_eq!(wrapped.value, Value::Real(vec![1.0, 2.0, 3.0]));
        assert_eq!(wrapped.attr("foo").unwrap().value, chr(&["bar"]));
        let dated = get("test_altrep_wrap_real_class_attribute");
        assert!(dated.inherits("Date") && dated.is_object);
        assert_eq!(get("test_altrep_wrap_string").value, chr(&["Hello"]));
    }
}

#[test]
fn ascii_files_including_windows_line_endings() {
    // list(1.1, 2L, 3+4i, NA, "aä"), saved on Linux and on Windows (CRLF, CP1252).
    for tag in ["", "win_"] {
        for v in [2, 3] {
            for ext in ["rds", "rda"] {
                let p = load(&format!("rdata/legacy-1.0.0/test_ascii_{tag}v{v}.{ext}"));
                assert_eq!(p.document.encoding, Encoding::Ascii);
                let Value::List(items) = &object(&p, "data").value else {
                    panic!()
                };
                assert_eq!(items[0].value, Value::Real(vec![1.1]));
                assert_eq!(items[1].value, Value::Integer(vec![2]));
                assert_eq!(items[2].value, Value::Complex(vec![[3.0, 4.0]]));
                assert_eq!(items[3].value, Value::Logical(vec![NA_INTEGER]));
                assert_eq!(items[4].value, chr(&["aä"]));
            }
        }
    }
}

#[test]
fn string_encodings() {
    let p = load("rdata/legacy-1.0.0/test_encodings_v3.rda");
    assert_eq!(p.document.native_encoding.as_deref(), Some("CP1252"));
    let get = |n: &str| object(&p, n).value.clone();
    assert_eq!(get("test_encoding_utf8"), chr(&["eĥoŝanĝo ĉiuĵaŭde"]));
    assert_eq!(get("test_encoding_latin1"), chr(&["cañón"]));
    // Unflagged strings are in the writer's native encoding.
    assert_eq!(get("test_encoding_latin1_implicit"), chr(&["Íñigo"]));
}

#[test]
fn special_doubles() {
    // c(0, -0, NaN, Inf, -Inf)
    for enc in ["ascii", "xdr", "binary"] {
        let p = load(&format!(
            "rdata/generated/test_nan_inf__{enc}__version_3.rds"
        ));
        let Value::Real(v) = &p.document.root.value else {
            panic!()
        };
        assert_eq!(v[0].to_bits(), 0f64.to_bits(), "{enc}");
        assert_eq!(v[1].to_bits(), (-0f64).to_bits(), "{enc}");
        assert!(
            v[2].is_nan() && v[2].to_bits() != NA_REAL.to_bits(),
            "{enc}"
        );
        assert_eq!(&v[3..], [f64::INFINITY, f64::NEG_INFINITY], "{enc}");
    }
    // NA_real_ stays distinct from NaN in every encoding.
    for enc in ["ascii", "xdr", "binary"] {
        let p = load(&format!(
            "rdata/generated/test_dataframe_float_with_na_nan__{enc}__version_3.rds"
        ));
        let mut bits = Vec::new();
        rdstream::extract::walk(&p.document, &mut |_, _, o| {
            if let Value::Real(v) = &o.value {
                bits.extend(
                    v.iter()
                        .filter(|x| x.is_nan())
                        .map(|x| x.to_bits() == NA_REAL.to_bits()),
                );
            }
            false
        });
        assert!(
            bits.contains(&true) && bits.contains(&false),
            "{enc}: {bits:?}"
        );
    }
}

#[test]
fn compression_variants_hold_the_same_objects() {
    let dir = "rdatajl/data_v3";
    let reference = format!(
        "{:?}",
        load(&format!("{dir}/compressed_false.rda")).document.root
    );
    for c in ["gzip", "bzip2", "xz"] {
        let p = load(&format!("{dir}/compressed_{c}.rda"));
        assert_eq!(p.layers.len(), 1, "{c}");
        assert_eq!(format!("{:?}", p.document.root), reference, "{c}");
    }
    // zurds: one named list in every encoding, version and compression R offers,
    // including zstd through R 4.5's zstdfile().
    let reference = format!(
        "{:?}",
        load("zurds/named_list-xdr-v3-none.rds").document.root
    );
    let mut n = 0;
    for path in data_files(&fixtures().join("zurds")) {
        let name = path.file_name().unwrap().to_str().unwrap();
        if !name.starts_with("named_list-") {
            continue;
        }
        let p = parse(&path).unwrap();
        assert_eq!(format!("{:?}", p.document.root), reference, "{name}");
        n += 1;
    }
    assert_eq!(n, 30);
    let p = load("zurds/multi-xdr-v3-zstd.rda");
    assert_eq!(p.layers[0].compression, rdstream::Compression::Zstd);
    assert_eq!(
        format!("{:?}", p.document.root),
        format!("{:?}", load("zurds/multi-xdr-v3-none.rda").document.root)
    );
}

#[test]
fn old_writers_and_data_frames() {
    let p = load("rdata/generated/test_dataframe__xdr__version_2.rds");
    let df = &p.document.root;
    assert!(df.inherits("data.frame"));
    for path in data_files(&fixtures().join("rdata/legacy-1.0.0")) {
        if let Ok(p) = parse(&path) {
            assert!(p.document.writer_version >= (3, 0, 0), "{}", path.display());
        }
    }
}

#[test]
fn seurat_object_structure() {
    let p = load("seurat/pbmc_small.rda");
    let (name, seurat) = p.document.objects()[0];
    assert_eq!(name, "pbmc_small");
    assert!(seurat.inherits("Seurat"));
    let counts = seurat
        .attr("assays")
        .unwrap()
        .get("RNA")
        .unwrap()
        .attr("counts")
        .unwrap();
    assert!(counts.inherits("dgCMatrix"));
    assert_eq!(
        counts.attr("Dim").unwrap().value,
        Value::Integer(vec![230, 80])
    );
    assert_eq!(counts.attr("x").unwrap().len(), Some(4456));
}

#[test]
fn streaming_extraction_from_r_files() {
    // A Seurat object's counts streamed to disk, matching what loading gives.
    let dir = tempfile::tempdir().unwrap();
    let mut ex = rdstream::extract::Extractor::new(
        &["$pbmc_small@assays[0]@counts@x".into()],
        dir.path(),
        0,
    )
    .unwrap();
    let opts = DecodeOptions {
        threads: 1,
        ..Default::default()
    };
    let parsed = rdstream::parse_source(
        Source::File(fixtures().join("seurat/pbmc_small.rda")),
        &mut ex,
        &opts,
    )
    .unwrap();
    let entries = ex.finish(&parsed.document).unwrap();
    assert_eq!(entries.len(), 1);
    let bytes = std::fs::read(dir.path().join(entries[0]["file"].as_str().unwrap())).unwrap();
    let loaded = load("seurat/pbmc_small.rda");
    let x = loaded.document.objects()[0]
        .1
        .attr("assays")
        .unwrap()
        .get("RNA")
        .unwrap()
        .attr("counts")
        .unwrap()
        .attr("x")
        .unwrap();
    let Value::Real(x) = &x.value else { panic!() };
    assert_eq!(
        &bytes[bytes.len() - x.len() * 8..],
        bytemuck::cast_slice::<f64, u8>(x)
    );
}

#[test]
fn byte_compiled_closures() {
    // A compiled closure in a Seurat object's command history.
    let p = load("seurat/scRep_example.rda");
    let mut bytecode = 0;
    rdstream::extract::walk(&p.document, &mut |_, _, o| {
        if let Value::Bytecode { code, consts } = &o.value {
            assert!(matches!(code.value, Value::Integer(_)));
            assert!(!consts.is_empty());
            bytecode += 1;
        }
        false
    });
    assert!(bytecode > 0);
    let p = load("zurds/bytecode.rds");
    let Value::Pairlist { items, .. } = &p.document.root.value else {
        panic!()
    };
    assert!(
        items
            .iter()
            .any(|(_, o)| matches!(o.value, Value::Bytecode { .. }))
    );
}
