//! Python bindings: parse R serialization streams into Python objects, with
//! atomic vectors as NumPy arrays that take over the parser's buffers.

use std::io::Cursor;
use std::path::PathBuf;

use numpy::PyArray1;
use pyo3::create_exception;
use pyo3::exceptions::{PyOSError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList, PyTuple};
use rdstream::sexp::type_name;
use rdstream::{DecodeOptions, Encoding, Format, Parsed, Policy, RObject, Rules, Source, Value};

create_exception!(
    _rdstream,
    ParseError,
    PyValueError,
    "The input is not a valid R serialization stream."
);

/// A path (`-` for standard input) or the file's bytes.
#[derive(FromPyObject)]
enum Input<'py> {
    Bytes(Bound<'py, PyBytes>),
    Path(PathBuf),
}

impl Input<'_> {
    fn source(&self) -> Source {
        match self {
            Input::Bytes(b) => Source::Stream(Box::new(Cursor::new(b.as_bytes().to_vec()))),
            Input::Path(p) => rdstream::source_for(p),
        }
    }
}

fn to_py_err(e: rdstream::Error) -> PyErr {
    match e {
        rdstream::Error::Io(e) => PyOSError::new_err(e.to_string()),
        e => ParseError::new_err(e.to_string()),
    }
}

fn options(
    threads: usize,
    memory_budget: Option<u64>,
    spool_dir: Option<PathBuf>,
) -> DecodeOptions {
    DecodeOptions {
        threads,
        memory_budget,
        spool_dir,
    }
}

/// Builds instances of the classes in `rdstream._objects`.
struct Conv<'py> {
    py: Python<'py>,
    robject: Bound<'py, PyAny>,
    skipped: Bound<'py, PyAny>,
    streamed: Bound<'py, PyAny>,
}

impl<'py> Conv<'py> {
    fn new(py: Python<'py>) -> PyResult<Self> {
        let m = py.import("rdstream._objects")?;
        Ok(Conv {
            py,
            robject: m.getattr("RObject")?,
            skipped: m.getattr("Skipped")?,
            streamed: m.getattr("Streamed")?,
        })
    }

    fn object(&self, o: RObject) -> PyResult<Bound<'py, PyAny>> {
        let t = type_name(o.sexptype());
        let attributes = PyDict::new(self.py);
        for (k, v) in o.attributes {
            attributes.set_item(k, self.object(v)?)?;
        }
        let value = self.value(o.value)?;
        self.robject.call1((t, value, attributes, o.is_object))
    }

    fn value(&self, v: Value) -> PyResult<Bound<'py, PyAny>> {
        let py = self.py;
        Ok(match v {
            Value::Null | Value::S4 | Value::ExternalPointer | Value::WeakReference => {
                py.None().into_bound(py)
            }
            Value::Symbol(s) => s.into_pyobject(py)?.into_any(),
            Value::Logical(v) | Value::Integer(v) => PyArray1::from_vec(py, v).into_any(),
            Value::Real(v) => PyArray1::from_vec(py, v).into_any(),
            Value::Complex(v) => {
                PyArray1::from_vec(py, v.into_flattened()).call_method1("view", ("complex128",))?
            }
            Value::Raw(v) => PyArray1::from_vec(py, v).into_any(),
            Value::Character(v) => PyList::new(py, v)?.into_any(),
            Value::List(items) | Value::Expression(items) => PyList::new(
                py,
                items
                    .into_iter()
                    .map(|o| self.object(o))
                    .collect::<PyResult<Vec<_>>>()?,
            )?
            .into_any(),
            Value::Pairlist { items, .. } => PyList::new(
                py,
                items
                    .into_iter()
                    .map(|(tag, o)| {
                        PyTuple::new(py, [tag.into_pyobject(py)?.into_any(), self.object(o)?])
                    })
                    .collect::<PyResult<Vec<_>>>()?,
            )?
            .into_any(),
            Value::Environment(i) => i.into_pyobject(py)?.into_any(),
            Value::Builtin { name, .. } => name.into_pyobject(py)?.into_any(),
            Value::Singleton(t) => type_name(t).into_pyobject(py)?.into_any(),
            Value::Reference { info, .. } => PyList::new(py, info)?.into_any(),
            Value::Bytecode { code, consts } => PyTuple::new(
                py,
                [
                    self.object(*code)?,
                    PyList::new(
                        py,
                        consts
                            .into_iter()
                            .map(|o| self.object(o))
                            .collect::<PyResult<Vec<_>>>()?,
                    )?
                    .into_any(),
                ],
            )?
            .into_any(),
            Value::AltRep { class, state } => PyTuple::new(
                py,
                [class.into_pyobject(py)?.into_any(), self.object(*state)?],
            )?
            .into_any(),
            Value::Skipped { sexptype, len } => self.skipped.call1((type_name(sexptype), len))?,
            Value::Streamed {
                sexptype,
                len,
                target,
            } => self.streamed.call1((type_name(sexptype), len, target))?,
        })
    }

    /// Fields of `rdstream.Document`.
    fn document(&self, parsed: Parsed) -> PyResult<Bound<'py, PyDict>> {
        let py = self.py;
        let d = parsed.document;
        let out = PyDict::new(py);
        out.set_item(
            "format",
            match d.format {
                Format::Rds => "rds",
                Format::RData => "rdata",
            },
        )?;
        out.set_item(
            "encoding",
            match d.encoding {
                Encoding::Xdr => "xdr",
                Encoding::Ascii => "ascii",
                Encoding::Binary => "binary",
            },
        )?;
        out.set_item("version", d.version)?;
        out.set_item("writer_version", d.writer_version)?;
        out.set_item("native_encoding", d.native_encoding)?;
        out.set_item("bytes_read", d.bytes_read)?;
        let layers = PyList::empty(py);
        for l in &parsed.layers {
            let layer = PyDict::new(py);
            layer.set_item("compression", format!("{:?}", l.compression).to_lowercase())?;
            layer.set_item("decoder", format!("{:?}", l.decoder))?;
            layer.set_item("threads", l.threads)?;
            layers.append(layer)?;
        }
        out.set_item("layers", layers)?;
        let envs = PyList::empty(py);
        for e in d.environments {
            let env = PyDict::new(py);
            env.set_item("locked", e.locked)?;
            env.set_item("enclosure", self.object(e.enclosure)?)?;
            env.set_item("frame", self.object(e.frame)?)?;
            env.set_item("hashtab", self.object(e.hashtab)?)?;
            env.set_item("attributes", self.object(e.attributes)?)?;
            envs.append(env)?;
        }
        out.set_item("environments", envs)?;
        out.set_item("root", self.object(d.root)?)?;
        Ok(out)
    }
}

/// A Python callable `(path, type, length) -> bool`, called from the parser
/// thread. The first exception it raises stops further calls and is re-raised
/// once parsing returns.
struct PyPolicy {
    keep: Py<PyAny>,
    error: Option<PyErr>,
}

impl Policy for PyPolicy {
    fn keep(&mut self, path: &str, sexptype: u8, len: u64) -> bool {
        if self.error.is_some() {
            return false;
        }
        Python::attach(|py| {
            match self
                .keep
                .call1(py, (path, type_name(sexptype), len))
                .and_then(|r| r.is_truthy(py))
            {
                Ok(keep) => keep,
                Err(e) => {
                    self.error = Some(e);
                    false
                }
            }
        })
    }
}

/// Parse `source` and return the fields of an `rdstream.Document`.
#[pyfunction]
#[pyo3(signature = (source, *, threshold, keep, skip, policy, threads, memory_budget, spool_dir))]
#[allow(clippy::too_many_arguments)]
fn read<'py>(
    py: Python<'py>,
    source: Input<'py>,
    threshold: Option<u64>,
    keep: Vec<String>,
    skip: Vec<String>,
    policy: Option<Py<PyAny>>,
    threads: usize,
    memory_budget: Option<u64>,
    spool_dir: Option<PathBuf>,
) -> PyResult<Bound<'py, PyDict>> {
    let source = source.source();
    let opts = options(threads, memory_budget, spool_dir);
    let parsed = match policy {
        Some(keep) => {
            let mut policy = PyPolicy { keep, error: None };
            let res = py.detach(|| rdstream::parse_source(source, &mut policy, &opts));
            if let Some(e) = policy.error {
                return Err(e);
            }
            res
        }
        None => {
            let mut rules = Rules {
                threshold: threshold.unwrap_or(u64::MAX),
                keep,
                skip,
            };
            py.detach(move || rdstream::parse_source(source, &mut rules, &opts))
        }
    }
    .map_err(to_py_err)?;
    Conv::new(py)?.document(parsed)
}

/// The object tree as printed by `rdstream describe`.
#[pyfunction]
#[pyo3(signature = (source, *, depth, threshold, threads, memory_budget, spool_dir))]
fn describe(
    py: Python<'_>,
    source: Input<'_>,
    depth: usize,
    threshold: u64,
    threads: usize,
    memory_budget: Option<u64>,
    spool_dir: Option<PathBuf>,
) -> PyResult<String> {
    let source = source.source();
    let opts = options(threads, memory_budget, spool_dir);
    py.detach(move || {
        let parsed = rdstream::parse_source(
            source,
            &mut Rules {
                threshold,
                ..Default::default()
            },
            &opts,
        )
        .map_err(to_py_err)?;
        let mut out = Vec::new();
        rdstream::describe::describe(&parsed.document, depth, &mut out)?;
        Ok(String::from_utf8_lossy(&out).into_owned())
    })
}

/// Stream vectors under `paths` into `out`; returns the manifest as JSON.
#[pyfunction]
#[pyo3(signature = (source, paths, out, *, threshold, threads, memory_budget, spool_dir))]
#[allow(clippy::too_many_arguments)]
fn extract(
    py: Python<'_>,
    source: Input<'_>,
    paths: Vec<String>,
    out: PathBuf,
    threshold: u64,
    threads: usize,
    memory_budget: Option<u64>,
    spool_dir: Option<PathBuf>,
) -> PyResult<String> {
    let source = source.source();
    let opts = options(threads, memory_budget, spool_dir);
    py.detach(move || {
        let mut extractor = rdstream::extract::Extractor::new(&paths, &out, threshold)?;
        let parsed = rdstream::parse_source(source, &mut extractor, &opts).map_err(to_py_err)?;
        extractor.finish(&parsed.document)?;
        Ok(std::fs::read_to_string(out.join("manifest.json"))?)
    })
}

#[pymodule]
fn _rdstream(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read, m)?)?;
    m.add_function(wrap_pyfunction!(describe, m)?)?;
    m.add_function(wrap_pyfunction!(extract, m)?)?;
    m.add("ParseError", m.py().get_type::<ParseError>())?;
    Ok(())
}
