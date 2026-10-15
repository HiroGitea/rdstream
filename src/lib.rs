//! Single-pass, constant-memory parser for R's serialization format
//! (`saveRDS()` `.rds` and `save()` `.RData`/`.rda`/`.Robj` files).
//!
//! The input is read strictly forward, so it can be a pipe or a download stream.
//! Compression layers (gzip, bzip2, xz, zstd, also nested) are peeled on background
//! threads. A [`Policy`] sees every vector's path, type and length before its data
//! is read and decides whether to materialize it; skipped vectors cost only the
//! time to read past them.
//!
//! ```no_run
//! let mut rules = rdstream::Rules { threshold: 10_000, keep: vec!["*@meta.data*".into()], ..Default::default() };
//! let parsed = rdstream::parse_file("seurat.rds", &mut rules)?;
//! for (name, obj) in parsed.document.objects() {
//!     println!("{name}: {:?}", obj.classes());
//! }
//! # Ok::<(), rdstream::Error>(())
//! ```

pub mod decompress;
pub mod describe;
mod error;
pub mod extract;
mod parser;
pub mod policy;
mod reader;
pub mod sexp;

use std::io::Read;
use std::path::Path;

pub use decompress::{Compression, DecodeOptions, Decoder, Layer, Source};
pub use error::{Error, Result};
pub use parser::Parser;
pub use policy::{Chunk, Policy, Rules, Sink};
pub use sexp::{Document, Encoding, Environment, Format, RObject, Value};

/// Parse result plus the compression layers that were peeled (outermost first)
/// and the decoder used for each.
#[derive(Debug, Clone)]
pub struct Parsed {
    pub document: Document,
    pub layers: Vec<Layer>,
}

/// Deeply nested objects recurse; give the parser a roomy stack.
const PARSER_STACK: usize = 256 << 20;

/// Parse a possibly compressed serialization stream.
pub fn parse_reader<P: Policy + Send>(
    input: Box<dyn Read + Send>,
    policy: &mut P,
) -> Result<Parsed> {
    parse_source(Source::Stream(input), policy, &DecodeOptions::default())
}

/// Parse a file; `-` reads standard input.
pub fn parse_file<P: Policy + Send>(path: impl AsRef<Path>, policy: &mut P) -> Result<Parsed> {
    parse_source(source_for(path.as_ref()), policy, &DecodeOptions::default())
}

/// `Source` for a CLI-style path argument (`-` is standard input). Standard
/// input redirected from a regular file is read as that file, so it can be
/// decoded in parallel.
pub fn source_for(path: &Path) -> Source {
    if path != Path::new("-") {
        return Source::File(path.to_path_buf());
    }
    #[cfg(unix)]
    if std::fs::metadata("/dev/stdin").is_ok_and(|m| m.is_file()) {
        return Source::File("/dev/stdin".into());
    }
    Source::Stream(Box::new(std::io::stdin()))
}

/// Parse from any source with explicit decoding options.
pub fn parse_source<P: Policy + Send>(
    source: Source,
    policy: &mut P,
    opts: &DecodeOptions,
) -> Result<Parsed> {
    let (inner, layers) = decompress::decompress_layers(source, opts)?;
    let document = std::thread::scope(|s| {
        std::thread::Builder::new()
            .name("rdstream-parser".into())
            .stack_size(PARSER_STACK)
            .spawn_scoped(s, || Parser::new(inner, policy).parse_document())
            .expect("failed to spawn parser thread")
            .join()
            .unwrap_or_else(|e| std::panic::resume_unwind(e))
    })?;
    Ok(Parsed { document, layers })
}
