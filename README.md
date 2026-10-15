<div align="center">

<h1>rdstream</h1>

<p><strong>Streaming access to R serialized objects</strong></p>
<p>A Rust library and command-line tool for reading R <code>.rds</code> and <code>.RData</code> files.<br>Supports object inspection, selective extraction, and decompression without an R runtime.</p>

<p>
  <a href="Cargo.toml"><img src="https://img.shields.io/badge/Rust-2024_edition-c45508?style=flat-square" alt="Rust 2024 edition"></a>
  <a href="#supported-formats"><img src="https://img.shields.io/badge/formats-.rds_%C2%B7_.RData-276DC3?style=flat-square" alt="Formats: .rds and .RData"></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-MIT_OR_Apache--2.0-357a50?style=flat-square" alt="License: MIT OR Apache-2.0"></a>
</p>

<p><strong>English</strong> · <a href="doc/README.zh-CN.md">简体中文</a> · <a href="doc/README.ja.md">日本語</a></p>
<p><a href="#installation">Installation</a> · <a href="#quick-start">Quick start</a> · <a href="#python-and-anndata">Python</a> · <a href="#rust-library">Rust library</a> · <a href="#performance">Performance</a> · <a href="#contributing">Contributing</a></p>

</div>

---

R objects from Seurat, SingleCellExperiment, and other analysis workflows can
contain tens of gigabytes of matrices, graphs, and metadata. Analysis tasks may
require only a subset of these data.

rdstream parses the serialization stream in a single forward pass and applies
selection rules before reading each vector's data. It supports object inspection,
cell annotation extraction, and decompressed output without loading the entire
object or creating a decompressed temporary file.

```sh
rdstream describe dataset.rds --depth 4
rdstream extract dataset.rds --path '*@meta.data*' --out meta/
```

## Features

| Capability | Description |
| :--- | :--- |
| **Selective loading** | Retain vectors by path and size; skip the remaining vector data. |
| **Streaming extraction** | `extract` writes selected vectors to files in chunks without retaining complete vectors in memory. |
| **One forward pass** | Read regular files, standard input, or download streams. |
| **Nested compression** | Automatically detect and decompress gzip, bzip2, xz, and zstd layers; support parallel decoding of both gzip layers in recompressed GEO files. |
| **Parallel gzip** | Choose decoder workers from available memory, with a single-threaded fallback. |
| **R serialization encodings** | XDR, ASCII (`ascii = TRUE` / `NA`), and native binary, serialization versions 2 and 3. |
| **R object structure** | Preserve attributes, S4 slots, factors, data frames, references, and byte code. |
| **CLI, Rust, Python** | Inspect objects from the command line, define selection policies in Rust, or convert Seurat and SingleCellExperiment objects to AnnData in Python. |

Skipped vector data are not retained in memory, and `extract` writes selected
vectors incrementally. Total memory use includes the object structure, retained
vectors, and decoder buffers.

## Installation

Install from crates.io with Rust 1.88 or newer:

```sh
cargo install rdstream --locked
rdstream --help
```

To build from a local checkout, run these commands in the repository root:

```sh
cargo install --path . --locked
rdstream --help
```

Parallel gzip support is enabled by default. To build without the parallel
decoders:

```sh
cargo install rdstream --locked --no-default-features
```

## Quick start

### 1. Inspect an object

```sh
rdstream describe GSE183852_DCM_Integrated.Robj.gz --depth 4
```

An abbreviated Seurat object tree:

```text
$RefMerge: S4 <Seurat>
  @assays: list[4]
    [0]RNA: S4 <Assay>
      @counts: S4 <dgCMatrix>
        @i: int[426,652,860] (skipped)
        @p: int[269,795] [0, 919, 1923, 3081, …]
        @Dim: int[2] [45068, 269794]
        @x: dbl[426,652,860] (skipped)
  …
  @meta.data: list[55] <data.frame>
    …
    [24]condition: chr[269,794] ["Donor", "Donor", "Donor", "Donor", …]
    …
    [50]Names: int[269,794] [13, 13, 13, 13, …] <factor>
```

`describe` keeps vectors of up to **1,000,000 elements** by default. Larger vectors
are shown with their type and length. Set `--threshold` to change that limit.

### 2. Extract selected data

```sh
rdstream extract GSE183852_DCM_Integrated.Robj.gz \
  --path '$RefMerge@meta.data*' \
  --out meta/
```

| Output | Contents |
| :--- | :--- |
| `*.npy` | Integer, logical, real, complex, and raw vectors. |
| `*.json` | Character vectors. |
| `manifest.json` | Paths, names, types, lengths, and available `dim`, `class`, and factor `levels` metadata. |

Selected vectors are written in chunks as the stream is read, without retaining
the complete vector data in memory. A path selects all descendant elements:
`--path '$RefMerge@meta.data'` extracts every column.

<details>
<summary><strong>Read extracted factors with Python and NumPy</strong></summary>

```python
import json
from pathlib import Path

import numpy as np

root = Path("meta")
manifest = json.loads((root / "manifest.json").read_text())
entry = next(e for e in manifest["entries"] if e["name"] == "Names")
codes = np.load(root / entry["file"])
levels = np.asarray(entry["levels"], dtype=object)

# R factor codes are 1-based; preserve missing values as None.
cell_type = np.full(codes.shape, None, dtype=object)
valid = (codes >= 1) & (codes <= len(levels))
cell_type[valid] = levels[codes[valid] - 1]
```

</details>

### 3. Work with streams

```sh
# Read a GEO download directly from standard input.
curl -sL https://ftp.ncbi.nlm.nih.gov/geo/series/GSE183nnn/GSE183852/suppl/GSE183852_DCM_Cells.Robj.gz \
  | rdstream describe - --depth 3

# Extract cell annotations from the download stream.
curl -sL https://ftp.ncbi.nlm.nih.gov/geo/series/GSE183nnn/GSE183852/suppl/GSE183852_DCM_Cells.Robj.gz \
  | rdstream extract - --path '*@meta.data' --out meta/

# Use file redirection to enable parallel decompression.
rdstream describe - < GSE183852_DCM_Cells.Robj.gz

# Spool archive output to /scratch for parallel decompression.
tar -xOf archive.tar data/seurat.rds.gz \
  | rdstream extract - --path '*@meta.data' --out meta/ --spool-dir /scratch

# Remove all compression layers, leaving the R serialization stream.
rdstream cat dataset.rds.gz > dataset.rds
```

Piped input uses one decoder thread per compression layer, with decompression
and parsing running as a pipeline. With `--spool-dir DIR`, rdstream copies the
compressed input to a temporary file in `DIR` before parallel decompression.
The temporary file requires space equal to the compressed input size and is
removed after processing. Spooling can improve throughput for fast local input;
the benefit depends on input and storage speeds. Use a disk-backed directory
to avoid storing the temporary file in memory. Standard input redirected from a
file (`rdstream describe - < dataset.rds.gz`) requires no spooling.

## Command reference

| Command | Purpose | Main options |
| :--- | :--- | :--- |
| `rdstream describe <FILE>` | Print the object tree. | `--depth`, `--threshold` |
| `rdstream extract <FILE>` | Stream selected vectors to files. | `--path` (repeatable), `--out`, `--threshold` |
| `rdstream cat <FILE>` | Write decompressed bytes to stdout. | Global decoder options below. |

Every command accepts `-` as its input file to read standard input. Run
`rdstream <COMMAND> --help` for the full options.

### Paths and selection

| Segment | Meaning | Example |
| :--- | :--- | :--- |
| `$name` | Tagged pairlist entry, such as an object saved in `.RData`. | `$RefMerge` |
| `@name` | Attribute or S4 slot. | `@meta.data`, `@counts` |
| `[k]` | List element at a zero-based index. | `[24]` |

Patterns support `*` to match any sequence of characters, including path
separators; all other characters are literal. Quote patterns in your shell:
`'*@meta.data*'` or `'$obj@assays[0]@counts*'`. For `extract`, a pattern also
selects everything below the paths it matches.

List elements use indices because R serializes their names **after** their data.
`describe` shows both the index and the name once available, as in `[24]condition`.

### Decoder memory and threads

```sh
# Set the decoder memory budget to 1 GiB.
rdstream describe dataset.rds.gz --memory-budget 1G

# Force single-threaded decoding for lower memory use.
rdstream describe dataset.rds.gz --threads 1
```

By default, the planner uses half of available memory reported by Linux, or a
2 GiB fallback budget when that information is unavailable. Layers that do not
fit the estimated budget use single-threaded decoding. Piped input decodes
single-threaded unless `--spool-dir` is given.

`--memory-budget` covers decoder estimates, not total process memory. An explicit
`--threads N` overrides automatic sizing; values above `1` ignore the budget.

## Python and AnnData

The Python bindings are available in [`python/`](python/). Build and install
them from a local checkout with a Rust toolchain:

```sh
pip install "./python[anndata]"
```

Atomic vectors are returned as NumPy arrays that reuse the parser's buffers, and
`rdstream.anndata` converts Seurat (v3/v4 `Assay` and v5 `Assay5`, including
split layers) and SingleCellExperiment objects:

```python
import rdstream
from rdstream.anndata import read

adata = read("GSE183852_DCM_Integrated.Robj.gz", assay="RNA", X="counts", layers=())
adata.write_h5ad("dcm.h5ad")

doc = rdstream.read("seurat.rds", keep=["*@meta.data*"], threshold=10_000)
meta = rdstream.as_pandas(doc.root.attr("meta.data"))
```

The AnnData reader selects assay matrices, metadata, and embeddings for
conversion. Sparse matrices are transposed without copying. See
[python/README.md](python/README.md) for the mapping and options.

## Rust library

Add rdstream as a dependency:

```toml
[dependencies]
rdstream = "0.1"
```

```rust
use rdstream::Rules;

fn main() -> rdstream::Result<()> {
    let mut rules = Rules {
        threshold: 10_000,
        keep: vec!["*@meta.data*".into()],
        ..Default::default()
    };

    let parsed = rdstream::parse_file("seurat.rds", &mut rules)?;
    for (name, object) in parsed.document.objects() {
        println!("{name}: {:?}", object.classes());
    }
    Ok(())
}
```

For custom selection, pass a closure with the signature
`(path: &str, sexptype: u8, len: u64) -> bool`. A `Policy` can also return a
`Sink` from `Policy::sink` to receive a vector's elements in chunks instead of
loading it; `extract::Extractor` uses this to write files. Use `parse_source`
with a `Source` and `DecodeOptions` to control input streams, decoder threads,
and memory budgets. Generate the local API documentation with
`cargo doc --open --no-deps`.

## Performance

The following measurements were recorded on an AMD EPYC 7402 with 24 cores,
48 threads, 62 GB RAM, and NVMe storage. The input was a synthetic, Seurat-like
2.9 GB serialization stream compressed with gzip `-6`, then compressed again
at the level shown below.

| Outer compression level | Elapsed time | Memory use |
| :--- | ---: | ---: |
| gzip `-6` | 1.96 s | 1.2 GB |
| gzip `-1` | 2.03 s | 1.3 GB |
| gzip `-9` | 2.03 s | 1.2 GB |

These results apply to the recorded workload and environment. Performance and
memory use depend on the input, compression, retained objects, and decoder
configuration.

## Supported formats

| Area | Support |
| :--- | :--- |
| Containers | `saveRDS()` (`.rds`) and `save()` (`.RData`, `.rda`, `.Robj`). |
| Serialization | Versions 2 and 3; XDR, ASCII (decimal or `ascii = NA` hex doubles, any line endings), and native binary. |
| Compression | gzip, bzip2, xz, zstd (R 4.5's `zstdfile()` or the `zstd` command), and nested combinations. |
| Object model | Attributes, S4 slots including pseudo-NULL, factors, data frames, environments with self-references, symbol references, byte-compiled closures, and long vectors. |
| ALTREP | Compact sequences, wrappers, and deferred strings; unknown classes retain their serialized state. |
| Strings | UTF-8, Latin-1, and unflagged strings in a Latin-1 or Windows-1252 native encoding. |

### Current limitations

- Native binary serialization is read as little-endian; R does not record the writer's byte order.
- Piped input decodes in parallel only when spooled to disk with `--spool-dir`.
- Decoder memory budgets are estimates, not hard limits, and exclude retained objects.

## Roadmap

- [x] Python bindings with PyO3 and Seurat / SingleCellExperiment → AnnData conversion.
- [x] Write selected vectors directly to disk during parsing.
- [x] Improve parallel decoding of recompressed outer layers.
- [x] Add R-generated fixtures to CI.
- [x] Support ASCII serialization.
- [x] Support zstd compression.
- [x] Bounded-memory decoding of streams, with parallel decoding of redirected and spooled standard input.

## Contributing

Contributions may be submitted as issue reports or pull requests. For parsing
issues, include the command, error message, R version and save format if known,
and a minimal reproducible input when available.

Run the tests with and without parallel gzip support, and the Python tests:

```sh
cargo test --locked
cargo test --locked --no-default-features
pip install "./python[test]" && pytest python/tests
```

The sources and contents of third-party test fixtures are documented in
[tests/fixtures/MANIFEST.md](tests/fixtures/MANIFEST.md). CI also generates
fixtures with [tests/r/generate.R](tests/r/generate.R) and validates the results
against R. Run the same checks locally with:

```sh
Rscript tests/r/generate.R target/r-fixtures
RDSTREAM_R_FIXTURES=target/r-fixtures cargo test --test r_generated
RDSTREAM_R_FIXTURES=target/r-fixtures pytest python/tests/test_r_generated.py
```

<details>
<summary><strong>Related projects</strong></summary>

Other tools for working with R serialization include
[rds2rust](https://github.com/andrewwbutler/rds2rust),
[rds2cpp](https://github.com/LTLA/rds2cpp) /
[rds2py](https://pypi.org/project/rds2py/),
[rdata](https://github.com/vnmabus/rdata), and
[readseurat](https://github.com/harryhaller001/readseurat).
[rapidgzip](https://github.com/mxmlnkn/rapidgzip) explores parallel gzip decoding.

</details>

## License

Licensed under either [MIT](LICENSE-MIT) or
[Apache License 2.0](LICENSE-APACHE), at your option.
