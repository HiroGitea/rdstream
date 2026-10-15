//! Streams are assembled byte by byte from the format in R's serialize.c, so every
//! test pins down exactly what is on the wire.

use std::io::{Cursor, Write};

mod common;

use common::*;
use rdstream::sexp::NA_REAL;
use rdstream::{
    Compression, DecodeOptions, Decoder, Encoding, Error, Format, RObject, Rules, Source, Value,
    parse_reader, parse_source,
};

fn parse(bytes: Vec<u8>, rules: &mut Rules) -> rdstream::Result<rdstream::Parsed> {
    parse_reader(Box::new(Cursor::new(bytes)), rules)
}

fn parse_all(bytes: Vec<u8>) -> rdstream::Parsed {
    parse(bytes, &mut Rules::all()).unwrap()
}

#[test]
fn integer_vector_with_na() {
    let mut w = W::rds();
    w.ints(&[1, i32::MIN, 3], false);
    let len = w.0.len() as u64;
    let p = parse_all(w.0);
    assert_eq!(p.document.format, Format::Rds);
    assert_eq!(p.document.version, 3);
    assert_eq!(p.document.writer_version, (4, 3, 1));
    assert_eq!(p.document.native_encoding.as_deref(), Some("UTF-8"));
    assert_eq!(p.document.root.value, Value::Integer(vec![1, i32::MIN, 3]));
    assert_eq!(p.document.bytes_read, len);
    assert!(p.layers.is_empty());
}

#[test]
fn strings_na_and_latin1() {
    let mut w = W::rds();
    w.flags(STR, false, false, false)
        .i(3)
        .chars(Some("héllo"))
        .chars(None);
    // latin1-flagged CHARSXP holding byte 0xE9 ('é')
    w.i(9 | (1 << 2) << 12).i(1).text(&[0xE9]);
    let p = parse_all(w.0);
    assert_eq!(
        p.document.root.value,
        Value::Character(vec![Some("héllo".into()), None, Some("é".into())])
    );
}

#[test]
fn rdata_objects_and_symbol_references() {
    // save(a, b): a = c(x = 1.5, y = 2), b = list(a = TRUE) with `names` reused via REFSXP
    let mut w = W::rdata();
    w.node("a");
    w.reals(&[1.5, 2.0], true)
        .node("names")
        .strs(&[Some("x"), Some("y")], false)
        .nil();
    // refs so far: #1 sym a, #2 sym names
    w.node("b");
    w.flags(VEC, false, true, false)
        .i(1)
        .flags(LGL, false, false, false)
        .i(1)
        .i(1);
    w.node_ref(2).strs(&[Some("a")], false).nil();
    w.nil();
    let p = parse_all(w.0);
    let objs = p.document.objects();
    assert_eq!(p.document.format, Format::RData);
    assert_eq!(objs.iter().map(|(n, _)| *n).collect::<Vec<_>>(), ["a", "b"]);
    let a = objs[0].1;
    assert_eq!(a.value, Value::Real(vec![1.5, 2.0]));
    assert_eq!(
        a.names().unwrap(),
        [Some("x".to_string()), Some("y".to_string())]
    );
    let b = objs[1].1;
    assert_eq!(b.get("a").unwrap().value, Value::Logical(vec![1]));
}

#[test]
fn long_vector_length_encoding() {
    let mut w = W::rds();
    w.flags(INT, false, false, false).i(-1).i(0).i(2).i(7).i(8);
    assert_eq!(
        parse_all(w.0).document.root.value,
        Value::Integer(vec![7, 8])
    );
}

#[test]
fn data_frame_with_factor() {
    let p = parse_all(data_frame().0);
    let df = &p.document.root;
    assert!(df.inherits("data.frame"));
    let g = df.get("g").unwrap();
    assert!(g.inherits("factor"));
    assert_eq!(g.value, Value::Integer(vec![1, 2, 1]));
    assert_eq!(
        g.attr("levels").unwrap().value,
        Value::Character(vec![Some("u".into()), Some("v".into())])
    );
}

#[test]
fn policy_skips_large_vectors_and_keeps_position() {
    // list(big = 1:1000 as doubles, small = 7L)
    let mut w = W::rds();
    w.flags(VEC, false, true, false).i(2);
    w.reals(&(0..1000).map(f64::from).collect::<Vec<_>>(), false);
    w.ints(&[7], false);
    w.node("names")
        .strs(&[Some("big"), Some("small")], false)
        .nil();
    let len = w.0.len() as u64;

    let mut seen = Vec::new();
    let mut policy = |path: &str, _t: u8, n: u64| {
        seen.push((path.to_string(), n));
        n < 100
    };
    let p = parse_reader(Box::new(Cursor::new(w.0.clone())), &mut policy).unwrap();
    let root = &p.document.root;
    assert_eq!(
        root.get("big").unwrap().value,
        Value::Skipped {
            sexptype: REAL,
            len: 1000
        }
    );
    assert_eq!(root.get("small").unwrap().value, Value::Integer(vec![7]));
    assert_eq!(p.document.bytes_read, len);
    assert_eq!(seen[0], ("[0]".to_string(), 1000));
    assert_eq!(seen[2], ("@names".to_string(), 2));

    // An explicit keep glob overrides the threshold; skip globs override everything.
    let mut rules = Rules {
        threshold: 10,
        keep: vec!["[0]".into()],
        skip: vec!["@names".into()],
    };
    let p = parse(w.0, &mut rules).unwrap();
    assert!(
        matches!(p.document.root.value, Value::List(ref v) if matches!(v[0].value, Value::Real(ref x) if x.len() == 1000))
    );
    assert!(matches!(
        p.document.root.attr("names").unwrap().value,
        Value::Skipped { .. }
    ));
}

#[test]
fn skipped_strings_are_read_past() {
    let mut w = W::rds();
    w.flags(VEC, false, false, false).i(2);
    w.strs(&[Some("aaaa"), None, Some(""), Some("bb")], false);
    w.ints(&[5], false);
    let p = parse(
        w.0,
        &mut Rules {
            threshold: 3,
            ..Default::default()
        },
    )
    .unwrap();
    match &p.document.root.value {
        Value::List(v) => {
            assert_eq!(
                v[0].value,
                Value::Skipped {
                    sexptype: STR,
                    len: 4
                }
            );
            assert_eq!(v[1].value, Value::Integer(vec![5]));
        }
        v => panic!("{v:?}"),
    }
}

#[test]
fn altrep_compact_sequence() {
    let p = parse_all(altrep_compact_intseq(5.0).0);
    assert_eq!(p.document.root.value, Value::Integer(vec![1, 2, 3, 4, 5]));
    let p = parse(
        altrep_compact_intseq(5.0).0,
        &mut Rules {
            threshold: 2,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(
        p.document.root.value,
        Value::Skipped {
            sexptype: INT,
            len: 5
        }
    );
}

#[test]
fn s4_object_with_pseudo_null_slot() {
    let mut w = W::rds();
    w.flags(S4, true, true, false);
    w.node("x").ints(&[1], false);
    w.node("misc").sym("\u{1}NULL\u{1}");
    w.node("class")
        .strs(&[Some("Thing")], true)
        .node("package")
        .strs(&[Some("pkg")], false)
        .nil();
    w.nil();
    let p = parse_all(w.0);
    let o = &p.document.root;
    assert_eq!(o.value, Value::S4);
    assert!(o.inherits("Thing"));
    assert_eq!(o.attr("misc").unwrap(), &RObject::null());
}

#[test]
fn environment_referencing_itself() {
    // e <- new.env(); e$self <- e
    let mut w = W::rds();
    w.i(ENV as i32).i(0); // unlocked
    w.i(253); // enclos = globalenv
    w.node("self").reference(1).nil(); // frame
    w.nil(); // hashtab
    w.nil(); // attributes
    let p = parse_all(w.0);
    assert_eq!(p.document.root.value, Value::Environment(0));
    let env = &p.document.environments[0];
    assert_eq!(env.frame.get("self").unwrap().value, Value::Environment(0));
}

fn compress(c: Compression, data: &[u8]) -> Vec<u8> {
    match c {
        Compression::Gzip => {
            let mut e = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
            e.write_all(data).unwrap();
            e.finish().unwrap()
        }
        Compression::Bzip2 => {
            let mut e = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::default());
            e.write_all(data).unwrap();
            e.finish().unwrap()
        }
        Compression::Xz => {
            let mut e = liblzma::write::XzEncoder::new(Vec::new(), 6);
            e.write_all(data).unwrap();
            e.finish().unwrap()
        }
        Compression::Zstd => zstd::encode_all(data, 3).unwrap(),
    }
}

#[test]
fn nested_compression_layers() {
    let raw = data_frame().0;
    for layers in [
        vec![Compression::Gzip],
        vec![Compression::Gzip, Compression::Gzip],
        vec![Compression::Xz, Compression::Bzip2, Compression::Gzip],
        vec![Compression::Zstd],
        vec![Compression::Gzip, Compression::Zstd],
        vec![Compression::Zstd, Compression::Xz],
    ] {
        let mut data = raw.clone();
        for &c in layers.iter().rev() {
            data = compress(c, &data);
        }
        let p = parse_all(data.clone());
        assert_eq!(
            p.layers.iter().map(|l| l.compression).collect::<Vec<_>>(),
            layers
        );
        // Same result through the single-threaded and the parallel stream decoders.
        for threads in [1, 4] {
            let opts = DecodeOptions {
                threads,
                ..Default::default()
            };
            let q = parse_source(
                Source::Stream(Box::new(Cursor::new(data.clone()))),
                &mut Rules::all(),
                &opts,
            )
            .unwrap();
            assert_eq!(q.document.root, p.document.root);
        }
        assert!(p.document.root.inherits("data.frame"));
        assert_eq!(p.document.bytes_read, raw.len() as u64);
    }
}

#[test]
fn zstd_frames_and_skippable_frames() {
    // `cat a.zst b.zst` gives several frames; zstd tools may put a skippable
    // frame first.
    let raw = data_frame().0;
    let (a, b) = raw.split_at(raw.len() / 2);
    let mut data = vec![0x50, 0x2a, 0x4d, 0x18, 3, 0, 0, 0, b'r', b'd', b's'];
    data.extend(zstd::encode_all(a, 1).unwrap());
    data.extend(zstd::encode_all(b, 19).unwrap());
    let p = parse_all(data);
    assert_eq!(
        p.layers.iter().map(|l| l.compression).collect::<Vec<_>>(),
        [Compression::Zstd]
    );
    assert!(p.document.root.inherits("data.frame"));
    assert_eq!(p.document.bytes_read, raw.len() as u64);

    // A long-distance window (`zstd --long=31`) needs the decoder's limit raised.
    let mut e = zstd::stream::write::Encoder::new(Vec::new(), 3).unwrap();
    e.long_distance_matching(true).unwrap();
    e.window_log(31).unwrap();
    e.write_all(&raw).unwrap();
    assert!(
        parse_all(e.finish().unwrap())
            .document
            .root
            .inherits("data.frame")
    );

    let mut corrupt = zstd::encode_all(raw.as_slice(), 3).unwrap();
    let n = corrupt.len();
    corrupt[n / 2] ^= 0xff;
    assert!(matches!(
        parse(corrupt, &mut Rules::all()),
        Err(Error::Io(_) | Error::UnexpectedEof { .. })
    ));
}

#[test]
fn truncated_stream_reports_offset() {
    let mut w = W::rds();
    w.reals(&[1.0, 2.0, 3.0], false);
    let mut bytes = w.0;
    bytes.truncate(bytes.len() - 5);
    match parse(bytes, &mut Rules::all()) {
        Err(Error::UnexpectedEof { .. }) => {}
        other => panic!("expected EOF error, got {other:?}"),
    }
}

#[test]
fn rejects_non_r_input() {
    assert!(matches!(
        parse(b"hello world".to_vec(), &mut Rules::all()),
        Err(Error::NotSerialized(_))
    ));
}

/// The same object in every encoding R can write.
fn kitchen_sink(enc: Enc) -> W {
    // list(n = c(1L, NA), x = c(-0.1, NA, NaN, -Inf, 1e300, 5e-324), s = c("a b\n\\", NA, "é"),
    //      r = as.raw(c(0, 255)), z = complex(real = 1, imaginary = -2.5), f = c(a = TRUE))
    let mut w = W::rds_in(enc);
    w.flags(VEC, false, true, false).i(6);
    w.ints(&[1, i32::MIN], false);
    w.reals(
        &[-0.1, NA_REAL, f64::NAN, f64::NEG_INFINITY, 1e300, 5e-324],
        false,
    );
    w.strs(&[Some("a b\n\\"), None, Some("é")], false);
    w.flags(RAW, false, false, false).i(2).byte(0).byte(255);
    w.flags(CPLX, false, false, false).i(1).real(1.0).real(-2.5);
    w.flags(LGL, false, true, false).i(1).i(1);
    w.node("names").strs(&[Some("a")], false).nil();
    w.node_ref(1)
        .strs(
            &[
                Some("n"),
                Some("x"),
                Some("s"),
                Some("r"),
                Some("z"),
                Some("f"),
            ],
            false,
        )
        .nil();
    w
}

#[test]
fn ascii_and_native_binary_match_xdr() {
    let xdr = parse_all(kitchen_sink(Enc::Xdr).0);
    assert_eq!(xdr.document.encoding, Encoding::Xdr);
    for (enc, expected) in [
        (Enc::Ascii, Encoding::Ascii),
        (Enc::Binary, Encoding::Binary),
    ] {
        let p = parse_all(kitchen_sink(enc).0);
        assert_eq!(p.document.encoding, expected);
        assert_eq!(p.document.native_encoding.as_deref(), Some("UTF-8"));
        let (a, b) = (
            format!("{:?}", p.document.root),
            format!("{:?}", xdr.document.root),
        );
        assert_eq!(a, b); // Debug, because NaN != NaN
    }
    let root = &xdr.document.root;
    let Value::Real(x) = &root.get("x").unwrap().value else {
        panic!()
    };
    assert_eq!(x[1].to_bits(), NA_REAL.to_bits());
    assert!(x[2].is_nan() && x[2].to_bits() != NA_REAL.to_bits());
    assert_eq!(
        root.get("s").unwrap().value,
        Value::Character(vec![Some("a b\n\\".into()), None, Some("é".into())])
    );
    assert_eq!(
        root.get("z").unwrap().value,
        Value::Complex(vec![[1.0, -2.5]])
    );
}

#[test]
fn ascii_skips_and_hex_doubles() {
    // save(x, ascii = NA) writes doubles as C99 hex floats; RData header "RDA3\n".
    let mut w = W(b"RDA3\nA\n".to_vec(), Enc::Ascii);
    w.i(3).i(0x040301).i(0x030500).i(5).text(b"UTF-8");
    w.node("x").flags(REAL, false, false, false).i(3);
    w.0.extend_from_slice(b"0x1.8p+1\n-0x1.999999999999ap-4\nNA\n");
    w.node("s").strs(&[Some("skip me"), Some("")], false);
    w.node("r")
        .flags(RAW, false, false, false)
        .i(2)
        .byte(0xab)
        .byte(0x01);
    w.nil();
    let p = parse_all(w.0.clone());
    assert_eq!(p.document.format, Format::RData);
    assert_eq!(p.document.encoding, Encoding::Ascii);
    let objs = p.document.objects();
    let Value::Real(x) = &objs[0].1.value else {
        panic!()
    };
    assert_eq!((x[0], x[1], x[2].to_bits()), (3.0, -0.1, NA_REAL.to_bits()));
    assert_eq!(objs[2].1.value, Value::Raw(vec![0xab, 0x01]));

    let p = parse(
        w.0,
        &mut Rules {
            threshold: 0,
            ..Default::default()
        },
    )
    .unwrap();
    let objs = p.document.objects();
    assert_eq!(
        objs[1].1.value,
        Value::Skipped {
            sexptype: STR,
            len: 2
        }
    );
    assert_eq!(
        objs[2].1.value,
        Value::Skipped {
            sexptype: RAW,
            len: 2
        }
    );
}

#[test]
fn ascii_reports_bad_numbers_with_path() {
    let mut w = W::rds_in(Enc::Ascii);
    w.flags(VEC, false, false, false)
        .i(1)
        .flags(INT, false, false, false)
        .i(1);
    w.0.extend_from_slice(b"12x\n");
    match parse(w.0, &mut Rules::all()) {
        Err(Error::Malformed { path, msg, .. }) => {
            assert_eq!(path, "[0]");
            assert!(msg.contains("12x"), "{msg}");
        }
        other => panic!("{other:?}"),
    }
}

fn gzip(level: u32, data: &[u8]) -> Vec<u8> {
    let mut e = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::new(level));
    e.write_all(data).unwrap();
    e.finish().unwrap()
}

/// A gzip-in-gzip file around `list(<raw[n] payload>, 42L)`, larger than the
/// window the parallel inner decoder reads it through.
fn check_recompressed(payload: Vec<u8>, outer_level: u32, outer: Decoder) {
    let mut w = W::rds();
    w.flags(VEC, false, false, false).i(2);
    w.flags(RAW, false, false, false).i(payload.len() as i32);
    w.0.extend(payload);
    w.ints(&[42], false);
    let raw = w.0;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("double.rds.gz");
    std::fs::write(&path, gzip(outer_level, &gzip(1, &raw))).unwrap();

    for threads in [1, 4] {
        let opts = DecodeOptions {
            threads,
            ..Default::default()
        };
        let p = parse_source(
            Source::File(path.clone()),
            &mut Rules {
                threshold: 10,
                ..Default::default()
            },
            &opts,
        )
        .unwrap();
        let decoders: Vec<_> = p.layers.iter().map(|l| l.decoder).collect();
        if threads > 1 && cfg!(feature = "parallel-gzip") {
            assert_eq!(decoders, [outer, Decoder::ParallelWindow]);
        } else {
            assert_eq!(decoders, [Decoder::Serial, Decoder::Serial]);
        }
        assert_eq!(p.document.bytes_read, raw.len() as u64);
        let Value::List(items) = &p.document.root.value else {
            panic!()
        };
        assert_eq!(items[1].value, Value::Integer(vec![42]));
    }
    // Everything the parser does not read is still decoded and checked.
    let opts = DecodeOptions {
        threads: 4,
        ..Default::default()
    };
    let (mut inner, _) =
        rdstream::decompress::decompress_layers(Source::File(path), &opts).unwrap();
    let mut out = Vec::new();
    std::io::Read::read_to_end(&mut inner, &mut out).unwrap();
    assert!(out == raw);
}

fn xorshift(n: usize, mask: u8) -> Vec<u8> {
    let mut x: u64 = 0x9E3779B97F4A7C15;
    (0..n)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x as u8 & mask
        })
        .collect()
}

#[test]
fn recompressed_file_with_dynamic_huffman_outer_layer() {
    // rusty-rapidgzip decodes zlib's dynamic blocks of compressed data in parallel.
    check_recompressed(xorshift(48 << 20, 0x0f), 6, Decoder::ParallelMapped);
}

#[test]
fn recompressed_file_falls_back_where_rusty_rapidgzip_is_slow() {
    // Level 1 codes the inner layer's bytes as fixed-Huffman literals and
    // stores random bytes; both are slow paths for rusty-rapidgzip.
    check_recompressed(xorshift(48 << 20, 0x0f), 1, Decoder::ParallelFile);
    check_recompressed(xorshift(40 << 20, 0xff), 1, Decoder::ParallelFile);
}

/// `list(<raw payload>, 42L)` gzipped `levels` times (outermost last).
fn gzipped_list(payload: Vec<u8>, levels: &[u32]) -> (Vec<u8>, Vec<u8>) {
    let mut w = W::rds();
    w.flags(VEC, false, false, false).i(2);
    w.flags(RAW, false, false, false).i(payload.len() as i32);
    w.0.extend(payload);
    w.ints(&[42], false);
    let data = levels.iter().fold(w.0.clone(), |d, &l| gzip(l, &d));
    (w.0, data)
}

fn parse_stream(data: Vec<u8>, opts: &DecodeOptions) -> rdstream::Parsed {
    let source = Source::Stream(Box::new(Cursor::new(data)));
    parse_source(
        source,
        &mut Rules {
            threshold: 10,
            ..Default::default()
        },
        opts,
    )
    .unwrap()
}

#[test]
fn streams_decode_on_one_thread_per_layer() {
    // rusty-rapidgzip's streaming pipeline mistook bytes of a recompressed
    // layer for a block start, and on stored or fixed-Huffman blocks it took
    // minutes; streams now use zlib-rs, one pipelined thread per layer.
    for (payload, levels) in [
        (xorshift(64 << 20, 0x0f), vec![6, 6]),
        (xorshift(40 << 20, 0xff), vec![1]),
    ] {
        let (raw, data) = gzipped_list(payload, &levels);
        let t = std::time::Instant::now();
        let p = parse_stream(
            data,
            &DecodeOptions {
                threads: 4,
                ..Default::default()
            },
        );
        assert!(t.elapsed().as_secs() < 30, "{:?}", t.elapsed());
        assert!(
            p.layers.iter().all(|l| l.decoder == Decoder::Serial),
            "{:?}",
            p.layers
        );
        assert_eq!(p.layers.len(), levels.len());
        assert_eq!(p.document.bytes_read, raw.len() as u64);
        let Value::List(items) = &p.document.root.value else {
            panic!()
        };
        assert_eq!(items[1].value, Value::Integer(vec![42]));
    }
}

#[test]
fn spooled_streams_decode_like_files() {
    let (raw, data) = gzipped_list(xorshift(48 << 20, 0x0f), &[1, 6]);
    let dir = tempfile::tempdir().unwrap();
    let opts = DecodeOptions {
        threads: 4,
        spool_dir: Some(dir.path().to_path_buf()),
        ..Default::default()
    };
    let (mut inner, layers) = rdstream::decompress::decompress_layers(
        Source::Stream(Box::new(Cursor::new(data.clone()))),
        &opts,
    )
    .unwrap();
    if cfg!(feature = "parallel-gzip") {
        assert_eq!(
            layers.iter().map(|l| l.decoder).collect::<Vec<_>>(),
            [Decoder::ParallelMapped, Decoder::ParallelWindow]
        );
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 1); // the spool file, while in use
    }
    let mut out = Vec::new();
    std::io::Read::read_to_end(&mut inner, &mut out).unwrap();
    assert!(out == raw);
    drop(inner);
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);

    let p = parse_stream(data, &opts);
    let Value::List(items) = &p.document.root.value else {
        panic!()
    };
    assert_eq!(items[1].value, Value::Integer(vec![42]));
    assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);

    // A missing spool directory is an error rather than a silent fallback.
    if cfg!(feature = "parallel-gzip") {
        let opts = DecodeOptions {
            threads: 4,
            spool_dir: Some(dir.path().join("missing")),
            ..Default::default()
        };
        let source = Source::Stream(Box::new(Cursor::new(vec![0x1f, 0x8b])));
        let missing = rdstream::decompress::decompress_layers(source, &opts);
        assert_eq!(
            missing.err().map(|e| e.kind()),
            Some(std::io::ErrorKind::NotFound)
        );
    }
}
