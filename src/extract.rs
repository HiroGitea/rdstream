//! Write selected parts of a document to disk: numeric vectors as `.npy`,
//! character vectors as `.json`, plus a `manifest.json` describing every file.
//!
//! [`Extractor`] is a [`Policy`]: selected vectors are written chunk by chunk
//! while the stream is parsed, so they are never held in memory. Afterwards
//! [`Extractor::finish`] names each file after its place in the tree (list
//! element names are serialized after the elements) and writes the manifest.
//!
//! A pattern selects every path it matches plus everything below such a path,
//! so `$obj@meta.data` and `$obj@meta.data*` both select the whole data frame.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

use serde_json::{Value as Json, json};

use crate::policy::{Chunk, Policy, Sink, glob_match};
use crate::sexp::{Document, RObject, Value, sexptype, type_name};

/// Visit every node with the same paths the parser uses. `f` gets the path, the
/// node's name within its parent (list element name, slot or tag), and the node;
/// it returns `true` to stop descending into that node.
pub fn walk(doc: &Document, f: &mut impl FnMut(&str, Option<&str>, &RObject) -> bool) {
    let mut path = String::new();
    match &doc.root.value {
        Value::Pairlist { items, .. } if doc.format == crate::Format::RData => {
            for (name, obj) in items {
                path.clear();
                path.push('$');
                path.push_str(name.as_deref().unwrap_or(""));
                walk_node(obj, name.as_deref(), &mut path, f);
            }
        }
        _ => walk_node(&doc.root, None, &mut path, f),
    }
}

fn walk_node(
    o: &RObject,
    name: Option<&str>,
    path: &mut String,
    f: &mut impl FnMut(&str, Option<&str>, &RObject) -> bool,
) {
    if f(path, name, o) {
        return;
    }
    let mark = path.len();
    for (k, a) in &o.attributes {
        path.push('@');
        path.push_str(k);
        walk_node(a, Some(k), path, f);
        path.truncate(mark);
    }
    match &o.value {
        Value::List(items) | Value::Expression(items) => {
            let names = o.names();
            for (k, el) in items.iter().enumerate() {
                path.push_str(&format!("[{k}]"));
                let name = names.and_then(|ns| ns.get(k)).and_then(|n| n.as_deref());
                walk_node(el, name, path, f);
                path.truncate(mark);
            }
        }
        Value::Pairlist { items, .. } => {
            for (k, (tag, el)) in items.iter().enumerate() {
                match tag {
                    Some(n) if !n.starts_with('<') => path.push_str(&format!("${n}")),
                    _ => path.push_str(&format!("[{k}]")),
                }
                walk_node(el, tag.as_deref(), path, f);
                path.truncate(mark);
            }
        }
        _ => {}
    }
}

/// `.npy` type of a vector, or `None` for character vectors (written as JSON).
fn npy_descr(sexptype: u8) -> Option<Option<&'static str>> {
    const LE: bool = cfg!(target_endian = "little");
    Some(Some(match sexptype {
        sexptype::LGL | sexptype::INT => {
            if LE {
                "<i4"
            } else {
                ">i4"
            }
        }
        sexptype::REAL => {
            if LE {
                "<f8"
            } else {
                ">f8"
            }
        }
        sexptype::CPLX => {
            if LE {
                "<c16"
            } else {
                ">c16"
            }
        }
        sexptype::RAW => "|u1",
        sexptype::STR => return Some(None),
        _ => return None,
    }))
}

fn npy_header(descr: &str, len: u64) -> Vec<u8> {
    let dict = format!("{{'descr': '{descr}', 'fortran_order': False, 'shape': ({len},), }}");
    let unpadded = 10 + dict.len() + 1;
    let pad = (64 - unpadded % 64) % 64;
    let mut h = b"\x93NUMPY\x01\x00".to_vec();
    h.extend_from_slice(&((dict.len() + pad + 1) as u16).to_le_bytes());
    h.extend_from_slice(dict.as_bytes());
    h.extend(std::iter::repeat_n(b' ', pad));
    h.push(b'\n');
    h
}

/// Writes one vector to a file as its chunks arrive.
struct FileSink {
    w: BufWriter<File>,
    json: bool,
    first: bool,
    label: String,
}

impl FileSink {
    fn create(file: &Path, label: String, descr: Option<&str>, len: u64) -> io::Result<Self> {
        let mut w = BufWriter::with_capacity(1 << 20, File::create(file)?);
        match descr {
            Some(d) => w.write_all(&npy_header(d, len))?,
            None => w.write_all(b"[")?,
        }
        Ok(FileSink {
            w,
            json: descr.is_none(),
            first: true,
            label,
        })
    }

    fn write_all_value(mut self, value: &Value) -> io::Result<()> {
        match value {
            Value::Integer(v) | Value::Logical(v) => self.write(Chunk::Int(v))?,
            Value::Real(v) => self.write(Chunk::Real(v))?,
            Value::Complex(v) => self.write(Chunk::Real(v.as_flattened()))?,
            Value::Raw(v) => self.write(Chunk::Raw(v))?,
            Value::Character(v) => v
                .iter()
                .try_for_each(|s| self.write(Chunk::Str(s.as_deref())))?,
            _ => unreachable!("not a vector"),
        }
        Box::new(self).finish().map(drop)
    }
}

impl Sink for FileSink {
    fn write(&mut self, chunk: Chunk<'_>) -> io::Result<()> {
        match chunk {
            Chunk::Int(v) => self.w.write_all(bytemuck::cast_slice(v)),
            Chunk::Real(v) => self.w.write_all(bytemuck::cast_slice(v)),
            Chunk::Raw(v) => self.w.write_all(v),
            Chunk::Str(s) => {
                if !std::mem::take(&mut self.first) {
                    self.w.write_all(b",")?;
                }
                serde_json::to_writer(&mut self.w, &s).map_err(io::Error::other)
            }
        }
    }

    fn finish(mut self: Box<Self>) -> io::Result<String> {
        if self.json {
            self.w.write_all(b"]")?;
        }
        self.w.flush()?;
        Ok(self.label)
    }
}

fn sanitize(path: &str) -> String {
    let s: String = path
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect();
    s.trim_matches('_').chars().take(120).collect()
}

/// Attributes the manifest and file names are built from. The extractor keeps
/// them in memory (up to this many elements unless selected) and never streams them.
const METADATA: [&str; 4] = ["@names", "@class", "@dim", "@levels"];
const METADATA_LIMIT: u64 = 1 << 16;

fn is_metadata(path: &str) -> bool {
    METADATA.iter().any(|a| path.ends_with(a))
}

/// Whether `pattern` matches `path` or one of its ancestors.
fn selects(pattern: &str, path: &str) -> bool {
    glob_match(pattern, path)
        || path
            .char_indices()
            .any(|(i, c)| i > 0 && matches!(c, '@' | '[' | '$') && glob_match(pattern, &path[..i]))
}

/// A file written during parsing, under a temporary name until [`Extractor::finish`].
struct Streamed {
    temp: String,
    path: String,
    sexptype: u8,
    len: u64,
    claimed: bool,
}

/// [`Policy`] that streams vectors under `patterns` into files in `out_dir` and
/// loads other vectors up to `threshold` elements. The `names`, `class`, `dim`
/// and `levels` attributes are loaded rather than streamed, since the manifest
/// is built from them.
///
/// ```no_run
/// let mut ex = rdstream::extract::Extractor::new(&["*@meta.data".into()], "meta", 0)?;
/// let parsed = rdstream::parse_file("seurat.rds", &mut ex)?;
/// let entries = ex.finish(&parsed.document)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Extractor {
    patterns: Vec<String>,
    threshold: u64,
    out_dir: PathBuf,
    streamed: Vec<Streamed>,
    by_label: HashMap<String, usize>,
}

impl Extractor {
    /// Creates `out_dir` if needed.
    pub fn new(
        patterns: &[String],
        out_dir: impl Into<PathBuf>,
        threshold: u64,
    ) -> io::Result<Self> {
        let out_dir = out_dir.into();
        fs::create_dir_all(&out_dir)?;
        Ok(Extractor {
            patterns: patterns.to_vec(),
            threshold,
            out_dir,
            streamed: Vec::new(),
            by_label: HashMap::new(),
        })
    }

    fn selected(&self, path: &str) -> bool {
        self.patterns.iter().any(|p| selects(p, path))
    }

    /// Give the files written during parsing their final names, write the
    /// selected vectors that were materialized instead (e.g. expanded ALTREP
    /// strings), and write `manifest.json`. Returns the manifest entries.
    pub fn finish(mut self, doc: &Document) -> io::Result<Vec<Json>> {
        let mut entries = Vec::new();
        let mut skipped = Vec::new();
        let mut res = Ok(());
        walk(doc, &mut |path, name, o| {
            if res.is_err() || !self.selected(path) {
                return res.is_err();
            }
            let label = match name {
                Some(n) if path.ends_with(']') => format!("{path}_{n}"),
                _ => path.to_string(),
            };
            let stem = format!("{:04}_{}", entries.len(), sanitize(&label));
            let Some(descr) = npy_descr(o.sexptype()) else {
                return false; // containers: descend
            };
            let file = format!("{stem}.{}", if descr.is_some() { "npy" } else { "json" });
            res = match &o.value {
                Value::Streamed { target, .. } => match self.by_label.get(target) {
                    Some(&i) if !self.streamed[i].claimed => {
                        self.streamed[i].claimed = true;
                        fs::rename(self.out_dir.join(target), self.out_dir.join(&file))
                    }
                    _ => return false, // streamed by someone else
                },
                Value::Skipped { len, .. } => {
                    skipped.push(json!({ "path": path, "len": len }));
                    return false;
                }
                Value::Integer(_)
                | Value::Logical(_)
                | Value::Real(_)
                | Value::Complex(_)
                | Value::Raw(_)
                | Value::Character(_) => FileSink::create(
                    &self.out_dir.join(&file),
                    file.clone(),
                    descr,
                    o.len().unwrap(),
                )
                .and_then(|s| s.write_all_value(&o.value)),
                _ => return false,
            };
            entries.push(entry(path, name, &file, o, descr));
            false
        });
        res?;
        // Vectors the tree walk cannot reach, such as those inside environments.
        for s in self.streamed.iter_mut().filter(|s| !s.claimed) {
            s.claimed = true;
            let descr = npy_descr(s.sexptype).flatten();
            let file = format!(
                "{:04}_{}.{}",
                entries.len(),
                sanitize(&s.path),
                if descr.is_some() { "npy" } else { "json" }
            );
            fs::rename(self.out_dir.join(&s.temp), self.out_dir.join(&file))?;
            let o = RObject::new(Value::Streamed {
                sexptype: s.sexptype,
                len: s.len,
                target: s.temp.clone(),
            });
            entries.push(entry(&s.path, None, &file, &o, descr));
        }
        let manifest = json!({ "entries": entries, "skipped_matches": skipped });
        let mut w = BufWriter::new(File::create(self.out_dir.join("manifest.json"))?);
        serde_json::to_writer_pretty(&mut w, &manifest).map_err(io::Error::other)?;
        w.flush()?;
        Ok(entries)
    }
}

impl Policy for Extractor {
    fn keep(&mut self, path: &str, _sexptype: u8, len: u64) -> bool {
        len <= self.threshold || self.selected(path) || (len <= METADATA_LIMIT && is_metadata(path))
    }

    fn sink(&mut self, path: &str, sexptype: u8, len: u64) -> io::Result<Option<Box<dyn Sink>>> {
        let Some(descr) = npy_descr(sexptype).filter(|_| !is_metadata(path) && self.selected(path))
        else {
            return Ok(None);
        };
        let temp = format!(".rdstream-{:06}.part", self.streamed.len());
        let sink = FileSink::create(&self.out_dir.join(&temp), temp.clone(), descr, len)?;
        self.by_label.insert(temp.clone(), self.streamed.len());
        self.streamed.push(Streamed {
            temp,
            path: path.to_string(),
            sexptype,
            len,
            claimed: false,
        });
        Ok(Some(Box::new(sink)))
    }
}

impl Drop for Extractor {
    /// Parsing failed or `finish` was never called: remove partial files.
    fn drop(&mut self) {
        for s in self.streamed.iter().filter(|s| !s.claimed) {
            let _ = fs::remove_file(self.out_dir.join(&s.temp));
        }
    }
}

fn entry(path: &str, name: Option<&str>, file: &str, o: &RObject, dtype: Option<&str>) -> Json {
    let mut entry = json!({
        "path": path,
        "name": name,
        "file": file,
        "type": type_name(o.sexptype()),
        "len": o.len(),
        "dtype": dtype,
    });
    if !o.classes().is_empty() {
        entry["class"] = json!(o.classes());
    }
    if let Some(Value::Integer(d)) = o.attr("dim").map(|d| &d.value) {
        entry["dim"] = json!(d);
    }
    if let Some(Value::Character(l)) = o.attr("levels").map(|d| &d.value) {
        entry["levels"] = json!(l);
    }
    entry
}

/// Write every vector under paths matching `patterns` from an already parsed
/// document into `out_dir`. Returns the manifest entries (also written to
/// `out_dir/manifest.json`). To avoid loading the vectors in the first place,
/// parse with an [`Extractor`] instead.
pub fn extract(doc: &Document, patterns: &[String], out_dir: &Path) -> io::Result<Vec<Json>> {
    Extractor::new(patterns, out_dir, 0)?.finish(doc)
}
