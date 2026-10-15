//! Streaming vectors to sinks and to files while parsing.

use std::cell::RefCell;
use std::io::{self, Cursor};
use std::rc::Rc;

mod common;

use common::*;
use rdstream::extract::Extractor;
use rdstream::{Chunk, Policy, Rules, Sink, Value, parse_reader};
use serde_json::Value as Json;

#[derive(Debug, Default, PartialEq)]
struct Collected {
    ints: Vec<i32>,
    reals: Vec<f64>,
    strs: Vec<Option<String>>,
    chunks: usize,
}

/// Streams the vector at `prefix` into a shared buffer.
struct Collect {
    prefix: &'static str,
    out: Rc<RefCell<Collected>>,
}

struct CollectSink(Rc<RefCell<Collected>>);

impl Sink for CollectSink {
    fn write(&mut self, chunk: Chunk<'_>) -> io::Result<()> {
        let mut c = self.0.borrow_mut();
        c.chunks += 1;
        match chunk {
            Chunk::Int(v) => c.ints.extend_from_slice(v),
            Chunk::Real(v) => c.reals.extend_from_slice(v),
            Chunk::Str(s) => c.strs.push(s.map(String::from)),
            Chunk::Raw(_) => unreachable!(),
        }
        Ok(())
    }
    fn finish(self: Box<Self>) -> io::Result<String> {
        Ok("collected".into())
    }
}

impl Policy for Collect {
    fn keep(&mut self, _: &str, _: u8, _: u64) -> bool {
        true
    }
    fn sink(&mut self, path: &str, _: u8, _: u64) -> io::Result<Option<Box<dyn Sink>>> {
        Ok((path == self.prefix).then(|| Box::new(CollectSink(self.out.clone())) as Box<dyn Sink>))
    }
}

/// list(big = as.double(0:199999), s = c("x", NA), seq = 1:70000) with names.
fn big_list() -> W {
    let mut w = W::rds();
    w.flags(VEC, false, true, false).i(3);
    w.reals(&(0..200_000).map(f64::from).collect::<Vec<_>>(), true);
    w.node("names").strs(&[Some("v")], false).nil(); // an attribute after streamed data
    w.strs(&[Some("x"), None], false);
    w.0.extend_from_slice(&altrep_compact_intseq(70_000.0).0[w_header_len()..]);
    w.node("names")
        .strs(&[Some("big"), Some("s"), Some("seq")], false)
        .nil();
    w
}

fn w_header_len() -> usize {
    W::rds().0.len()
}

fn parse_with<P: Policy + Send>(bytes: Vec<u8>, policy: &mut P) -> rdstream::Parsed {
    parse_reader(Box::new(Cursor::new(bytes)), policy).unwrap()
}

#[test]
fn sinks_receive_vectors_in_chunks() {
    for (prefix, expect_chunks) in [("[0]", 4), ("[1]", 2), ("[2]", 2)] {
        let out = Rc::new(RefCell::new(Collected::default()));
        let mut policy = Collect {
            prefix,
            out: out.clone(),
        };
        // Rc is not Send; the parser thread is avoided by parsing in place.
        let doc = rdstream::Parser::new(Cursor::new(big_list().0), &mut policy)
            .parse_document()
            .unwrap();
        let Value::List(items) = &doc.root.value else {
            panic!()
        };
        let k: usize = prefix[1..2].parse().unwrap();
        let c = out.borrow();
        assert_eq!(c.chunks, expect_chunks, "{prefix}");
        match k {
            0 => {
                assert_eq!(c.reals, (0..200_000).map(f64::from).collect::<Vec<_>>());
                assert_eq!(
                    items[0].value,
                    Value::Streamed {
                        sexptype: REAL,
                        len: 200_000,
                        target: "collected".into()
                    }
                );
                // Attributes after the data are still parsed and attached.
                assert_eq!(items[0].names().unwrap(), [Some("v".to_string())]);
            }
            1 => assert_eq!(c.strs, [Some("x".to_string()), None]),
            _ => {
                assert_eq!(c.ints, (1..=70_000).collect::<Vec<_>>());
                assert_eq!(
                    items[2].value,
                    Value::Streamed {
                        sexptype: INT,
                        len: 70_000,
                        target: "collected".into()
                    }
                );
            }
        }
        assert_eq!(doc.root.names().unwrap().len(), 3);
    }
}

fn read_npy(path: &std::path::Path) -> (String, Vec<u8>) {
    let b = std::fs::read(path).unwrap();
    assert_eq!(&b[..8], b"\x93NUMPY\x01\x00");
    let hlen = u16::from_le_bytes([b[8], b[9]]) as usize;
    assert_eq!((10 + hlen) % 64, 0);
    (
        String::from_utf8(b[10..10 + hlen].to_vec()).unwrap(),
        b[10 + hlen..].to_vec(),
    )
}

#[test]
fn extractor_streams_to_files_and_names_them_after_parsing() {
    let dir = tempfile::tempdir().unwrap();
    let mut ex = Extractor::new(&["[0]".into(), "[2]".into()], dir.path(), 0).unwrap();
    let parsed = parse_with(big_list().0, &mut ex);
    // Selected vectors were never materialized; the rest fell under the threshold.
    let Value::List(items) = &parsed.document.root.value else {
        panic!()
    };
    assert!(matches!(items[0].value, Value::Streamed { .. }));
    assert_eq!(
        items[1].value,
        Value::Skipped {
            sexptype: STR,
            len: 2
        }
    );
    let entries = ex.finish(&parsed.document).unwrap();

    let files: Vec<&str> = entries
        .iter()
        .map(|e| e["file"].as_str().unwrap())
        .collect();
    assert_eq!(
        files,
        ["0000_0__big.npy", "0001_0__names.json", "0002_2__seq.npy"]
    );
    let mut on_disk: Vec<String> = std::fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect();
    on_disk.sort();
    assert_eq!(
        on_disk,
        [
            "0000_0__big.npy",
            "0001_0__names.json",
            "0002_2__seq.npy",
            "manifest.json"
        ]
    );

    let (header, data) = read_npy(&dir.path().join(files[0]));
    assert!(
        header.contains("'descr': '<f8'") && header.contains("'shape': (200000,)"),
        "{header}"
    );
    let values: &[f64] = bytemuck::cast_slice(&data);
    assert_eq!(values, (0..200_000).map(f64::from).collect::<Vec<_>>());
    let (header, data) = read_npy(&dir.path().join(files[2]));
    assert!(header.contains("'descr': '<i4'"), "{header}");
    assert_eq!(
        bytemuck::cast_slice::<u8, i32>(&data),
        (1..=70_000).collect::<Vec<_>>()
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join(files[1])).unwrap(),
        r#"["v"]"#
    );

    let manifest: Json =
        serde_json::from_str(&std::fs::read_to_string(dir.path().join("manifest.json")).unwrap())
            .unwrap();
    assert_eq!(manifest["entries"][0]["path"], "[0]");
    assert_eq!(manifest["entries"][0]["name"], "big");
    assert_eq!(manifest["entries"][0]["len"], 200_000);
    assert_eq!(manifest["entries"][2]["type"], "int");
}

#[test]
fn extractor_output_matches_extracting_a_loaded_document() {
    let streamed = tempfile::tempdir().unwrap();
    let mut ex = Extractor::new(&["*".into()], streamed.path(), 0).unwrap();
    let parsed = parse_with(data_frame().0, &mut ex);
    let a = ex.finish(&parsed.document).unwrap();

    let loaded = tempfile::tempdir().unwrap();
    let doc = parse_with(data_frame().0, &mut Rules::all()).document;
    let b = rdstream::extract::extract(&doc, &["*".into()], loaded.path()).unwrap();
    assert_eq!(a, b);
    assert!(
        a.iter()
            .any(|e| e["levels"] == serde_json::json!(["u", "v"]))
    );
    for e in &a {
        let f = e["file"].as_str().unwrap();
        assert_eq!(
            std::fs::read(streamed.path().join(f)).unwrap(),
            std::fs::read(loaded.path().join(f)).unwrap(),
            "{f}"
        );
    }
}

#[test]
fn extractor_cleans_up_after_a_failed_parse() {
    let dir = tempfile::tempdir().unwrap();
    let mut bytes = big_list().0;
    bytes.truncate(100_000);
    let mut ex = Extractor::new(&["*".into()], dir.path(), 0).unwrap();
    assert!(parse_reader(Box::new(Cursor::new(bytes)), &mut ex).is_err());
    drop(ex);
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
}
