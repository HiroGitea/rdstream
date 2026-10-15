# rdstream for Python

The rdstream Python bindings read R `.rds` and `.RData` files in a single
streaming pass without an R runtime. Numeric vectors are exposed as NumPy arrays
that reuse the parser's buffers. Conversion functions support R data frames as
pandas objects, `Matrix` sparse matrices as SciPy objects, and Seurat and
SingleCellExperiment objects as AnnData.

## Installation

Installation from a local checkout requires Python 3.10 or newer and a stable
Rust toolchain. Run the following commands from the repository root:

```sh
pip install "./python"            # NumPy only
pip install "./python[anndata]"   # with pandas, SciPy and AnnData
```

## Reading objects

```python
import rdstream

doc = rdstream.read("seurat.rds.gz", threshold=10_000, keep=["*@meta.data*"])
doc.format, doc.encoding, doc.layers      # ("rds", "xdr", [{"compression": "gzip", ...}])
obj = doc.root                            # for .RData: doc.objects / doc["name"]
obj.classes                               # ["Seurat"]
meta = obj.attr("meta.data")              # attributes and S4 slots
rdstream.as_pandas(meta)                  # pandas DataFrame
```

Vectors larger than `threshold` elements are skipped unless a `keep` glob
matches their path. `skip` globs override both, and `threshold=None` disables
size-based filtering. Paths follow the notation displayed by
`rdstream.describe(source)`. For custom selection rules, pass
`policy=lambda path, type, length: ...` to evaluate each vector.

`source` can be a path, `"-"` for standard input, or the file's bytes.
gzip, bzip2, xz, and zstd compression layers, including nested layers, are
detected and decompressed automatically. gzip files support parallel decoding.
For piped input, set `spool_dir` to a disk-backed directory to store a temporary
copy before parallel decoding. The GIL is released during parsing.

```python
# curl -sL https://…/GSE183852_DCM_Cells.Robj.gz | python load.py
doc = rdstream.read("-", threshold=10_000)

# tar -xOf archive.tar data/seurat.rds.gz | python load.py
doc = rdstream.read("-", spool_dir="/scratch")

# Convert a Seurat object from standard input to AnnData.
from rdstream.anndata import read
adata = read("-")
```

### The object model

Every node is an `RObject` with `type`, `value`, `attributes` and `is_object`:

| R | `type` | `value` |
| :--- | :--- | :--- |
| integer, logical | `"int"`, `"lgl"` | `int32` array; `NA` is `rdstream.NA_INTEGER` |
| double, complex, raw | `"dbl"`, `"cplx"`, `"raw"` | `float64` / `complex128` / `uint8` array |
| character | `"chr"` | list of `str`, `None` for `NA` |
| list | `"list"` | list of `RObject` |
| S4 object | `"S4"` | `None`; slots are in `attributes` |
| skipped vector | as above | `rdstream.Skipped(type, length)` |

`obj["name"]` and `obj[0]` index lists, `obj.attr("x")` reads attributes and
slots, and `obj.names`, `obj.dim` and `obj.inherits(...)` work as in R.

### Conversions

- `rdstream.as_pandas(obj)` converts data frames, S4Vectors `DataFrame`s,
  matrices and vectors. Factors become categoricals, `Date`/`POSIXct` become
  datetimes, and integers or logicals with `NA` become nullable arrays.
- `rdstream.as_sparse(obj, transpose=False)` converts `dgCMatrix`,
  `dgRMatrix`, `dgTMatrix` and their logical, pattern and symmetric variants.
  `transpose=True` returns the transpose as CSR without copying.
- `rdstream.as_array(obj)` returns a vector, matrix or array shaped by its `dim`.

### Streaming vectors to files

```python
manifest = rdstream.extract("big.rds", ["*@assays[0]@counts"], "out/")
```

This function writes `.npy` and `.json` files with a `manifest.json` index during
parsing, using the same extraction behavior as `rdstream extract`. Vector data
are written in chunks without retaining complete vectors in memory.

## Seurat and SingleCellExperiment to AnnData

```python
from rdstream.anndata import read, read_seurat, read_sce

adata = read("pbmc.rds")                                    # either class
adata = read_seurat("GSE.Robj.gz", name="RefMerge", assay="RNA", X="counts", layers=())
adata = read_sce("sce.rds", X="logcounts")
```

| AnnData | Seurat (v3/v4 `Assay`, v5 `Assay5`) | SingleCellExperiment |
| :--- | :--- | :--- |
| `X`, `layers` | `counts` / `data` / `scale.data`; v5 split layers (`counts.1`, `counts.2`, …) are joined | `assays(sce)` |
| `obs` | `meta.data` and `active.ident` | `colData(sce)` |
| `var` | `meta.features` (v5: feature `meta.data`) and `highly_variable` | `rowData(sce)` |
| `obsm["X_<name>"]` | reductions' `cell.embeddings` | `reducedDims(sce)` |
| `obsp[name]` | `graphs` (with `obsp=True`) | — |

Defaults:

- Seurat: `X="data"` (falling back to `counts`) and `layers=("counts",)`.
- SingleCellExperiment: `X` is `logcounts` if present, else the first assay, and the other assays become layers.

Expression matrices are transposed from R's features × cells layout to cells ×
features; sparse matrices are converted to CSR without copying.

The reader selects assay matrices, cell and feature metadata, and embeddings
for conversion. Command history, images, other reductions' loadings, and similar
slots are skipped above `threshold` elements (default 100,000). Because list
element names are serialized after their contents, matrices from all assays
are read even when a single assay is selected for conversion. Seurat v3
`scale.data` and dense v5 layers are read only when `"scale.data"` is requested.

Seurat v5 layers that share cells (such as `counts.raw` and `counts.filtered`)
cannot be joined. Select a specific layer with `X="counts.raw"`.

## Tests

```sh
pip install "./python[test]"
pytest python/tests
```

The tests use synthetic objects as well as real Seurat and SingleCellExperiment
files from [`tests/fixtures`](../tests/fixtures/MANIFEST.md).
`test_r_generated.py` also checks objects written by
[`tests/r/generate.R`](../tests/r/generate.R) when `RDSTREAM_R_FIXTURES` is set.

## License

Licensed under either [MIT](../LICENSE-MIT) or
[Apache License 2.0](../LICENSE-APACHE), at your option.
