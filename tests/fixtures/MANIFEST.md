# rdstream test fixtures — MANIFEST

Small R-serialized files (`.rds` / `.rda`) collected from open-source projects that commit
R-generated test data. Every data file below was written by R in its upstream project and is
stored byte-for-byte as published there (sha256 in Appendix A).
806 files, 750 KiB in total: 796 R data files, 2 CSV ground-truth files, license texts, and this manifest.

Collected 2026-10-15. Format facts (container, magic, serialization format and version,
compression, writer R version, native encoding, line endings, byte order) come from the magic
bytes and headers. Contents were cross-checked with the independent Python reader
**rdata 1.1.0** (MIT, vnmabus): its high-level conversion where it works, and its low-level parse
tree (`rdata.parser.parse_file`) where conversion fails (closures, Assay5 `LogMap`s, …).

## Layout

| path | source (pinned) | license | files |
|---|---|---|---|
| `rdata/generated/` | [vnmabus/rdata@0edddf4](https://github.com/vnmabus/rdata/tree/0edddf4b692cf5c011976bd7b369ccea08aa5db5/rdata/tests/data/generated) | MIT | 580 |
| `rdata/legacy-1.0.0/` | [vnmabus/rdata tag 1.0.0 @567a2af](https://github.com/vnmabus/rdata/tree/567a2af915b795ddc14394bbb63aa0c1f912f4be/rdata/tests/data) | MIT | 69 |
| `rdatajl/data_v2/`, `rdatajl/data_v3/` | [JuliaData/RData.jl@3103bea](https://github.com/JuliaData/RData.jl/tree/3103bea8f89a443ce86ad6f5d4a97887773c2ede/test) | MIT | 29 + 38 |
| `zurds/` | [pedrobtz/zurds@f35d593](https://github.com/pedrobtz/zurds/tree/f35d59311ce71ab6ebc97eb173393ac44cb775f2/tests/testthat/fixtures) | MIT | 72 |
| `seurat/pbmc_small.rda` | CRAN `SeuratObject_5.4.0.tar.gz` → `SeuratObject/data/pbmc_small.rda` | MIT | 1 |
| `seurat/scRep_example.rda` | [ncborcherding/scRepertoire@49459e4](https://github.com/ncborcherding/scRepertoire/blob/49459e4127ca2976c5fb75a40ecf2388d159c7e9/data/scRep_example.rda) (v2.6.1) | MIT | 1 |
| `seurat/v5/` | [cellgeni/py8rds@3ade6f8](https://github.com/cellgeni/py8rds/tree/3ade6f8e2e6682343e48d146a710867849d7b5a8/tests) | MIT | 3 rds + 2 csv |
| `sce/mouseBrainSubsetSCE.rda`, `sce/scExample.rda` | [compbiomed/singleCellTK@6723890](https://github.com/compbiomed/singleCellTK/tree/6723890f56292752cb75397a988cc9721ec7ff71/data) (v2.18.0) | MIT | 2 |
| `sce/simple_sce.rds` | [BiocPy/rds2py@3e79717](https://github.com/BiocPy/rds2py/blob/3e797173292992bd843c4bf5240f077b90a42645/tests/data/simple_sce.rds) | MIT | 1 |
| `LICENSES/` | license texts for every source above | — | 8 |

Every kept file is MIT-licensed. No Artistic-2.0, GPL, or unlicensed material was kept.

### Reading the entries

- Expected values quoted "verbatim" come from the upstream test suites, in the upstream test
  language: Python/numpy/pandas for rdata, Julia for RData.jl. They describe the R object. Notes
  on R semantics are mine.
- R `NA_real_` is the NaN with low word 1954 (`0x7FF00000000007A2`). rdata calls it `R_FLOAT_NA`.
  `NA_integer_`/`NA` logical is `-2147483648`.
- rdata shows the S4 "NULL slot" placeholder (the symbol `\001NULL\001`) as the string
  `'\x01NULL\x01'`.
- In `.rda` files the top level is a pairlist tagged with the saved variable names. The names
  are given per file.

### Coverage matrix (data files, all sources)

| container | format | ser. version | compression | files |
|---|---|---|---|---|
| rda | ascii | 2 | gzip | 1 |
| rda | ascii | 2 | none | 64 |
| rda | ascii | 3 | gzip | 2 |
| rda | ascii | 3 | none | 64 |
| rda | binary | 2 | gzip | 1 |
| rda | binary | 3 | gzip | 1 |
| rda | xdr | 2 | bzip2 | 2 |
| rda | xdr | 2 | gzip | 88 |
| rda | xdr | 3 | bzip2 | 3 |
| rda | xdr | 3 | gzip | 105 |
| rda | xdr | 3 | none | 2 |
| rda | xdr | 3 | xz | 3 |
| rda | xdr | 3 | zstd | 1 |
| rds | ascii | 2 | bzip2 | 1 |
| rds | ascii | 2 | gzip | 4 |
| rds | ascii | 2 | none | 61 |
| rds | ascii | 2 | xz | 1 |
| rds | ascii | 2 | zstd | 1 |
| rds | ascii | 3 | bzip2 | 1 |
| rds | ascii | 3 | gzip | 4 |
| rds | ascii | 3 | none | 65 |
| rds | ascii | 3 | xz | 1 |
| rds | ascii | 3 | zstd | 1 |
| rds | binary | 2 | bzip2 | 1 |
| rds | binary | 2 | gzip | 1 |
| rds | binary | 2 | none | 59 |
| rds | binary | 2 | xz | 1 |
| rds | binary | 2 | zstd | 1 |
| rds | binary | 3 | bzip2 | 1 |
| rds | binary | 3 | gzip | 1 |
| rds | binary | 3 | none | 59 |
| rds | binary | 3 | xz | 1 |
| rds | binary | 3 | zstd | 1 |
| rds | xdr | 2 | bzip2 | 1 |
| rds | xdr | 2 | gzip | 9 |
| rds | xdr | 2 | none | 60 |
| rds | xdr | 2 | xz | 1 |
| rds | xdr | 2 | zstd | 1 |
| rds | xdr | 3 | bzip2 | 1 |
| rds | xdr | 3 | gzip | 58 |
| rds | xdr | 3 | none | 60 |
| rds | xdr | 3 | xz | 1 |
| rds | xdr | 3 | zstd | 1 |

ASCII files: 271 (CRLF line endings: `rdata/legacy-1.0.0/test_ascii_win_v2.rds`, `rdata/legacy-1.0.0/test_ascii_win_v3.rds`); asciihex (`ascii=NA`) files: 0. Native-binary ('B') files: 128 (all little-endian). Writer R versions seen: 3.0.2, 3.4.4, 3.5.2, 3.6.1, 3.6.2, 3.6.3, 4.0.0, 4.0.2, 4.0.3, 4.0.4, 4.0.5, 4.1.1, 4.1.2, 4.1.3, 4.2.0, 4.3.0, 4.3.1, 4.3.2, 4.4.0, 4.4.3, 4.5.2, 4.6.1. Native encodings seen (v3): CP1252, UTF-8.

## Coverage gaps and caveats

1. **asciihex (`save(ascii = NA)` / `serialize(ascii = NA)`): none found.** This format writes doubles
   with C99 `%a` (for example `0x1.199999999999ap+0`) under the same `A\n` header. GitHub code
   search for hex-float tokens in `extension:rds|rda|RData` files and for `ascii = NA` in R test
   code finds ASCII `.rda` files (the disrobe corpus and the RData.jl/rdata ascii files), but none
   in asciihex. No ASCII fixture here contains a hex-float line. A synthetic asciihex file could be made by
   rewriting the decimal doubles of an ASCII fixture with `float.hex()`. That file would not be
   R-written, and glibc `%a` differs from Python for 0 and subnormals.
2. **No big-endian native-binary (`B`) files.** All 128 `B` files are little-endian. rdata's tests
   build a big-endian header synthetically from bytes.
3. **No real long vectors** (length > 2^31−1, which uses the `-1` + two-int length encoding).
   The largest vectors are `seurat/scRep_example.rda` (dgCMatrix with 140,880 non-zeros, 2000×500,
   3.7 MB uncompressed), `zurds/zlib.rds` (raw vectors of up to 70,000 bytes),
   `sce/mouseBrainSubsetSCE.rda` (dense int 2314×30) and `rdatajl/data_v3/altrep.rda`
   (a 20,000-element factor).
4. **SCE:** no permissively licensed *real* SCE was found that has all of counts + logcounts +
   colData + rowData + reducedDim. `sce/mouseBrainSubsetSCE.rda` has everything except rowData columns (rowData has 0
   columns). `sce/scExample.rda` adds a rowData with 2 columns. `sce/simple_sce.rds` adds an altExp and metadata.
5. **Seurat v5 (`Assay5`):** the `seurat/v5/` objects are genuine Seurat 5.3.0 objects (R 4.5.2) built on
   small synthetic `rpois` counts. They have no reductions or graphs. The real-data Seurat objects
   (`pbmc_small`, `scRep_example`) use the v3-style `Assay` class.
6. **No `compact_realseq` ALTREP anywhere.** rdata's `test_altrep_compact_realseq*` objects are `seq(..., by = 1)`,
   which R stores as a plain REALSXP. RData.jl's `realseq*` objects are plain doubles too. The ALTREP classes present are
   `compact_intseq`, `deferred_string`, `wrap_real`, `wrap_integer`, `wrap_logical` and `wrap_string` (v3 files only;
   v2 files carry the expanded vectors).
7. **zstd** files come only from zurds (R 4.6.1 `zstdfile()`).
8. **Native-binary `.rda` (`RDB2`/`RDB3`)** in zurds are hand-assembled by its generator
   (`"RDB3\n"` + `serialize(as.pairlist(objs), NULL, xdr = FALSE)`), because `save()` cannot write them.
   The payload is still R's serializer output.
9. **CRLF ASCII** appears only in the two Windows-written rdata legacy `.rds` files.
10. rdata 1.1.0 **cannot parse** RAWSXP (24), PERSISTSXP (247), PACKAGESXP (248) and UNBOUNDVALUE (252).
   For `zurds/{raw,zlib,persistent,package_env,promise}.rds` and both `rdatajl/*/budworm_glm.rda`,
   the expected contents come from the generator code plus hexdump checks (noted in each entry).
   For closures, `...` (DOTSXP) and pairlists, rdata parses the file but cannot convert it.
11. The 580 `rdata/generated/*.rds` files are uncompressed (written with `serialize()` to a file connection).
    Compressed `.rds` coverage comes from rdata legacy, RData.jl, zurds, seurat and sce.

### Candidates considered but not taken

- **1-3-7/disrobe `corpus/r/objects/`** is a systematic corpus that includes native/ascii/bzip2/xz variants.
  Its license is proprietary ("Disrobe Source-Available License 1.1", with a non-compete clause), so nothing from it is included.
- **bernaferrari/R-rust** (`workspace-ascii.rda`, `-bzip2.rda`, `-xz.rda`): GPL-2.0.
- **R sources (wch/r-source mirror):** GPL, so not included. Two files are noted as header edge cases:
  `tests/nanbug.rda` is uncompressed `RDX2` written by R 1.4.0. `src/library/stats/tests/hills.rds` is
  xz v3 with native encoding `ANSI_X3.4-1968`.
- **mianaz/scConvert** (`inst/testdata/pbmc_small.rds` 1.48 MB, plus demo Seurat objects): GPL-3.0. Not inspected.
- **UCCC-BBSR/refmat-rs** (`testdata/{sce,seurat-v3,seurat-v4}.rds`, about 1 KB each): no license, so all rights reserved.
- **dcellwanger/CellTrails `data/exSCE.rda`** (Artistic-2.0, 31 KB, xz, R 3.4.3): a 2018-era SCE whose
  assays sit in a reference-class environment. rdata cannot convert it. It is an option if you want an old-SCE edge case.
- **ccb-hms/scDiagnostics `data/*_data.rda`** (Artistic-2.0): logcounts only (counts were removed) and ≥ 300 KB.
- **campbio/celda `data/sceCeldaCG.rda`** (MIT, 32 KB, xz, CP1252): simulated counts plus an altExp. An option.
- **singleCellTK `data/sceBatches.rda`** (MIT, 75 KB): uses the legacy `@reducedDims` slot, with PCA rows ≠ ncol. An option as a legacy edge case.
- **ncborcherding/escape `pbmc_small_ssGSEA.rds`** (MIT, 71 KB): pbmc_small plus one extra `Assay`. Redundant.
- **cellgeni/py8rds `seu_sketch*.rds`** (MIT, Seurat v5 with a sketch assay, about 15 MB) and `big_list.rds` (11 MB): too large.
- **AlexsLemonade/rOpenScPCA** test SCEs (1.2–2.2 MB): too large.
- **stuart-lab/signac `data/atac_small.rda`** (MIT, 437 KB, Seurat with ChromatinAssay): over the size guideline. Not inspected.
- **zurds `cve_top.rds` / `cve_nested.rds`**: hand-assembled CVE-2024-27322-style promise payloads
  (`system("echo zurds")`), not R serializer output, so skipped. zurds `lazyload*.rdb/.rdx`: lazy-load
  databases, not rds/rda.

## Licenses

| source | license | text in `LICENSES/` |
|---|---|---|
| vnmabus/rdata | MIT, © 2018 Rdata developers | `rdata-MIT.txt` |
| JuliaData/RData.jl | MIT, © 2013-2018 Douglas Bates, John Myles White, Alexey Stukalov, and other contributors | `RData.jl-MIT.md` |
| pedrobtz/zurds | MIT (`DESCRIPTION: MIT + file LICENSE`), © 2026 Pedro Baltazar | `zurds-MIT.md` |
| SeuratObject 5.4.0 | MIT (`MIT + file LICENSE`, stub "YEAR: 2021 / COPYRIGHT HOLDER: SeuratObject authors") | `SeuratObject-MIT.txt` (stub + standard MIT text) |
| scRepertoire 2.6.1 | MIT, © 2024 scRepertoire authors | `scRepertoire-MIT.md` |
| cellgeni/py8rds | MIT, © 2024 Cellular Genetics Informatics | `py8rds-MIT.txt` |
| singleCellTK 2.18.0 | MIT (`MIT + file LICENSE`, © 2016 David Jenkins, W. Evan Johnson, Joshua D. Campbell) | `singleCellTK-MIT.txt` (stub + standard MIT text) |
| BiocPy/rds2py | MIT, © 2022 Genentech, Inc. | `rds2py-MIT.txt` |

Notes on underlying data: pbmc_small comes from 10x Genomics' public 3k PBMC dataset.
mouseBrainSubsetSCE is from Zeisel et al. 2015 (GSE60361). scExample is from 10x pbmc4k.
simple_sce is from Tasic et al. 2016 via `scRNAseq::ReprocessedAllenData`. scRep_example is from the study cited in
scRepertoire's `R/data.R` (PubMed 33622974). Each file is distributed inside its package/repo under the license above. The
original public datasets have their own terms; check them before redistributing beyond test use.

## 1. `rdata/generated/` — vnmabus/rdata format matrix (580 files)

Source: <https://github.com/vnmabus/rdata/tree/0edddf4b692cf5c011976bd7b369ccea08aa5db5/rdata/tests/data/generated> (default branch
`develop`). Raw URL: `https://raw.githubusercontent.com/vnmabus/rdata/0edddf4b692cf5c011976bd7b369ccea08aa5db5/rdata/tests/data/generated/<file>`.
MIT. All files were written by R 4.4.3.

rdata's `conftest.py` (`create_dataset` fixture, run as `pytest --generate-datasets`) writes each test
object in 10 variants. It evaluates the docstring R code (lines starting `:::`) and then appends:

```r
# .rds variants: a raw serialization stream, NOT saveRDS(), so no compression
con <- file("<path>", "wb")
serialize(<name>, con, ascii = <TRUE for ascii>, xdr = <TRUE for xdr>, version = <2|3>)
close(con)
# .rda variants (there is no "binary" .rda: save() cannot write native binary)
save(<name>, file = "<path>", ascii = <TRUE for ascii>, version = <2|3>)
```

So `.rds` files are uncompressed. xdr `.rda` files are gzip (the `save()` default). ascii `.rda` files are
uncompressed (`save(compress = isTRUE(!ascii))`). The `ascii` variants use R's decimal ASCII format,
not asciihex. In each `.rda` the single object is bound to its own name (for example `test_logical`).
Variant legend, verified from the bytes of every file:

| variant suffix | container (magic) | format | ser. version | compression | writer R | native enc |
|---|---|---|---|---|---|---|
| `__ascii__version_2.rda` | rda (RDA2) | ascii | 2 | none | 4.4.3 | — |
| `__ascii__version_2.rds` | rds (none) | ascii | 2 | none | 4.4.3 | — |
| `__ascii__version_3.rda` | rda (RDA3) | ascii | 3 | none | 4.4.3 | UTF-8 |
| `__ascii__version_3.rds` | rds (none) | ascii | 3 | none | 4.4.3 | UTF-8 |
| `__binary__version_2.rds` | rds (none) | binary (little-endian) | 2 | none | 4.4.3 | — |
| `__binary__version_3.rds` | rds (none) | binary (little-endian) | 3 | none | 4.4.3 | UTF-8 |
| `__xdr__version_2.rda` | rda (RDX2) | xdr | 2 | gzip | 4.4.3 | — |
| `__xdr__version_2.rds` | rds (none) | xdr | 2 | none | 4.4.3 | — |
| `__xdr__version_3.rda` | rda (RDX3) | xdr | 3 | gzip | 4.4.3 | UTF-8 |
| `__xdr__version_3.rds` | rds (none) | xdr | 3 | none | 4.4.3 | UTF-8 |

#### `test_open_file`

Tests the different ways of passing a file to be opened by `parse_file`. (test `test_open_file` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_open_file <- c(1.0, 2.0, 3.0)
```

Expected value (rdata's assertion, verbatim):

```python
open_options: Mapping[
    str,
    Callable[[], AbstractContextManager[str | Path | AcceptableFile]],
] = {
    "string": lambda: nullcontext(dataset_path),
    "path": lambda: nullcontext(Path(dataset_path)),
    "opened file": lambda: Path(dataset_path).open("rb"),  # noqa: SIM115
}

for option_name, open_option in open_options.items():
    with subtests.test(msg=option_name):
        with open_option() as f:
            parsed = rdata.parser.parse_file(
                f,
                extension=f".{rdata_format_suffix}",
            )

        converted = rdata.conversion.convert(parsed)

        reference = np.array([1., 2., 3.])

        match rdata_format_suffix:
            case "rds":
                np.testing.assert_equal(
                    converted,
                    reference,
                )
            case "rda":
                np.testing.assert_equal(
                    converted,
                    {"test_open_file": reference},
                )
```

Files: `rdata/generated/test_open_file__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_open_file__binary__version_{2,3}.rds` (10 files, 29–91 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_logical`

Test parsing of logical vectors. (test `test_logical` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_logical <- c(TRUE, TRUE, FALSE, TRUE, FALSE)
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    np.array([True, True, False, True, False]),
)
```

Files: `rdata/generated/test_logical__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_logical__binary__version_{2,3}.rds` (10 files, 33–84 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_nullable_logical`

Test parsing of logical vectors containing NA. (test `test_nullable_logical` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_nullable_logical <- c(TRUE, FALSE, NA)
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_array_equal(
    dataset_object.data,
    np.array([True, False, True]),
)
np.testing.assert_array_equal(
    dataset_object.mask,
    np.array([False, False, True]),
)
```

Files: `rdata/generated/test_nullable_logical__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_nullable_logical__binary__version_{2,3}.rds` (10 files, 30–93 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_nullable_int`

Test parsing of integer vectors containing NA. (test `test_nullable_int` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_nullable_int <- c(313L, -12L, NA)
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_array_equal(
    dataset_object.data,
    np.array([313, -12, -2**31]),
)
np.testing.assert_array_equal(
    dataset_object.mask,
    np.array([False, False, True]),
)
```

Files: `rdata/generated/test_nullable_int__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_nullable_int__binary__version_{2,3}.rds` (10 files, 34–94 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_vector`

Test parsing of numerical vectors. (test `test_vector` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_vector <- c(1.0, 2.0, 3.0)
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    np.array([1.0, 2.0, 3.0]),
)
```

Files: `rdata/generated/test_vector__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_vector__binary__version_{2,3}.rds` (10 files, 29–88 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_named_vector`

Test parsing of vectors with names. (test `test_named_vector` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_named_vector <- c(a=1, b=2, c=3)
```

Expected value (rdata's assertion, verbatim):

```python
xarray.testing.assert_identical(
    dataset_object,
    xarray.DataArray(
        [1.0, 2.0, 3.0],
        coords=[["a", "b", "c"]],
    ),
)
```

Files: `rdata/generated/test_named_vector__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_named_vector__binary__version_{2,3}.rds` (10 files, 94–146 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_empty_string`

Test that the empty string is parsed correctly. (test `test_empty_string` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_empty_string <- ""
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_array_equal(
    dataset_object,
    np.array([""]),
)
```

Files: `rdata/generated/test_empty_string__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_empty_string__binary__version_{2,3}.rds` (10 files, 30–85 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_na_string`

Test that the NA string is parsed correctly. (test `test_na_string` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_na_string <- as.character(NA)
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_array_equal(
    dataset_object,
    np.array([None]),
)
```

Files: `rdata/generated/test_na_string__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_na_string__binary__version_{2,3}.rds` (10 files, 28–87 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_complex`

Test that complex numbers can be parsed. (test `test_complex` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_complex <- c(1 + 2i, 2, 0, 1 + 3i, -1i)
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    np.array([1 + 2j, 2, 0, 1 + 3j, -1j]),
)
```

Files: `rdata/generated/test_complex__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_complex__binary__version_{2,3}.rds` (10 files, 45–111 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_matrix`

Test that a matrix can be parsed. (test `test_matrix` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_matrix <- matrix(1:6, nrow=2, byrow=TRUE)
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    np.array([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ]),
)
```

Files: `rdata/generated/test_matrix__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_matrix__binary__version_{2,3}.rds` (10 files, 69–115 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_named_matrix`

Test that a named matrix can be parsed. (test `test_named_matrix` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
dimnames <- list(
    c("dim0_0", "dim0_1"),
    c("dim1_0", "dim1_1", "dim1_2")
)
test_named_matrix <- matrix(1:6, nrow=2, byrow=TRUE, dimnames=dimnames)
```

Expected value (rdata's assertion, verbatim):

```python
reference = xarray.DataArray(
    [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ],
    dims=["dim_0", "dim_1"],
    coords={
        "dim_0": ["dim0_0", "dim0_1"],
        "dim_1": ["dim1_0", "dim1_1", "dim1_2"],
    },
)

xarray.testing.assert_identical(
    dataset_object,
    reference,
)
```

Files: `rdata/generated/test_named_matrix__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_named_matrix__binary__version_{2,3}.rds` (10 files, 146–241 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_half_named_matrix`

Test that a named matrix with no name for a dim can be parsed. (test `test_half_named_matrix` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_half_named_matrix <- matrix(
    1:6,
    nrow=2,
    byrow=TRUE,
    dimnames=list(c("dim0_0", "dim0_1"))
)
```

Expected value (rdata's assertion, verbatim):

```python
reference = xarray.DataArray(
    [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ],
    dims=["dim_0", "dim_1"],
    coords={
        "dim_0": ["dim0_0", "dim0_1"],
    },
)

xarray.testing.assert_identical(
    dataset_object,
    reference,
)
```

Files: `rdata/generated/test_half_named_matrix__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_half_named_matrix__binary__version_{2,3}.rds` (10 files, 137–197 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_full_named_matrix`

Test that a named matrix with dim names can be parsed. (test `test_full_named_matrix` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
dimnames <- list(
    my_dim_0=c("dim0_0", "dim0_1"),
    my_dim_1=c("dim1_0", "dim1_1", "dim1_2")
)
test_full_named_matrix <- matrix(
    1:6,
    nrow=2,
    byrow=TRUE,
    dimnames=dimnames
)
```

Expected value (rdata's assertion, verbatim):

```python
reference = xarray.DataArray(
    [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ],
    dims=["my_dim_0", "my_dim_1"],
    coords={
        "my_dim_0": ["dim0_0", "dim0_1"],
        "my_dim_1": ["dim1_0", "dim1_1", "dim1_2"],
    },
)

xarray.testing.assert_identical(
    dataset_object,
    reference,
)
```

Files: `rdata/generated/test_full_named_matrix__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_full_named_matrix__binary__version_{2,3}.rds` (10 files, 169–314 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_empty_list`

Test parsing the empty list. (test `test_empty_list` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_empty_list <- list()
```

Expected value (rdata's assertion, verbatim):

```python
assert dataset_object == []
```

Files: `rdata/generated/test_empty_list__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_empty_list__binary__version_{2,3}.rds` (10 files, 22–81 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_empty_named_list`

Test parsing the empty named list. (test `test_empty_named_list` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_empty_named_list <- setNames(list(), character(0))
```

Expected value (rdata's assertion, verbatim):

```python
assert dataset_object == {}
```

Files: `rdata/generated/test_empty_named_list__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_empty_named_list__binary__version_{2,3}.rds` (10 files, 55–111 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_list`

Test that list can be parsed. (test `test_list` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_list <- list(1, c("a", "b", "c"), c(2, 3), "hi")
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    [
        np.array([1.0]),
        np.array(["a", "b", "c"]),
        np.array([2.0, 3.0]),
        np.array(["hi"]),
    ],
)
```

Files: `rdata/generated/test_list__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_list__binary__version_{2,3}.rds` (10 files, 94–137 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_list_attrs`

Test that lists accept attributes. (test `test_list_attrs` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_list_attrs <- list("list", 5)
attr(test_list_attrs, "my_attr") <- "attr_value"
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    [np.array(["list"]), np.array([5])],
)
```

Files: `rdata/generated/test_list_attrs__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_list_attrs__binary__version_{2,3}.rds` (10 files, 104–154 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_file`

Test that external pointers can be parsed. (test `test_file` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_file <- file()
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    np.array([3]),
)
```

Files: `rdata/generated/test_file__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_file__binary__version_{2,3}.rds` (10 files, 118–189 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_expression`

Test that expressions can be parsed. (test `test_expression` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_expression <- expression(base^exponent)
```

Expected value (rdata's assertion, verbatim):

```python
assert dataset_object == rdata.conversion.RExpression([
    rdata.conversion.RLanguage(
        ["^", "base", "exponent"],
        attributes={},
    ),
])
```

Files: `rdata/generated/test_expression__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_expression__binary__version_{2,3}.rds` (10 files, 82–132 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_builtin`

Test that builtin functions can be parsed. (test `test_builtin` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_builtin <- abs
```

Expected value (rdata's assertion, verbatim):

```python
assert dataset_object == rdata.conversion.RBuiltin(name="abs")
```

Files: `rdata/generated/test_builtin__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_builtin__binary__version_{2,3}.rds` (10 files, 25–82 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_minimal_function_uncompiled`

Test that a minimal function can be parsed. (test `test_minimal_function_uncompiled` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
options(keep.source = TRUE)
test_minimal_function_uncompiled <- function() NULL
```

Expected value (rdata's assertion, verbatim):

```python
assert isinstance(
    dataset_object,
    rdata.conversion.RFunction,
)

np.testing.assert_equal(dataset_object.environment, ChainMap({}))
np.testing.assert_equal(dataset_object.formals, None)
np.testing.assert_equal(dataset_object.body, None)
np.testing.assert_equal(
    dataset_object.source,
    "test_minimal_function_uncompiled <- function() NULL\n",
)
```

Files: `rdata/generated/test_minimal_function_uncompiled__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_minimal_function_uncompiled__binary__version_{2,3}.rds` (10 files, 212–440 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_minimal_function`

Test that a minimal function (compiled) can be parsed. (test `test_minimal_function` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
options(keep.source = TRUE)
test_minimal_function <- function() NULL
library(compiler)
test_minimal_function <- cmpfun(test_minimal_function)
```

Expected value (rdata's assertion, verbatim):

```python
assert isinstance(
    dataset_object,
    rdata.conversion.RFunction,
)

np.testing.assert_equal(dataset_object.environment, ChainMap({}))
np.testing.assert_equal(dataset_object.formals, None)

converted_body = dataset_object.body

assert isinstance(
    converted_body,
    rdata.conversion.RBytecode,
)

np.testing.assert_equal(converted_body.code, np.array([12, 17, 1]))
np.testing.assert_equal(converted_body.attributes, {})

np.testing.assert_equal(
    dataset_object.source,
    "test_minimal_function <- function() NULL\n",
)
```

Files: `rdata/generated/test_minimal_function__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_minimal_function__binary__version_{2,3}.rds` (10 files, 268–639 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_empty_function_uncompiled`

Test that a simple function can be parsed. (test `test_empty_function_uncompiled` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
options(keep.source = TRUE)
test_empty_function_uncompiled <- function() {}
```

Expected value (rdata's assertion, verbatim):

```python
assert isinstance(
    dataset_object,
    rdata.conversion.RFunction,
)

np.testing.assert_equal(dataset_object.environment, ChainMap({}))
np.testing.assert_equal(dataset_object.formals, None)
assert isinstance(dataset_object.body, rdata.conversion.RLanguage)
np.testing.assert_equal(
    dataset_object.source,
    "test_empty_function_uncompiled <- function() {}\n",
)
```

Files: `rdata/generated/test_empty_function_uncompiled__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_empty_function_uncompiled__binary__version_{2,3}.rds` (10 files, 265–663 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_empty_function`

Test that a simple function (compiled) can be parsed. (test `test_empty_function` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
options(keep.source = TRUE)
test_empty_function <- function() {}
library(compiler)
test_empty_function <- cmpfun(test_empty_function)
```

Expected value (rdata's assertion, verbatim):

```python
assert isinstance(
    dataset_object,
    rdata.conversion.RFunction,
)

np.testing.assert_equal(dataset_object.environment, ChainMap({}))
np.testing.assert_equal(dataset_object.formals, None)

converted_body = dataset_object.body

assert isinstance(
    converted_body,
    rdata.conversion.RBytecode,
)

np.testing.assert_equal(converted_body.code, np.array([12, 17, 1]))
np.testing.assert_equal(converted_body.attributes, {})

np.testing.assert_equal(
    dataset_object.source,
    "test_empty_function <- function() {}\n",
)
```

Files: `rdata/generated/test_empty_function__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_empty_function__binary__version_{2,3}.rds` (10 files, 316–885 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_function`

Test that functions can be parsed. (test `test_function` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
options(keep.source = TRUE)
test_function <- function() {print("Hello")}
library(compiler)
test_function <- cmpfun(test_function)
```

Expected value (rdata's assertion, verbatim):

```python
assert isinstance(
    dataset_object,
    rdata.conversion.RFunction,
)

np.testing.assert_equal(dataset_object.environment, ChainMap({}))
np.testing.assert_equal(dataset_object.formals, None)

converted_body = dataset_object.body

assert isinstance(
    converted_body,
    rdata.conversion.RBytecode,
)

np.testing.assert_equal(
    converted_body.code,
    np.array([12, 23, 1, 34, 4, 38, 2, 1]),
)
np.testing.assert_equal(converted_body.attributes, {})

np.testing.assert_equal(
    dataset_object.source,
    "test_function <- function() {print(\"Hello\")}\n",
)
```

Files: `rdata/generated/test_function__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_function__binary__version_{2,3}.rds` (10 files, 375–1158 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_function_arg`

Test that functions can be parsed. (test `test_function_arg` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
options(keep.source = TRUE)
test_function_arg <- function(a) {print(a)}
library(compiler)
test_function_arg <- cmpfun(test_function_arg)
```

Expected value (rdata's assertion, verbatim):

```python
assert isinstance(
    dataset_object,
    rdata.conversion.RFunction,
)

np.testing.assert_equal(dataset_object.environment, ChainMap({}))
np.testing.assert_equal(dataset_object.formals, {"a": NotImplemented})

converted_body = dataset_object.body

assert isinstance(
    converted_body,
    rdata.conversion.RBytecode,
)

np.testing.assert_equal(
    converted_body.code,
    np.array([12, 23, 1, 29, 4, 38, 2, 1]),
)
np.testing.assert_equal(converted_body.attributes, {})

np.testing.assert_equal(
    dataset_object.source,
    "test_function_arg <- function(a) {print(a)}\n",
)
```

Files: `rdata/generated/test_function_arg__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_function_arg__binary__version_{2,3}.rds` (10 files, 401–1414 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_empty_dataframe`

Test conversion of an empty dataframe. (test `test_empty_dataframe` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_empty_dataframe <- data.frame()
```

Expected value (rdata's assertion, verbatim):

```python
pd.testing.assert_frame_equal(
    dataset_object,
    pd.DataFrame(columns=[], index=np.array([], dtype=np.int32)),
)
```

Files: `rdata/generated/test_empty_dataframe__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_empty_dataframe__binary__version_{2,3}.rds` (10 files, 117–189 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_empty_dataframe_without_names`

Test conversion of an empty dataframe without names. (test `test_empty_dataframe_without_names` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_empty_dataframe_without_names <- data.frame()
attr(test_empty_dataframe_without_names, "names") <- NULL
```

Expected value (rdata's assertion, verbatim):

```python
pd.testing.assert_frame_equal(
    dataset_object,
    pd.DataFrame(index=np.array([], dtype=np.int32)),
)
```

Files: `rdata/generated/test_empty_dataframe_without_names__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_empty_dataframe_without_names__binary__version_{2,3}.rds` (10 files, 106–176 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_dataframe`

Test dataframe conversion. (test `test_dataframe` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_dataframe <- data.frame(
    class=factor(c("a", "b", "b")),
    value=c(1L, 2L, 3L)
)
```

Expected value (rdata's assertion, verbatim):

```python
pd.testing.assert_frame_equal(
    dataset_object,
    pd.DataFrame(
        {
            "class": pd.Categorical(
                ["a", "b", "b"],
            ),
            "value": pd.Series(
                [1, 2, 3],
                dtype=pd.Int32Dtype(),
            ).array,
        },
        index=pd.RangeIndex(start=1, stop=4),
    ),
)
```

Files: `rdata/generated/test_dataframe__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_dataframe__binary__version_{2,3}.rds` (10 files, 176–327 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_dataframe_rownames`

Test dataframe conversion. (test `test_dataframe_rownames` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_dataframe_rownames <- data.frame(
    class=factor(c("a", "b", "b")),
    value=c(1L, 2L, 3L),
    row.names=c("Madrid", "Frankfurt", "Herzberg am Harz")
)
```

Expected value (rdata's assertion, verbatim):

```python
pd.testing.assert_frame_equal(
    dataset_object,
    pd.DataFrame(
        {
            "class": pd.Categorical(
                ["a", "b", "b"],
            ),
            "value": pd.Series(
                [1, 2, 3],
                dtype=pd.Int32Dtype(),
            ).array,
        },
        index=("Madrid", "Frankfurt", "Herzberg am Harz"),
    ),
)
```

Files: `rdata/generated/test_dataframe_rownames__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_dataframe_rownames__binary__version_{2,3}.rds` (10 files, 206–398 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_dataframe_int_rownames`

Test dataframe conversion. (test `test_dataframe_int_rownames` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_dataframe_int_rownames <- data.frame(
    col1=c(10, 20, 30),
    row.names=c(3L, 6L, 9L)
)
```

Expected value (rdata's assertion, verbatim):

```python
index = np.array([3, 6, 9], dtype=np.int32)
reference = pd.DataFrame(
    {
        "col1": pd.Series(
            [10., 20., 30.],
            dtype=float, index=index),
    },
    index=index,
)
pd.testing.assert_frame_equal(dataset_object, reference)
```

Files: `rdata/generated/test_dataframe_int_rownames__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_dataframe_int_rownames__binary__version_{2,3}.rds` (10 files, 149–230 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_dataframe_range_rownames`

Test dataframe conversion. (test `test_dataframe_range_rownames` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_dataframe_range_rownames <- data.frame(
    col1=c(10, 20, 30),
    row.names=2:4
)
```

Expected value (rdata's assertion, verbatim):

```python
index = pd.Index([2, 3, 4], dtype=np.int32)
reference = pd.DataFrame(
    {
        "col1": pd.Series(
            [10., 20., 30.],
            dtype=float, index=index),
    },
    index=index,
)
pd.testing.assert_frame_equal(dataset_object, reference)
```

Files: `rdata/generated/test_dataframe_range_rownames__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_dataframe_range_rownames__binary__version_{2,3}.rds` (10 files, 150–301 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_dataframe_dtypes`

Test dataframe conversion. (test `test_dataframe_dtypes` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_dataframe_dtypes <- data.frame(
    int=c(10L, 20L, 30L),
    float=c(1.1, 2.2, 3.3),
    string=c("x", "y", "z"),
    bool=as.logical(c(1, 0, 1)),
    complex=c(4+5i, 6+7i, 8+9i)
)
```

Expected value (rdata's assertion, verbatim):

```python
index = pd.RangeIndex(1, 4)
reference = pd.DataFrame(
    {
        "int": pd.Series(
            [10, 20, 30],
            dtype=pd.Int32Dtype(), index=index),
        "float": pd.Series(
            [1.1, 2.2, 3.3],
            dtype=float, index=index),
        "string": pd.Series(
            ["x", "y", "z"],
            dtype=pd.StringDtype(), index=index),
        "bool": pd.Series(
            [True, False, True],
            dtype=pd.BooleanDtype(), index=index),
        "complex": pd.Series(
            [4+5j, 6+7j, 8+9j],
            dtype=complex, index=index),
    },
    index=index,
)
pd.testing.assert_frame_equal(dataset_object, reference)
```

Files: `rdata/generated/test_dataframe_dtypes__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_dataframe_dtypes__binary__version_{2,3}.rds` (10 files, 239–380 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_dataframe_dtypes_with_na`

Test dataframe conversion. (test `test_dataframe_dtypes_with_na` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_dataframe_dtypes_with_na <- data.frame(
    int=c(10L, 20L, 30L, NA),
    float=c(1.1, 2.2, 3.3, NA),
    string=c("x", "y", "z", NA),
    bool=as.logical(c(1, 0, 1, NA)),
    complex=c(4+5i, 6+7i, 8+9i, NA)
)
```

Expected value (rdata's assertion, verbatim):

```python
index = pd.RangeIndex(1, 5)
reference = pd.DataFrame(
    {
        "int": pd.Series(
            [10, 20, 30, pd.NA],
            dtype=pd.Int32Dtype(), index=index),
        "float": pd.Series(
            [1.1, 2.2, 3.3, R_FLOAT_NA],
            dtype=float, index=index),
        "string": pd.Series(
            ["x", "y", "z", pd.NA],
            dtype=pd.StringDtype(), index=index),
        "bool": pd.Series(
            [True, False, True, pd.NA],
            dtype=pd.BooleanDtype(), index=index),
        "complex": pd.Series(
            [4+5j, 6+7j, 8+9j, R_FLOAT_NA],
            dtype=complex, index=index),
    },
    index=index,
)

with np.errstate(invalid="ignore"):
    # Comparing complex arrays with R_FLOAT_NA gives warning
    pd.testing.assert_frame_equal(dataset_object, reference)
```

Files: `rdata/generated/test_dataframe_dtypes_with_na__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_dataframe_dtypes_with_na__binary__version_{2,3}.rds` (10 files, 264–420 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_dataframe_float_with_na_nan`

Test dataframe conversion. (test `test_dataframe_float_with_na_nan` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_dataframe_float_with_na_nan <- data.frame(
    float=c(1.1, 2.2, 3.3, NA, NaN, Inf, -Inf)
)
```

Expected value (rdata's assertion, verbatim):

```python
index = pd.RangeIndex(1, 8)
reference = pd.DataFrame(
    {
        "float": pd.Series(
            [1.1, 2.2, 3.3, R_FLOAT_NA, np.nan, np.inf, -np.inf],
            dtype=float, index=index),
    },
    index=index,
)
pd.testing.assert_frame_equal(dataset_object, reference)
```

Files: `rdata/generated/test_dataframe_float_with_na_nan__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_dataframe_float_with_na_nan__binary__version_{2,3}.rds` (10 files, 175–255 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_factor`

Test factor conversion. (test `test_factor` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_factor <- factor(c("a", "b", "b"))
```

Expected value (rdata's assertion, verbatim):

```python
pd.testing.assert_frame_equal(
    pd.DataFrame(dataset_object),
    pd.DataFrame(pd.Categorical(["a", "b", "b"])),
)
```

Files: `rdata/generated/test_factor__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_factor__binary__version_{2,3}.rds` (10 files, 112–173 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_ts`

Test time series conversion. (test `test_ts` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_ts <- ts(c(1, 2, 3), start=c(2000, 3), frequency = 12)
```

Expected value (rdata's assertion, verbatim):

```python
pd.testing.assert_series_equal(
    dataset_object,
    pd.Series({
        2000 + Fraction(2, 12): 1.0,
        2000 + Fraction(3, 12): 2.0,
        2000 + Fraction(4, 12): 3.0,
    }),
)
```

Files: `rdata/generated/test_ts__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_ts__binary__version_{2,3}.rds` (10 files, 121–178 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_s4`

Test parsing of S4 classes. (test `test_s4` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
setClass("Person", representation(name = "character", age = "numeric"))
test_s4 <- new("Person", name = "Carlos", age = 28)
```

Expected value (rdata's assertion, verbatim):

```python
assert len(recwarn) == 1
w = recwarn.pop(UserWarning)
assert issubclass(w.category, UserWarning)
assert str(w.message).startswith("Missing constructor")

np.testing.assert_equal(
    dataset_object,
    SimpleNamespace(
        age=np.array(28),
        name=["Carlos"],
        **{"class": ["Person"]},
    ),
)
```

Files: `rdata/generated/test_s4__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_s4__binary__version_{2,3}.rds` (10 files, 139–237 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_environment_global_default`

Test parsing of environments, with default global environment. (test `test_environment_global_default` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_environment_global_default <- new.env()
assign("string", "test", envir = test_environment_global_default)
```

Expected value (rdata's assertion, verbatim):

```python
dict_env = {"string": np.array(["test"])}
empty_global_env: dict[str, np.typing.NDArray[np.str_]] = {}

assert dataset_object == ChainMap(dict_env, ChainMap(empty_global_env))
```

Files: `rdata/generated/test_environment_global_default__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_environment_global_default__binary__version_{2,3}.rds` (10 files, 118–264 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_environment_global_argument`

Test parsing of environments, with default global environment. (test `test_environment_global_argument` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_environment_global_argument <- new.env()
assign("string", "test", envir = test_environment_global_argument)
```

Read with rdata kwargs: `{'global_environment': "{'global': ['test']}"}`

Expected value (rdata's assertion, verbatim):

```python
dict_env = {"string": ["test"]}
global_env = {"global": ["test"]}

assert dataset_object == ChainMap(dict_env, ChainMap(global_env))
```

Files: `rdata/generated/test_environment_global_argument__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_environment_global_argument__binary__version_{2,3}.rds` (10 files, 117–265 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_emptyenv`

Test parsing the empty environment. (test `test_emptyenv` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_emptyenv <- emptyenv()
```

Expected value (rdata's assertion, verbatim):

```python
assert dataset_object == ChainMap({})
```

Files: `rdata/generated/test_emptyenv__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_emptyenv__binary__version_{2,3}.rds` (10 files, 18–78 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_altrep_compact_intseq`

Test alternative representation of sequences of ints. (test `test_altrep_compact_intseq` in `rdata/tests/test_read.py`)

Note: Bytes: v3 files start with ALTREP_SXP (238) whose class pairlist names `compact_intseq` (package `base`). The state is the REALSXP `c(1000, 0, 1)` (length, start, step). v2 files hold the expanded 1000-element INTSXP, which is why they are about 4 KB versus about 130 B.

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_altrep_compact_intseq <- 0:999
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    np.arange(1000),
)
```

Files: `rdata/generated/test_altrep_compact_intseq__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_altrep_compact_intseq__binary__version_{2,3}.rds` (10 files, 109–4022 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_altrep_compact_intseq_asymmetric`

Test alternative representation of sequences of ints. (test `test_altrep_compact_intseq_asymmetric` in `rdata/tests/test_read.py`)

Note: v3: ALTREP `compact_intseq` (state `c(11, -5, 1)`). v2: an expanded INTSXP.

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_altrep_compact_intseq_asymmetric <- -5:5
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    np.arange(-5, 6),
)
```

Files: `rdata/generated/test_altrep_compact_intseq_asymmetric__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_altrep_compact_intseq_asymmetric__binary__version_{2,3}.rds` (10 files, 51–172 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_altrep_compact_realseq`

Test alternative representation of sequences of ints. (test `test_altrep_compact_realseq` in `rdata/tests/test_read.py`)

Note: **Not ALTREP in the files.** `seq(0, 999, by = 1)` is a plain REALSXP (type 14) in v2 *and* v3.

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_altrep_compact_realseq <- seq(0, 999, by=1)
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    np.arange(1000.0),
)
```

Files: `rdata/generated/test_altrep_compact_realseq__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_altrep_compact_realseq__binary__version_{2,3}.rds` (10 files, 2077–8031 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_altrep_compact_realseq_asymmetric`

Test alternative representation of sequences of reals. (test `test_altrep_compact_realseq_asymmetric` in `rdata/tests/test_read.py`)

Note: **Not ALTREP in the files.** A plain REALSXP in v2 and v3.

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_altrep_compact_realseq_asymmetric <- seq(-5, 5, by=1)
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    np.arange(-5.0, 6.0),
)
```

Files: `rdata/generated/test_altrep_compact_realseq_asymmetric__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_altrep_compact_realseq_asymmetric__binary__version_{2,3}.rds` (10 files, 51–138 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_altrep_deferred_string`

Test alternative representation of deferred strings. (test `test_altrep_deferred_string` in `rdata/tests/test_read.py`)

Note: v3: ALTREP `deferred_string` (its state holds the source REALSXP). v2: an expanded STRSXP.

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_altrep_deferred_string <- as.character(
    c(1, 2.3, 10000, 1e+05, -10000, -1e+05, 0.001, 1e-04, 1e-05)
)
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_array_equal(
    dataset_object,
    np.array([
        "1", "2.3", "10000",
        "1e+05", "-10000", "-1e+05",
        "0.001", "1e-04", "1e-05",
    ]),
)
```

Files: `rdata/generated/test_altrep_deferred_string__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_altrep_deferred_string__binary__version_{2,3}.rds` (10 files, 131–217 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_altrep_wrap_real`

Test alternative representation of wrap_real. (test `test_altrep_wrap_real` in `rdata/tests/test_read.py`)

Note: v3: ALTREP `wrap_real`. v2: a plain REALSXP.

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_altrep_wrap_real <- .Internal(wrap_meta(3, 0, 0))
```

Expected value (rdata's assertion, verbatim):

```python
parsed = dataset_object_parsed
assert parsed.info.type == rdata.parser.RObjectType.REAL  # sanity check
assert not parsed.info.object
assert not parsed.info.attributes
assert parsed.attributes is None

np.testing.assert_equal(
    dataset_object,
    np.array([3]),
)
```

Files: `rdata/generated/test_altrep_wrap_real__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_altrep_wrap_real__binary__version_{2,3}.rds` (10 files, 25–155 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_altrep_wrap_real_attributes`

Test alternative representation of wrap_real with attributes. (test `test_altrep_wrap_real_attributes` in `rdata/tests/test_read.py`)

Note: v3: ALTREP `wrap_real` with an attribute `foo`. v2: a plain REALSXP with attributes.

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_altrep_wrap_real_attributes <- .Internal(
    wrap_meta(c(1, 2, 3), 0, 0)
)
attr(test_altrep_wrap_real_attributes, "foo") <- "bar"
```

Expected value (rdata's assertion, verbatim):

```python
parsed = dataset_object_parsed
assert parsed.info.type == rdata.parser.RObjectType.REAL  # sanity check
assert not parsed.info.object
assert parsed.info.attributes
assert parsed.attributes is not None
assert parsed.attributes.tag is not None
assert parsed.attributes.tag.value.value == b"foo"
assert parsed.attributes.value[0].value[0].value == b"bar"

np.testing.assert_equal(
    dataset_object,
    np.array([1., 2., 3.]),
)
```

Files: `rdata/generated/test_altrep_wrap_real_attributes__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_altrep_wrap_real_attributes__binary__version_{2,3}.rds` (10 files, 72–208 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_altrep_wrap_real_class_attribute`

Test altrep of wrap_real with class attribute. (test `test_altrep_wrap_real_class_attribute` in `rdata/tests/test_read.py`)

Note: v3: the ALTREP header flags are `0x1EE` (object bit set), with class `Date`.

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_altrep_wrap_real_class_attribute <- .Internal(
    wrap_meta(c(1, 2, 3), 0, 0)
)
attr(test_altrep_wrap_real_class_attribute, "class") <- "Date"
```

Expected value (rdata's assertion, verbatim):

```python
parsed = dataset_object_parsed
assert parsed.info.type == rdata.parser.RObjectType.REAL  # sanity check
assert parsed.info.object
assert parsed.info.attributes
assert parsed.attributes is not None
assert parsed.attributes.tag is not None
assert parsed.attributes.tag.value.value == b"class"
assert parsed.attributes.value[0].value[0].value == b"Date"

np.testing.assert_equal(
    dataset_object,
    np.array([1., 2., 3.]),
)
```

Files: `rdata/generated/test_altrep_wrap_real_class_attribute__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_altrep_wrap_real_class_attribute__binary__version_{2,3}.rds` (10 files, 75–216 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_altrep_wrap_string`

Test alternative representation of wrap_string. (test `test_altrep_wrap_string` in `rdata/tests/test_read.py`)

Note: v3: ALTREP `wrap_string`.

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_altrep_wrap_string <- .Internal(wrap_meta("Hello", 0, 0))
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_array_equal(
    dataset_object,
    np.array(["Hello"]),
)
```

Files: `rdata/generated/test_altrep_wrap_string__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_altrep_wrap_string__binary__version_{2,3}.rds` (10 files, 35–173 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_altrep_wrap_logical`

Test alternative representation of wrap_logical. (test `test_altrep_wrap_logical` in `rdata/tests/test_read.py`)

Note: v3: ALTREP `wrap_logical`.

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_altrep_wrap_logical <- .Internal(wrap_meta(TRUE, 0, 0))
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    np.array([True]),
)
```

Files: `rdata/generated/test_altrep_wrap_logical__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_altrep_wrap_logical__binary__version_{2,3}.rds` (10 files, 25–162 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_ascii_characters`

Test reading string with all ascii printable characters. (test `test_ascii_characters` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_ascii_characters <- "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\v\f\r\n"
```

Expected value (rdata's assertion, verbatim):

```python
assert dataset_object == "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\v\f\r\n"  # noqa: E501
```

Files: `rdata/generated/test_ascii_characters__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_ascii_characters__binary__version_{2,3}.rds` (10 files, 132–207 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_nan_inf`

Test reading nan and inf. (test `test_nan_inf` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_nan_inf <- c(0, -0, NaN, Inf, -Inf)
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_equal(
    dataset_object,
    np.array([0., -0., np.nan, np.inf, -np.inf]),
)
np.testing.assert_equal(
    np.signbit(dataset_object),
    np.array([False, True, False, False, True]),
)
```

Files: `rdata/generated/test_nan_inf__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_nan_inf__binary__version_{2,3}.rds` (10 files, 41–95 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_encoding_utf8`

Test of character vector with UTF-8 encoding. (test `test_encoding_utf8` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_encoding_utf8 <- "e\xc4\xa5o\xc5\x9dan\xc4\x9do \xc4\x89iu\xc4\xb5a\xc5\xadde"
Encoding(test_encoding_utf8) <- "UTF-8"
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_array_equal(
    dataset_object,
    np.array(["eĥoŝanĝo ĉiuĵaŭde"]),
)
```

Files: `rdata/generated/test_encoding_utf8__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_encoding_utf8__binary__version_{2,3}.rds` (10 files, 53–148 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_encoding_latin1`

Test of character vector with latin1 encoding. (test `test_encoding_latin1` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_encoding_latin1 <- "ca\xf1\xf3n"
Encoding(test_encoding_latin1) <- "latin1"
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_array_equal(
    dataset_object,
    np.array(["cañón"]),
)
```

Files: `rdata/generated/test_encoding_latin1__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_encoding_latin1__binary__version_{2,3}.rds` (10 files, 35–98 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_encoding_bytes`

Test of character vector with bytes encoding. (test `test_encoding_bytes` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_encoding_bytes <- "reba\xf1o"
Encoding(test_encoding_bytes) <- "bytes"
```

Expected value (rdata's assertion, verbatim):

```python
np.testing.assert_array_equal(
    dataset_object,
    np.array([b"reba\xf1o"]),
)
```

Files: `rdata/generated/test_encoding_bytes__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_encoding_bytes__binary__version_{2,3}.rds` (10 files, 36–96 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_encoding_unknown`

Test of character vector with unknown encoding. (test `test_encoding_unknown` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
latin1_str <- "\xcd\xf1igo"
Encoding(latin1_str) <- "latin1"
test_encoding_unknown <-  enc2native(latin1_str)
Encoding(test_encoding_unknown) <- "unknown"
```

Expected value (rdata's assertion, verbatim):

```python
match rdata_format_version:
    case 2:
        # There are 2 possibilities, depending on the native encoding
        reference_utf8 = np.array([b"\xc3\x8d\xc3\xb1igo"])
        reference_latin1 = np.array([b"\xcd\xf1igo"])

        matches_utf8 = np.equal(dataset_object, reference_utf8)
        matches_latin1 = np.equal(dataset_object, reference_latin1)

        assert matches_utf8 or matches_latin1
    case 3:
        np.testing.assert_array_equal(
            dataset_object,
            np.array(["Íñigo"]),
        )
```

Files: `rdata/generated/test_encoding_unknown__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_encoding_unknown__binary__version_{2,3}.rds` (10 files, 37–103 bytes). rdata 1.1.0: 10× parse+convert OK.

#### `test_namespace`

Test of namespace objects. (test `test_namespace` in `rdata/tests/test_read.py`)

R code that generated it (docstring `:::` lines; then serialized/saved as above):

```r
test_namespace <- asNamespace("stats")
```

Expected value (rdata's assertion, verbatim):

```python
assert isinstance(dataset_object, rdata.conversion.RNamespace)
assert dataset_object.name == "stats"
assert isinstance(dataset_object.version, str)
```

Files: `rdata/generated/test_namespace__{ascii,xdr}__version_{2,3}.{rds,rda}` + `test_namespace__binary__version_{2,3}.rds` (10 files, 52–105 bytes). rdata 1.1.0: 10× parse+convert OK.

## 2. `rdata/legacy-1.0.0/` — vnmabus/rdata 1.0.0 test files (69 files)

Source: <https://github.com/vnmabus/rdata/tree/567a2af915b795ddc14394bbb63aa0c1f912f4be/rdata/tests/data> (tag `1.0.0`). HEAD later
replaced these with the generated matrix. Only `test_dataframe.rda`, `test_dataframe.rds` and
`test_vector.rda` are still at HEAD, byte-identical. MIT. These were written ad hoc with
`save()`/`saveRDS()` defaults (gzip) by R 3.0.2 through 4.4.3. They add what the HEAD matrix lacks: gzip-compressed
`.rds`, old writer versions, Windows-written ASCII (CRLF `.rds`, CP1252 native encoding), and the
v2/v3 encoding mix. Expected values are quoted from `rdata/tests/test_rdata.py` at that tag.

#### `test_altrep_compact_intseq` — `test_altrep_compact_intseq.rda`

Test alternative representation of sequences of ints.

- `rdata/legacy-1.0.0/test_altrep_compact_intseq.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8; 126 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_altrep_compact_intseq <- 0:999
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_altrep_compact_intseq.rda")

np.testing.assert_equal(data, {
    "test_altrep_compact_intseq": np.arange(1000),
})
```

#### `test_altrep_compact_intseq_asymmetric` — `test_altrep_compact_intseq_asymmetric.rda`

Test alternative representation of sequences of ints.

- `rdata/legacy-1.0.0/test_altrep_compact_intseq_asymmetric.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.3.1 · enc UTF-8; 141 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_altrep_compact_intseq_asymmetric <- -5:5
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(
    TESTDATA_PATH / "test_altrep_compact_intseq_asymmetric.rda",
)

np.testing.assert_equal(data, {
    "test_altrep_compact_intseq_asymmetric": np.arange(-5, 6),
})
```

#### `test_altrep_compact_realseq` — `test_altrep_compact_realseq.rda`

Test alternative representation of sequences of ints.

- `rdata/legacy-1.0.0/test_altrep_compact_realseq.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8; 128 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_altrep_compact_realseq <- seq(0, 999, by=1)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(
    TESTDATA_PATH / "test_altrep_compact_realseq.rda",
)

np.testing.assert_equal(data, {
    "test_altrep_compact_realseq": np.arange(1000.0),
})
```

#### `test_altrep_compact_realseq_asymmetric` — `test_altrep_compact_realseq_asymmetric.rda`

Test alternative representation of sequences of ints.

- `rdata/legacy-1.0.0/test_altrep_compact_realseq_asymmetric.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.3.1 · enc UTF-8; 144 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_altrep_compact_realseq_asymmetric <- seq(-5, 5, by=1)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(
    TESTDATA_PATH / "test_altrep_compact_realseq_asymmetric.rda",
)

np.testing.assert_equal(data, {
    "test_altrep_compact_realseq_asymmetric": np.arange(-5.0, 6.0),
})
```

#### `test_altrep_deferred_string` — `test_altrep_deferred_string.rda`

Test alternative representation of deferred strings.

- `rdata/legacy-1.0.0/test_altrep_deferred_string.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8; 174 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_altrep_deferred_string <- as.character(
    c(1, 2.3, 10000, 1e+05, -10000, -1e+05, 0.001, 1e-04, 1e-05)
)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(
    TESTDATA_PATH / "test_altrep_deferred_string.rda",
)

np.testing.assert_equal(data, {
    "test_altrep_deferred_string": [
        "1", "2.3", "10000",
        "1e+05", "-10000", "-1e+05",
        "0.001", "1e-04", "1e-05",
    ],
})
```

#### `test_altrep_wrap_logical` — `test_altrep_wrap_logical.rda`

Test alternative representation of wrap_logical.

- `rdata/legacy-1.0.0/test_altrep_wrap_logical.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8; 125 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_altrep_wrap_logical <- .Internal(wrap_meta(TRUE, 0, 0))
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_altrep_wrap_logical.rda")

np.testing.assert_equal(data, {
    "test_altrep_wrap_logical": [True],
})
```

#### `test_altrep_wrap_real` — `test_altrep_wrap_real.rda`

Test alternative representation of wrap_real.

- `rdata/legacy-1.0.0/test_altrep_wrap_real.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8; 125 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_altrep_wrap_real <- .Internal(wrap_meta(3, 0, 0))
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
parsed = rdata.parser.parse_file(
    TESTDATA_PATH / "test_altrep_wrap_real.rda",
)
obj = parsed.object.value[0]  # taking value as it's an rda file
assert obj.info.type == rdata.parser.RObjectType.REAL  # sanity check
assert not obj.info.object
assert not obj.info.attributes
assert obj.attributes is None

data = rdata.conversion.convert(parsed)
np.testing.assert_equal(data, {
    "test_altrep_wrap_real": [3],
})
```

#### `test_altrep_wrap_real_attributes` — `test_altrep_wrap_real_attributes.rds`

Test alternative representation of wrap_real with attributes.

- `rdata/legacy-1.0.0/test_altrep_wrap_real_attributes.rds` — rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8; 119 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
a = .Internal(wrap_meta(c(1, 2, 3), 0, 0)); attr(a, "foo") = "bar"; saveRDS(a, file="test_altrep_wrap_real_attributes.rds")
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
parsed = rdata.parser.parse_file(
    TESTDATA_PATH / "test_altrep_wrap_real_attributes.rds",
)
obj = parsed.object
assert obj.info.type == rdata.parser.RObjectType.REAL  # sanity check
assert not obj.info.object
assert obj.info.attributes
assert obj.attributes is not None
assert obj.attributes.tag is not None
assert obj.attributes.tag.value.value == b"foo"
assert obj.attributes.value[0].value[0].value == b"bar"

data = rdata.conversion.convert(parsed)
np.testing.assert_equal(data, [1., 2., 3.])
```

#### `test_altrep_wrap_real_class_attribute` — `test_altrep_wrap_real_class_attribute.rds`

Test altrep of wrap_real with class attribute.

- `rdata/legacy-1.0.0/test_altrep_wrap_real_class_attribute.rds` — rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8; 124 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
a = .Internal(wrap_meta(c(1, 2, 3), 0, 0)); attr(a, "class") = "Date"; saveRDS(a, file="test_altrep_wrap_real_class_attribute.rds")
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
parsed = rdata.parser.parse_file(
    TESTDATA_PATH / "test_altrep_wrap_real_class_attribute.rds",
)
obj = parsed.object
assert obj.info.type == rdata.parser.RObjectType.REAL  # sanity check
assert obj.info.object
assert obj.info.attributes
assert obj.attributes is not None
assert obj.attributes.tag is not None
assert obj.attributes.tag.value.value == b"class"
assert obj.attributes.value[0].value[0].value == b"Date"

data = rdata.conversion.convert(parsed)
np.testing.assert_equal(data, [1., 2., 3.])
```

#### `test_altrep_wrap_string` — `test_altrep_wrap_string.rda`

Test alternative representation of wrap_string.

- `rdata/legacy-1.0.0/test_altrep_wrap_string.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8; 130 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_altrep_wrap_string <- .Internal(wrap_meta("Hello", 0, 0))
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_altrep_wrap_string.rda")

np.testing.assert_equal(data, {
    "test_altrep_wrap_string": ["Hello"],
})
```

#### `test_ascii_ascii_characters` — `test_ascii_ascii_chars.rds`

Test reading string with all ascii printable characters.

- `rdata/legacy-1.0.0/test_ascii_ascii_chars.rds` — rds · ascii · v3 · none · R 4.4.0 · enc UTF-8; 159 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
saveRDS("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\v\f\r\n", file="test_ascii_ascii_chars.rds", ascii=TRUE, compress=FALSE)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(TESTDATA_PATH / "test_ascii_ascii_chars.rds")
assert data == "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\v\f\r\n", data  # noqa: E501
```

#### `test_ascii_characters` — `test_ascii_chars.rds`

Test reading string with all ascii printable characters.

- `rdata/legacy-1.0.0/test_ascii_chars.rds` — rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8; 154 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
saveRDS("0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\v\f\r\n", file="test_ascii_chars.rds")
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(TESTDATA_PATH / "test_ascii_chars.rds")
assert data == "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\v\f\r\n", data  # noqa: E501
```

#### `test_ascii_empty_string` — `test_ascii_empty_str.rds`

Test that the empty string is parsed correctly from ascii file.

- `rdata/legacy-1.0.0/test_ascii_empty_str.rds` — rds · ascii · v3 · none · R 4.3.2 · enc UTF-8; 41 B; rdata 1.1.0: parse+convert OK

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(TESTDATA_PATH / "test_ascii_empty_str.rds")
np.testing.assert_equal(data, [""])
```

#### `test_ascii_na_string` — `test_ascii_na_string.rds`

Test that the NA string is parsed correctly.

- `rdata/legacy-1.0.0/test_ascii_na_string.rds` — rds · ascii · v3 · none · R 4.4.0 · enc UTF-8; 36 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
saveRDS(as.character(NA), file="test_ascii_na_string.rds", ascii=TRUE, compress=FALSE)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(TESTDATA_PATH / "test_ascii_na_string.rds")
np.testing.assert_equal(data, [None])
```

#### `test_ascii_nan_inf` — `test_ascii_nan_inf.rds`

Test reading nan and inf in ascii.

- `rdata/legacy-1.0.0/test_ascii_nan_inf.rds` — rds · ascii · v3 · none · R 4.4.0 · enc UTF-8; 46 B; rdata 1.1.0: parse+convert OK

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(TESTDATA_PATH / "test_ascii_nan_inf.rds")
np.testing.assert_equal(data, [0., np.nan, np.inf, -np.inf])
```

#### `test_ascii` — `test_ascii_v2.rda`, `test_ascii_v2.rds`, `test_ascii_v3.rda`, `test_ascii_v3.rds`, `test_ascii_win_v2.rda`, `test_ascii_win_v2.rds`, `test_ascii_win_v3.rda`, `test_ascii_win_v3.rds`

Test ascii files.

- `rdata/legacy-1.0.0/test_ascii_v2.rda` — rda `RDA2` · ascii · v2 · none · R 4.3.2; 109 B; rdata 1.1.0: parse+convert OK
- `rdata/legacy-1.0.0/test_ascii_v2.rds` — rds · ascii · v2 · none · R 4.3.2; 79 B; rdata 1.1.0: parse+convert OK
- `rdata/legacy-1.0.0/test_ascii_v3.rda` — rda `RDA3` · ascii · v3 · none · R 4.3.2 · enc UTF-8; 117 B; rdata 1.1.0: parse+convert OK
- `rdata/legacy-1.0.0/test_ascii_v3.rds` — rds · ascii · v3 · none · R 4.3.2 · enc UTF-8; 87 B; rdata 1.1.0: parse+convert OK
- `rdata/legacy-1.0.0/test_ascii_win_v2.rda` — rda `RDA2` · ascii · v2 · none · R 4.3.2; 105 B; rdata 1.1.0: parse+convert OK
- `rdata/legacy-1.0.0/test_ascii_win_v2.rds` — rds · ascii · v2 · none · R 4.3.2 · CRLF; 99 B; rdata 1.1.0: parse+convert OK
- `rdata/legacy-1.0.0/test_ascii_win_v3.rda` — rda `RDA3` · ascii · v3 · none · R 4.3.2 · enc CP1252; 114 B; rdata 1.1.0: parse+convert OK
- `rdata/legacy-1.0.0/test_ascii_win_v3.rds` — rds · ascii · v3 · none · R 4.3.2 · enc CP1252 · CRLF; 110 B; rdata 1.1.0: parse+convert OK

No generating code is in the repo. From the expected value and the bytes, the object is
`data <- list(1.1, 2L, 3+4i, NA, "aä")`, saved as `save(data, ascii = TRUE, version = v)` (object name
`data`) or `saveRDS(data, ascii = TRUE, compress = FALSE, version = v)`. The `win_` files were written on
Windows. Their `.rds` files have **CRLF** line endings (the first line is `A\r\n`). Their v3 `.rda` declares native
encoding `CP1252` and stores "aä" as a LATIN1-flagged CHARSXP (levels 16393 = 0x4009) with bytes
`a\344`. The non-Windows files store it UTF-8-flagged (32777 = 0x8009) as `a\303\244`. The v2 `.rda` and
`.rds` from Windows also carry the latin1 form. The fourth element is a *logical* NA.

Expected value (rdata 1.0.0 assertion, verbatim):

```python
ref_ma = np.ma.array(  # type: ignore[no-untyped-call]
    data=[True],
    mask=[True],
    fill_value=True,
)
ref = [[1.1], [2], [3. + 4.j], ref_ma, ["aä"]]

for tag, v, ext in itertools.product(
        ("", "win_"),
        (2, 3),
        ("rda", "rds"),
):
    f = f"test_ascii_{tag}v{v}.{ext}"
    with self.subTest(file=f):
        parsed = rdata.parser.parse_file(
            TESTDATA_PATH / f,
        )
        converted = rdata.conversion.convert(parsed)

        if ext == "rda":
            np.testing.assert_equal(converted, {"data": ref})
            ma = converted["data"][3]
        else:
            np.testing.assert_equal(converted, ref)
            ma = converted[3]

        # Test masked array separately
        np.testing.assert_equal(ma.data, ref_ma.data)
        np.testing.assert_equal(ma.mask, ref_ma.mask)
        np.testing.assert_equal(ma.mask, ref_ma.mask)
        np.testing.assert_equal(ma.get_fill_value(),
                                ref_ma.get_fill_value())
```

#### `test_builtin` — `test_builtin.rda`

Test that builtin functions can be parsed.

- `rdata/legacy-1.0.0/test_builtin.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8; 79 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_builtin <- abs
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_builtin.rda")

np.testing.assert_equal(data, {
    "test_builtin": rdata.conversion.RBuiltin(name="abs"),
})
```

#### `test_complex` — `test_complex.rda`

Test that complex numbers can be parsed.

- `rdata/legacy-1.0.0/test_complex.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.0.2; 89 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_complex <- c(1 + 2i, 2, 0, 1 + 3i, -1i)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_complex.rda")

np.testing.assert_equal(data, {
    "test_complex": np.array([1 + 2j, 2, 0, 1 + 3j, -1j]),
})
```

#### `test_dataframe` — `test_dataframe.rda`, `test_dataframe_v3.rda`

Test dataframe conversion.

- `rdata/legacy-1.0.0/test_dataframe.rda` — rda `RDX2` · xdr · v2 · gzip · R 4.1.3; 176 B; rdata 1.1.0: parse+convert OK
- `rdata/legacy-1.0.0/test_dataframe_v3.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.1.3 · enc UTF-8; 187 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
test_dataframe = data.frame(class=factor(c("a", "b", "b")), value=c(1L, 2L, 3L)); save(test_dataframe, file="test_dataframe.rda", version=2)
test_dataframe = data.frame(class=factor(c("a", "b", "b")), value=c(1L, 2L, 3L)); save(test_dataframe, file="test_dataframe_v3.rda")
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
for f in ("test_dataframe.rda", "test_dataframe_v3.rda"):
    with self.subTest(file=f):
        data = rdata.read_rda(TESTDATA_PATH / f)

        pd.testing.assert_frame_equal(
            data["test_dataframe"],
            pd.DataFrame(
                {
                    "class": pd.Categorical(
                        ["a", "b", "b"],
                    ),
                    "value": pd.Series(
                        [1, 2, 3],
                        dtype=pd.Int32Dtype(),
                    ).array,
                },
                index=pd.RangeIndex(start=1, stop=4),
            ),
        )
```

#### `test_dataframe_rds` — `test_dataframe.rds`, `test_dataframe_v3.rds`

Test dataframe conversion.

- `rdata/legacy-1.0.0/test_dataframe.rds` — rds · xdr · v2 · gzip · R 4.1.3; 153 B; rdata 1.1.0: parse+convert OK
- `rdata/legacy-1.0.0/test_dataframe_v3.rds` — rds · xdr · v3 · gzip · R 4.1.3 · enc UTF-8; 164 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
df = data.frame(class=factor(c("a", "b", "b")), value=c(1L, 2L, 3L)); saveRDS(df, file="test_dataframe.rds", version=2)
df = data.frame(class=factor(c("a", "b", "b")), value=c(1L, 2L, 3L)); saveRDS(df, file="test_dataframe_v3.rds")
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
for f in ("test_dataframe.rds", "test_dataframe_v3.rds"):
    with self.subTest(file=f):
        data = rdata.read_rds(TESTDATA_PATH / f)

        pd.testing.assert_frame_equal(
            data,
            pd.DataFrame(
                {
                    "class": pd.Categorical(
                        ["a", "b", "b"],
                    ),
                    "value": pd.Series(
                        [1, 2, 3],
                        dtype=pd.Int32Dtype(),
                    ).array,
                },
                index=pd.RangeIndex(start=1, stop=4),
            ),
        )
```

#### `test_dataframe_dtypes` — `test_dataframe_dtypes.rds`

Test dataframe conversion.

- `rdata/legacy-1.0.0/test_dataframe_dtypes.rds` — rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8; 217 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
df = data.frame(int=c(10L, 20L, 30L), float=c(1.1, 2.2, 3.3), string=c("x", "y", "z"), bool=as.logical(c(1, 0, 1)), complex=c(4+5i, 6+7i, 8+9i)); print(df); saveRDS(df, file="test_dataframe_dtypes.rds")
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(TESTDATA_PATH / "test_dataframe_dtypes.rds")

index = pd.RangeIndex(1, 4)
ref = pd.DataFrame(
    {
        "int": pd.Series(
            [10, 20, 30],
            dtype=pd.Int32Dtype(), index=index),
        "float": pd.Series(
            [1.1, 2.2, 3.3],
            dtype=float, index=index),
        "string": pd.Series(
            ["x", "y", "z"],
            dtype=pd.StringDtype(), index=index),
        "bool": pd.Series(
            [True, False, True],
            dtype=pd.BooleanDtype(), index=index),
        "complex": pd.Series(
            [4+5j, 6+7j, 8+9j],
            dtype=complex, index=index),
    },
    index=index,
)
pd.testing.assert_frame_equal(data, ref)
```

#### `test_dataframe_dtypes_with_na` — `test_dataframe_dtypes_with_na.rds`

Test dataframe conversion.

- `rdata/legacy-1.0.0/test_dataframe_dtypes_with_na.rds` — rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8; 235 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
df = data.frame(int=c(10L, 20L, 30L, NA), float=c(1.1, 2.2, 3.3, NA), string=c("x", "y", "z", NA), bool=as.logical(c(1, 0, 1, NA)), complex=c(4+5i, 6+7i, 8+9i, NA)); saveRDS(df, file="test_dataframe_dtypes_with_na.rds")
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(
    TESTDATA_PATH / "test_dataframe_dtypes_with_na.rds",
)

index = pd.RangeIndex(1, 5)
ref = pd.DataFrame(
    {
        "int": pd.Series(
            [10, 20, 30, pd.NA],
            dtype=pd.Int32Dtype(), index=index),
        "float": pd.Series(
            [1.1, 2.2, 3.3, R_FLOAT_NA],
            dtype=float, index=index),
        "string": pd.Series(
            ["x", "y", "z", pd.NA],
            dtype=pd.StringDtype(), index=index),
        "bool": pd.Series(
            [True, False, True, pd.NA],
            dtype=pd.BooleanDtype(), index=index),
        "complex": pd.Series(
            [4+5j, 6+7j, 8+9j, R_FLOAT_NA],
            dtype=complex, index=index),
    },
    index=index,
)

with np.errstate(invalid="ignore"):
    # Comparing complex arrays with R_FLOAT_NA gives warning
    pd.testing.assert_frame_equal(data, ref)
```

#### `test_dataframe_float_with_na_nan` — `test_dataframe_float_with_na_nan.rds`

Test dataframe conversion.

- `rdata/legacy-1.0.0/test_dataframe_float_with_na_nan.rds` — rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8; 147 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
df = data.frame(float=c(1.1, 2.2, 3.3, NA, NaN, Inf, -Inf)); saveRDS(df, file="test_dataframe_float_with_na_nan.rds")
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(
    TESTDATA_PATH / "test_dataframe_float_with_na_nan.rds",
)

index = pd.RangeIndex(1, 8)
ref = pd.DataFrame(
    {
        "float": pd.Series(
            [1.1, 2.2, 3.3, R_FLOAT_NA, np.nan, np.inf, -np.inf],
            dtype=float, index=index),
    },
    index=index,
)
pd.testing.assert_frame_equal(data, ref)
```

#### `test_dataframe_int_rownames` — `test_dataframe_int_rownames.rds`

Test dataframe conversion.

- `rdata/legacy-1.0.0/test_dataframe_int_rownames.rds` — rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8; 123 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
df = data.frame(col1=c(10, 20, 30), row.names=c(3L, 6L, 9L)); saveRDS(df, file="test_dataframe_int_rownames.rds")
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(
    TESTDATA_PATH / "test_dataframe_int_rownames.rds",
)

index = np.array([3, 6, 9], dtype=np.int32)
ref = pd.DataFrame(
    {
        "col1": pd.Series(
            [10., 20., 30.],
            dtype=float, index=index),
    },
    index=index,
)
pd.testing.assert_frame_equal(data, ref)
```

#### `test_dataframe_range_rownames` — `test_dataframe_range_rownames.rds`

Test dataframe conversion.

- `rdata/legacy-1.0.0/test_dataframe_range_rownames.rds` — rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8; 163 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
df = data.frame(col1=c(10, 20, 30), row.names=2:4); saveRDS(df, file="test_dataframe_range_rownames.rds")
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(
    TESTDATA_PATH / "test_dataframe_range_rownames.rds",
)

index = pd.RangeIndex(2, 5)
ref = pd.DataFrame(
    {
        "col1": pd.Series(
            [10., 20., 30.],
            dtype=float, index=index),
    },
    index=index,
)
pd.testing.assert_frame_equal(data, ref)
```

#### `test_dataframe_rownames` — `test_dataframe_rownames.rda`

Test dataframe conversion.

- `rdata/legacy-1.0.0/test_dataframe_rownames.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.1.1 · enc UTF-8; 216 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_dataframe_rownames <- data.frame(
    class=factor(c("a", "b", "b")),
    value=c(1L, 2L, 3L),
    row.names=c("Madrid", "Frankfurt", "Herzberg am Harz")
)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_dataframe_rownames.rda")

pd.testing.assert_frame_equal(
    data["test_dataframe_rownames"],
    pd.DataFrame(
        {
            "class": pd.Categorical(
                ["a", "b", "b"],
            ),
            "value": pd.Series(
                [1, 2, 3],
                dtype=pd.Int32Dtype(),
            ).array,
        },
        index=("Madrid", "Frankfurt", "Herzberg am Harz"),
    ),
)
```

#### `test_empty_dataframe` — `test_empty_dataframe.rda`

Test conversion of an empty dataframe.

- `rdata/legacy-1.0.0/test_empty_dataframe.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8; 125 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
test_empty_dataframe = data.frame()
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_empty_dataframe.rda")

pd.testing.assert_frame_equal(
    data["test_empty_dataframe"],
    pd.DataFrame(columns=[], index=np.array([], dtype=np.int32)),
)
```

#### `test_empty_dataframe_without_names` — `test_empty_dataframe_without_names.rda`

Test conversion of an empty dataframe without names.

- `rdata/legacy-1.0.0/test_empty_dataframe_without_names.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8; 132 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
test_empty_dataframe_without_names = data.frame()
attr(test_empty_dataframe_without_names, "names") <- NULL
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(
    TESTDATA_PATH / "test_empty_dataframe_without_names.rda",
)

pd.testing.assert_frame_equal(
    data["test_empty_dataframe_without_names"],
    pd.DataFrame(index=np.array([], dtype=np.int32)),
)
```

#### `test_empty_function` — `test_empty_function.rda`

Test that a simple function (compiled) can be parsed.

- `rdata/legacy-1.0.0/test_empty_function.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8; 323 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
options(keep.source = TRUE)
test_empty_function <- function() {}
library(compiler)
test_empty_function <- cmpfun(test_empty_function)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_empty_function.rda")

converted_fun = data["test_empty_function"]

assert isinstance(
    converted_fun,
    rdata.conversion.RFunction,
)

np.testing.assert_equal(converted_fun.environment, ChainMap({}))
np.testing.assert_equal(converted_fun.formals, None)

converted_body = converted_fun.body

assert isinstance(
    converted_body,
    rdata.conversion.RBytecode,
)

np.testing.assert_equal(converted_body.code, np.array([12, 17, 1]))
np.testing.assert_equal(converted_body.attributes, {})

np.testing.assert_equal(
    converted_fun.source,
    "test_empty_function <- function() {}\n",
)
```

#### `test_empty_function_uncompiled` — `test_empty_function_uncompiled.rda`

Test that a simple function can be parsed.

- `rdata/legacy-1.0.0/test_empty_function_uncompiled.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8; 271 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
options(keep.source = TRUE)
test_empty_function_uncompiled <- function() {}
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(
    TESTDATA_PATH / "test_empty_function_uncompiled.rda",
)

converted_fun = data["test_empty_function_uncompiled"]

assert isinstance(
    converted_fun,
    rdata.conversion.RFunction,
)

np.testing.assert_equal(converted_fun.environment, ChainMap({}))
np.testing.assert_equal(converted_fun.formals, None)
assert isinstance(converted_fun.body, rdata.conversion.RLanguage)
np.testing.assert_equal(
    converted_fun.source,
    "test_empty_function_uncompiled <- function() {}\n",
)
```

#### `test_empty_list` — `test_empty_list.rds`

Test parsing the empty list.

- `rdata/legacy-1.0.0/test_empty_list.rds` — rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8; 45 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_empty_list <- list()
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(TESTDATA_PATH / "test_empty_list.rds")
assert data == []
```

#### `test_empty_named_list` — `test_empty_named_list.rds`

Test parsing the empty list.

- `rdata/legacy-1.0.0/test_empty_named_list.rds` — rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8; 69 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_empty_named_list <- setNames(list(), character(0))
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(TESTDATA_PATH / "test_empty_named_list.rds")
assert data == {}
```

#### `test_empty_string` — `test_empty_str.rda`

Test that the empty string is parsed correctly.

- `rdata/legacy-1.0.0/test_empty_str.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.6.3; 72 B; rdata 1.1.0: parse+convert OK

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_empty_str.rda")

np.testing.assert_equal(data, {
    "test_empty_str": [""],
})
```

#### `test_emptyenv` — `test_emptyenv.rda`

Test parsing the empty environment.

- `rdata/legacy-1.0.0/test_emptyenv.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.4.4; 69 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_emptyenv <- emptyenv()
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_emptyenv.rda")

assert data == {
    "test_emptyenv": ChainMap({}),
}
```

#### `test_encodings` — `test_encodings.rda`

Test of differents encodings.

- `rdata/legacy-1.0.0/test_encodings.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.6.3; 175 B; rdata 1.1.0: parse+convert OK

Four objects. The equivalent R code from rdata HEAD tests is below; the legacy files predate it.

```r
test_encoding_utf8 <- "e\xc4\xa5o\xc5\x9dan\xc4\x9do \xc4\x89iu\xc4\xb5a\xc5\xadde"; Encoding(test_encoding_utf8) <- "UTF-8"
test_encoding_latin1 <- "ca\xf1\xf3n"; Encoding(test_encoding_latin1) <- "latin1"
test_encoding_bytes <- "reba\xf1o"; Encoding(test_encoding_bytes) <- "bytes"
test_encoding_latin1_implicit <- "\xcd\xf1igo"  # native-encoded string with no encoding flag
```
`test_encodings_v3.rda` was written on Windows (native encoding `CP1252`). The unflagged string must be
decoded with the header's native encoding ("Íñigo"). In the v2 file there is no header encoding, so rdata
returns raw bytes `b"\xcd\xf1igo"` and warns.

Expected value (rdata 1.0.0 assertion, verbatim):

```python
with self.assertWarns(
    UserWarning,
    msg="Unknown encoding. Assumed ASCII.",
):
    data = rdata.read_rda(
        TESTDATA_PATH / "test_encodings.rda",
    )

    np.testing.assert_equal(data, {
        "test_encoding_utf8": ["eĥoŝanĝo ĉiuĵaŭde"],
        "test_encoding_latin1": ["cañón"],
        "test_encoding_bytes": [b"reba\xf1o"],
        "test_encoding_latin1_implicit": [b"\xcd\xf1igo"],
    })
```

#### `test_encodings_v3` — `test_encodings_v3.rda`

Test encodings in version 3 format.

- `rdata/legacy-1.0.0/test_encodings_v3.rda` — rda `RDX3` · xdr · v3 · gzip · R 3.6.3 · enc CP1252; 184 B; rdata 1.1.0: parse+convert OK

Four objects. The equivalent R code from rdata HEAD tests is below; the legacy files predate it.

```r
test_encoding_utf8 <- "e\xc4\xa5o\xc5\x9dan\xc4\x9do \xc4\x89iu\xc4\xb5a\xc5\xadde"; Encoding(test_encoding_utf8) <- "UTF-8"
test_encoding_latin1 <- "ca\xf1\xf3n"; Encoding(test_encoding_latin1) <- "latin1"
test_encoding_bytes <- "reba\xf1o"; Encoding(test_encoding_bytes) <- "bytes"
test_encoding_latin1_implicit <- "\xcd\xf1igo"  # native-encoded string with no encoding flag
```
`test_encodings_v3.rda` was written on Windows (native encoding `CP1252`). The unflagged string must be
decoded with the header's native encoding ("Íñigo"). In the v2 file there is no header encoding, so rdata
returns raw bytes `b"\xcd\xf1igo"` and warns.

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_encodings_v3.rda")

np.testing.assert_equal(data, {
    "test_encoding_utf8": ["eĥoŝanĝo ĉiuĵaŭde"],
    "test_encoding_latin1": ["cañón"],
    "test_encoding_bytes": [b"reba\xf1o"],
    "test_encoding_latin1_implicit": ["Íñigo"],
})
```

#### `test_environment` — `test_environment.rda`

Test parsing of environments.

- `rdata/legacy-1.0.0/test_environment.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.4.4; 103 B; rdata 1.1.0: parse+convert OK

Expected value (rdata 1.0.0 assertion, verbatim):

```python
parsed = rdata.parser.parse_file(
    TESTDATA_PATH / "test_environment.rda",
)
converted = rdata.conversion.convert(parsed)

dict_env = {"string": ["test"]}
empty_global_env: dict[str, Any] = {}

np.testing.assert_equal(converted, {
    "test_environment": ChainMap(dict_env, ChainMap(empty_global_env)),
})

global_env = {"global": "test"}

converted_global = rdata.conversion.convert(
    parsed,
    global_environment=global_env,
)

np.testing.assert_equal(converted_global, {
    "test_environment": ChainMap(dict_env, ChainMap(global_env)),
})
```

#### `test_expression` — `test_expression.rda`

Test that expressions can be parsed.

- `rdata/legacy-1.0.0/test_expression.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.0.2; 100 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_expression <- expression(base^exponent)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_expression.rda")

np.testing.assert_equal(data, {
    "test_expression": rdata.conversion.RExpression([
        rdata.conversion.RLanguage(
            ["^", "base", "exponent"],
            attributes={},
        ),
    ]),
})
```

#### `test_factor` — `test_factor.rds`

Test factor conversion.

- `rdata/legacy-1.0.0/test_factor.rds` — rds · xdr · v3 · gzip · R 4.1.3 · enc UTF-8; 100 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
data = factor(c("a", "b", "b")); saveRDS(data, file="test_factor.rds")
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(
    TESTDATA_PATH / "test_factor.rds",
)

pd.testing.assert_frame_equal(
    pd.DataFrame(data),
    pd.DataFrame(pd.Categorical(["a", "b", "b"])),
)
```

#### `test_file` — `test_file.rda`

Test that external pointers can be parsed.

- `rdata/legacy-1.0.0/test_file.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8; 124 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_file <- file()
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_file.rda")

np.testing.assert_equal(data, {
    "test_file": [5],
})
```

#### `test_full_named_matrix` — `test_full_named_matrix.rda`

Test that a named matrix with dim names can be parsed.

- `rdata/legacy-1.0.0/test_full_named_matrix.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8; 184 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
dimnames <- list(
    my_dim_0=c("dim0_0", "dim0_1"),
    my_dim_1=c("dim1_0", "dim1_1", "dim1_2")
)
test_full_named_matrix <- matrix(
    1:6,
    nrow=2,
    byrow=TRUE,
    dimnames=dimnames
)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_full_named_matrix.rda")

reference = xarray.DataArray(
    [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ],
    dims=["my_dim_0", "my_dim_1"],
    coords={
        "my_dim_0": ["dim0_0", "dim0_1"],
        "my_dim_1": ["dim1_0", "dim1_1", "dim1_2"],
    },
)

xarray.testing.assert_identical(
    data["test_full_named_matrix"],
    reference,
)
```

#### `test_full_named_matrix_rds` — `test_full_named_matrix.rds`

Test that a named matrix with dim names can be parsed.

- `rdata/legacy-1.0.0/test_full_named_matrix.rds` — rds · xdr · v3 · gzip · R 4.1.3 · enc UTF-8; 154 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
dimnames <- list(
    my_dim_0=c("dim0_0", "dim0_1"),
    my_dim_1=c("dim1_0", "dim1_1", "dim1_2")
)
test_full_named_matrix <- matrix(
    1:6,
    nrow=2,
    byrow=TRUE,
    dimnames=dimnames
)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(TESTDATA_PATH / "test_full_named_matrix.rds")

reference = xarray.DataArray(
    [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ],
    dims=["my_dim_0", "my_dim_1"],
    coords={
        "my_dim_0": ["dim0_0", "dim0_1"],
        "my_dim_1": ["dim1_0", "dim1_1", "dim1_2"],
    },
)

xarray.testing.assert_identical(
    data,
    reference,
)
```

#### `test_function` — `test_function.rda`

Test that functions can be parsed.

- `rdata/legacy-1.0.0/test_function.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8; 382 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
options(keep.source = TRUE)
test_function <- function() {print("Hello")}
library(compiler)
test_function <- cmpfun(test_function)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_function.rda")

converted_fun = data["test_function"]

assert isinstance(
    converted_fun,
    rdata.conversion.RFunction,
)

np.testing.assert_equal(converted_fun.environment, ChainMap({}))
np.testing.assert_equal(converted_fun.formals, None)

converted_body = converted_fun.body

assert isinstance(
    converted_body,
    rdata.conversion.RBytecode,
)

np.testing.assert_equal(
    converted_body.code,
    np.array([12, 23, 1, 34, 4, 38, 2, 1]),
)
np.testing.assert_equal(converted_body.attributes, {})

np.testing.assert_equal(
    converted_fun.source,
    "test_function <- function() {print(\"Hello\")}\n",
)
```

#### `test_function_arg` — `test_function_arg.rda`

Test that functions can be parsed.

- `rdata/legacy-1.0.0/test_function_arg.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8; 409 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
options(keep.source = TRUE)
test_function_arg <- function(a) {print(a)}
library(compiler)
test_function_arg <- cmpfun(test_function_arg)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_function_arg.rda")

converted_fun = data["test_function_arg"]

assert isinstance(
    converted_fun,
    rdata.conversion.RFunction,
)

np.testing.assert_equal(converted_fun.environment, ChainMap({}))
np.testing.assert_equal(converted_fun.formals, {"a": NotImplemented})

converted_body = converted_fun.body

assert isinstance(
    converted_body,
    rdata.conversion.RBytecode,
)

np.testing.assert_equal(
    converted_body.code,
    np.array([12, 23, 1, 29, 4, 38, 2, 1]),
)
np.testing.assert_equal(converted_body.attributes, {})

np.testing.assert_equal(
    converted_fun.source,
    "test_function_arg <- function(a) {print(a)}\n",
)
```

#### `test_half_named_matrix` — `test_half_named_matrix.rda`

Test that a named matrix with no name for a dim can be parsed.

- `rdata/legacy-1.0.0/test_half_named_matrix.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8; 152 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_half_named_matrix <- matrix(
    1:6,
    nrow=2,
    byrow=TRUE,
    dimnames=list(c("dim0_0", "dim0_1"))
)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_half_named_matrix.rda")

reference = xarray.DataArray(
    [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ],
    dims=["dim_0", "dim_1"],
    coords={
        "dim_0": ["dim0_0", "dim0_1"],
    },
)

xarray.testing.assert_identical(
    data["test_half_named_matrix"],
    reference,
)
```

#### `test_list` — `test_list.rda`

Test that list can be parsed.

- `rdata/legacy-1.0.0/test_list.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.0.2; 103 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_list <- list(1, c("a", "b", "c"), c(2, 3), "hi")
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_list.rda")

np.testing.assert_equal(data, {
    "test_list":
        [
            np.array([1.0]),
            ["a", "b", "c"],
            np.array([2.0, 3.0]),
            ["hi"],
        ],
})
```

#### `test_list_attrs` — `test_list_attrs.rda`

Test that lists accept attributes.

- `rdata/legacy-1.0.0/test_list_attrs.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.4.4; 111 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_list_attrs <- list("list", 5)
attr(test_list_attrs, "my_attr") <- "attr_value"
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_list_attrs.rda")

np.testing.assert_equal(data, {
    "test_list_attrs": [["list"], [5]],
})
```

#### `test_logical` — `test_logical.rda`

Test parsing of logical vectors.

- `rdata/legacy-1.0.0/test_logical.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.0.2; 73 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_logical <- c(TRUE, TRUE, FALSE, TRUE, FALSE)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_logical.rda")

np.testing.assert_equal(data, {
    "test_logical": np.array([True, True, False, True, False]),
})
```

#### `test_matrix` — `test_matrix.rda`

Test that a matrix can be parsed.

- `rdata/legacy-1.0.0/test_matrix.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.0.2; 102 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_matrix <- matrix(1:6, nrow=2, byrow=TRUE)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_matrix.rda")

np.testing.assert_equal(data, {
    "test_matrix": np.array([
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ]),
})
```

#### `test_minimal_function` — `test_minimal_function.rda`

Test that a minimal function (compiled) can be parsed.

- `rdata/legacy-1.0.0/test_minimal_function.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8; 275 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
options(keep.source = TRUE)
test_minimal_function <- function() NULL
library(compiler)
test_minimal_function <- cmpfun(test_minimal_function)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_minimal_function.rda")

converted_fun = data["test_minimal_function"]

assert isinstance(
    converted_fun,
    rdata.conversion.RFunction,
)

np.testing.assert_equal(converted_fun.environment, ChainMap({}))
np.testing.assert_equal(converted_fun.formals, None)

converted_body = converted_fun.body

assert isinstance(
    converted_body,
    rdata.conversion.RBytecode,
)

np.testing.assert_equal(converted_body.code, np.array([12, 17, 1]))
np.testing.assert_equal(converted_body.attributes, {})

np.testing.assert_equal(
    converted_fun.source,
    "test_minimal_function <- function() NULL\n",
)
```

#### `test_minimal_function_uncompiled` — `test_minimal_function_uncompiled.rda`

Test that a minimal function can be parsed.

- `rdata/legacy-1.0.0/test_minimal_function_uncompiled.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8; 217 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
options(keep.source = TRUE)
test_minimal_function_uncompiled <- function() NULL
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(
    TESTDATA_PATH / "test_minimal_function_uncompiled.rda",
)

converted_fun = data["test_minimal_function_uncompiled"]

assert isinstance(
    converted_fun,
    rdata.conversion.RFunction,
)

np.testing.assert_equal(converted_fun.environment, ChainMap({}))
np.testing.assert_equal(converted_fun.formals, None)
np.testing.assert_equal(converted_fun.body, None)
np.testing.assert_equal(
    converted_fun.source,
    "test_minimal_function_uncompiled <- function() NULL\n",
)
```

#### `test_na_string` — `test_na_string.rda`

Test that the NA string is parsed correctly.

- `rdata/legacy-1.0.0/test_na_string.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.6.3; 78 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_na_string <- as.character(NA)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_na_string.rda")

np.testing.assert_equal(data, {
    "test_na_string": [None],
})
```

#### `test_named_matrix` — `test_named_matrix.rda`

Test that a named matrix can be parsed.

- `rdata/legacy-1.0.0/test_named_matrix.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8; 161 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
dimnames <- list(
    c("dim0_0", "dim0_1"),
    c("dim1_0", "dim1_1", "dim1_2")
)
test_named_matrix <- matrix(1:6, nrow=2, byrow=TRUE, dimnames=dimnames)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_named_matrix.rda")

reference = xarray.DataArray(
    [
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0],
    ],
    dims=["dim_0", "dim_1"],
    coords={
        "dim_0": ["dim0_0", "dim0_1"],
        "dim_1": ["dim1_0", "dim1_1", "dim1_2"],
    },
)

xarray.testing.assert_identical(
    data["test_named_matrix"],
    reference,
)
```

#### `test_named_vector` — `test_named_vector.rda`

Test parsing of vectors with names.

- `rdata/legacy-1.0.0/test_named_vector.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8; 119 B; rdata 1.1.0: parse+convert OK

R code recorded in the 1.0.0 test:

```r
test_named_vector = c(a=1, b=2, c=3)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_named_vector.rda")

xarray.testing.assert_identical(
    data["test_named_vector"],
    xarray.DataArray(
        [1.0, 2.0, 3.0],
        coords=[["a", "b", "c"]],
    ),
)
```

#### `test_nan_inf` — `test_nan_inf.rds`

Test reading nan and inf.

- `rdata/legacy-1.0.0/test_nan_inf.rds` — rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8; 57 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_nan_inf <- c(0, -0, NaN, Inf, -Inf)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rds(TESTDATA_PATH / "test_nan_inf.rds")
np.testing.assert_equal(data, [0., np.nan, np.inf, -np.inf])
```

#### `test_nullable_int` — `test_nullable_int.rda`

Test parsing of integer vectors containing NA.

- `rdata/legacy-1.0.0/test_nullable_int.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.3.1 · enc UTF-8; 94 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_nullable_int <- c(313L, -12L, NA)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_nullable_int.rda")

array = data["test_nullable_int"]
np.testing.assert_array_equal(
    array.data,
    np.array([313, -12, -2**31]),
)
np.testing.assert_array_equal(
    array.mask,
    np.array([False, False, True]),
)
```

#### `test_nullable_logical` — `test_nullable_logical.rda`

Test parsing of logical vectors containing NA.

- `rdata/legacy-1.0.0/test_nullable_logical.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.3.1 · enc UTF-8; 93 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_nullable_logical <- c(TRUE, FALSE, NA)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_nullable_logical.rda")

array = data["test_nullable_logical"]
np.testing.assert_array_equal(
    array.data,
    np.array([True, False, True]),
)
np.testing.assert_array_equal(
    array.mask,
    np.array([False, False, True]),
)
```

#### `test_s4` — `test_s4.rda`

Test parsing of S4 classes.

- `rdata/legacy-1.0.0/test_s4.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.4.4; 139 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
setClass("Person", representation(name = "character", age = "numeric"))
test_s4 <- new("Person", name = "Carlos", age = 28)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
with pytest.warns(UserWarning, match="Missing constructor"):
    data = rdata.read_rda(TESTDATA_PATH / "test_s4.rda")

np.testing.assert_equal(data, {
    "test_s4": SimpleNamespace(
        age=np.array(28),
        name=["Carlos"],
        **{"class": ["Person"]},
    ),
})
```

#### `test_ts` — `test_ts.rda`

Test time series conversion.

- `rdata/legacy-1.0.0/test_ts.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.0.2; 119 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_ts <- ts(c(1, 2, 3), start=c(2000, 3), frequency = 12)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_ts.rda")

pd.testing.assert_series_equal(
    data["test_ts"],
    pd.Series({
        2000 + Fraction(2, 12): 1.0,
        2000 + Fraction(3, 12): 2.0,
        2000 + Fraction(4, 12): 3.0,
    }),
)
```

#### `test_vector` — `test_vector.rda`

Test parsing of numerical vectors.

- `rdata/legacy-1.0.0/test_vector.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.0.2; 78 B; rdata 1.1.0: parse+convert OK

No R code recorded in 1.0.0; the same-named object in rdata HEAD is generated by (the legacy file predates that test, so the 1.0.0 expectation below is authoritative):

```r
test_vector <- c(1.0, 2.0, 3.0)
```

Expected value (rdata 1.0.0 assertion, verbatim):

```python
data = rdata.read_rda(TESTDATA_PATH / "test_vector.rda")

np.testing.assert_equal(data, {
    "test_vector": np.array([1.0, 2.0, 3.0]),
})
```

## 3. `rdatajl/` — JuliaData/RData.jl test files (67 files)

Source: <https://github.com/JuliaData/RData.jl/tree/3103bea8f89a443ce86ad6f5d4a97887773c2ede/test> (`data_v2/`, `data_v3/`). MIT.
Generated by `test/generate_rda.R` (quoted in full below) with R 3.5.2–4.1.2, using `save()`/`saveRDS()` with
`version = 2|3`. Compression follows the defaults: gzip, except `*_ascii.rda` (uncompressed),
`*_ascii.rds` (gzip-compressed ASCII), `altrep_ascii.rda` (gzip-compressed ASCII, `compress = TRUE`),
`types_decomp.rds` (uncompressed) and `compressed_*.rda`.

Caveats found while checking the bytes: `data_v2/dfattributes.rda` is actually serialization **v3** and is
byte-identical to `data_v3/dfattributes.rda`. `data_v3/dup_levels.rda` is **v2** (R 3.4.4). It is not
produced by `generate_rda.R`; it was added for a duplicated-factor-levels issue. The four
`data_v3/compressed_{false,gzip,bzip2,xz}.rda` decompress to byte-identical payloads (sha256 verified).

<details><summary><code>test/generate_rda.R</code> (verbatim)</summary>

```r
# R script to generate test .rda and .rds files
if ((as.integer(version$major) < 3) ||
    ((as.integer(version$major) == 3) && (as.integer(substr(version$minor, 1, 1)) < 5))) {
    stop("Script requires R>=3.5 to generate RData version 2 and 3")
}
sys_tz = Sys.getenv("TZ") # remember System TimeZone

for (ver in c(2, 3)) {
message("Generating RData/RDS files (version=", ver, ")...")

df <- data.frame(num = c(1.1, 2.2))
rdata_path <- paste0("data_v", ver)
save(df, file=file.path(rdata_path, "minimal.rda"), version=ver)
save(df, file=file.path(rdata_path, "minimal_ascii.rda"), version=ver, ascii=TRUE)

df["int"] <- c(1L, 2L)
df["logi"] <- c(TRUE, FALSE)
df["chr"] <- c("ab", "c")
df["factor"] <- factor(df$chr)
df["cplx"] <- complex( real = c(1.1, 0.0), imaginary = c(0.5, 1.0) )
#utf<-c("Ж", "∰")) R handles it, RData doesn"t.
save(df, file=file.path(rdata_path, "types.rda"), version=ver)
save(df, file=file.path(rdata_path, "types_ascii.rda"), version=ver, ascii=TRUE)
saveRDS(df, file=file.path(rdata_path, "types.rds"), version=ver)
saveRDS(df, file=file.path(rdata_path, "types_ascii.rds"), version=ver, ascii=TRUE)
saveRDS(df, file=file.path(rdata_path, "types_decomp.rds"), version=ver, compress=FALSE)

df[2, ] <- NA
df[3, ] <- df[2, ]
df[3, "num"] <- NaN
df[, "cplx"] <- complex( real = c(1.1, 1, NaN), imaginary = c(NA, NaN, 0) )
save(df, file=file.path(rdata_path, "NAs.rda"), version=ver)
save(df, file=file.path(rdata_path, "NAs_ascii.rda"), version=ver, ascii=TRUE)

names(df) <- c("end", "!", "1", "%_B*\tC*", NA, "x")
save(df, file=file.path(rdata_path, "names.rda"), version=ver)
save(df, file=file.path(rdata_path, "names_ascii.rda"), version=ver, ascii=TRUE)

empty.env <- new.env(parent = emptyenv())
empty_nohash.env <- new.env(parent = emptyenv())
empty_child.env <- new.env(parent = empty.env)
a.env <- new.env(parent = emptyenv())
assign( "a", 43, envir = a.env )
b.env <- new.env(parent = a.env)
assign( "b", 48, envir = b.env )
save(empty.env, empty_nohash.env, empty_child.env, a.env, b.env,
     file=file.path(rdata_path, "envs.rda"), version=ver)

pl.empty <- pairlist()
pl <- pairlist(u = 1L, x = c(2.0, 3.0), y = "A", NULL, z = TRUE )
save(pl.empty, pl, file=file.path(rdata_path, "pairlists.rda"), version=ver)

test.fun0 <- function() return(NULL)
test.fun1 <- function(x) return(x + 1)
test.fun2 <- function(x, y) x + y
save(test.fun0, test.fun1, test.fun2,
     file=file.path(rdata_path, "closures.rda"), version=ver)

require(compiler)
test.cmpfun0 <- cmpfun(test.fun0)
test.cmpfun1 <- cmpfun(test.fun1)
test.cmpfun2 <- cmpfun(test.fun2)
save(test.cmpfun0, test.cmpfun1, test.cmpfun2,
     file=file.path(rdata_path, "cmpfun.rda"), version=ver)

x <- factor(c("a", "b", "c"))
y <- ordered(x, levels=c("b", "a", "c"))
save(x, y, file=file.path(rdata_path, "ord.rda"), version=ver)

numdates <- as.Date(as.numeric(1:4), origin="2017-01-01")
intdates <- seq.Date(as.Date("2017-01-02"), by="day", length.out=4)
if (typeof(intdates) != "integer") stop("intdates are not integer-backed dates: ", typeof(intdates))
datetimes <- as.POSIXct("2017-01-01 13:23", tz="UTC") + 1:4

dateNAs = list(c(numdates, NA), c(datetimes, NA))
saveRDS(dateNAs, file=file.path(rdata_path, "datesNA.rds"), version=ver)

saveRDS(numdates, file=file.path(rdata_path, "numdates.rds"), version=ver)
saveRDS(intdates, file=file.path(rdata_path, "intdates.rds"), version=ver)
saveRDS(numdates, file=file.path(rdata_path, "numdates_ascii.rds"), version=ver, ascii=TRUE)
saveRDS(intdates, file=file.path(rdata_path, "intdates_ascii.rds"), version=ver, ascii=TRUE)

dtlst = list(datetimes, datetimes[1])
names(datetimes) = LETTERS[1:length(datetimes)]
dtlst = c(dtlst, list(datetimes), list(datetimes[1]))
saveRDS(dtlst, file=file.path(rdata_path, "datetimes.rds"), version=ver)

datedfs = list(data.frame(date=numdates[1], datetime=datetimes[1]),
               data.frame(date=numdates, datetime=datetimes))
saveRDS(datedfs, file=file.path(rdata_path, "datedfs.rds"), version=ver)

# the first element here is assumed to be in the local timezone but is saved in
# UTC time, without any timezone attribute. When R reads it, it assumes local time.
# So the test associated with this first datapoint is going to assume which timezone
# the data is generated in! (PST/-8)
Sys.setenv(TZ = "America/Los_Angeles")
saveRDS(list(as.POSIXct("2017-01-01 13:23"),
             as.POSIXct("2017-01-01 13:23", tz="CST"),
             as.POSIXct("2017-01-01 13:23", tz="America/Chicago")),
        file=file.path(rdata_path, "datetimes_tz.rds"), version=ver)
Sys.setenv(TZ = sys_tz) # restore timezone

# Importing data frame attributes as defined by common packages to metadata

# Column-level attributes used by packages haven, labelled and sjlabelled
# Generating code:
# library(haven)
# v1 <- labelled(c(1, 2, 2, 3, NA, 1), label="V1", labels=c(a=1, b=2, c=3))
# v2 <- labelled_spss(c(1, 2, 2, 3, NA, 1), label="V2", labels=c(a=1, b=2, c=3),
#                     na_values=3)
# v3 <- labelled_spss(c(1, 2, 2, 3, NA, 1), label="V3", labels=c(a=1, b=2, c=3),
#                     na_range=c(3, Inf))
v1 <- structure(c(1, 2, 2, 3, NA, 1), labels=c(a=1, b=2, c=3), label="V1",
                class="numeric")
v2 <- structure(c(1, 2, 2, 3, NA, 1), labels=c(a=1, b=2, c=3), label="V2",
                na_values=3, class="numeric")
v3 <- structure(c(1, 2, 2, 3, NA, 1), labels=c(a=1, b=2, c=3), label="V3",
                na_range=c(3, Inf), class="numeric")

# Column-level attributes used by packages Hmisc, units and labelVector
# (plus `comment` from base R and some custom attributes)
# Generating code:
# library(Hmisc)
# v4 <- c(1, 2, 2, 3, NA, 1)
# label(v4) <- "V4"
# comment(v4) <- "A comment"
# units(v4) <- "m/s^2"
# attr(v4, "custom") <- 1
v4 <- structure(c(1, 2, 2, 3, NA, 1), label="V4", class="numeric",
                comment="A comment", units="m/s^2", custom=1)

# Data frame-level attributes
df <- data.frame(v1, v2, v3, v4)
comment(df) <- "This is a data frame"
attr(df, "collectiontimes") <- c(as.POSIXct("2022-05-25 22:05:00", tz="UTC"),
                                 as.POSIXct("2022-05-26 22:05:00", tz="UTC"))

save(df, file=file.path(rdata_path, "dfattributes.rda"), version=ver)

savedotdotdot <- function(...) {
     dotdotdot <- get("...", environment())
     save(dotdotdot, file=file.path(rdata_path, "dotdotdot.rda"), version=ver)
}
savedotdotdot(1,2,3)

expr <- parse(text="a+b\nx+2")
save(expr, file=file.path(rdata_path, "expr.rda"), version=ver)

# generate files storing GLM results (as an example of a more complex R object)
# taken from "Generalized Linear Models in R", https://web.stanford.edu/class/stats306a/RforGLM.pdf
df = data.frame(ldose = rep(0:5, 2),
                numdead = c(1,4,9,13,18,20,0,2,6,10,12,16),
                sex = factor(rep(c("M", "F"), rep(6, 2))))
df$numalive = 20 - df$numdead
df$SF = cbind(df$numdead, df$numalive)
budworm <- glm(SF ~ sex * ldose, family=binomial, data = df)
save(budworm, file=file.path(rdata_path, "budworm_glm.rda"), version=ver)

} # for (ver in ...)

# generate V3 format AltRep objects
longseq <- 1:1000 # compact_intseq AltRep
wrapvec <- .Internal(wrap_meta(c(1, 2.5, 3), TRUE, TRUE)) # wrap_real AltRep

 # AltRep for factor (long enough vector is represented as AltRep)
factoraltrep <- rep(factor(c("A", "B", NA_character_, "C")), each=5000)

# wrap_real AltRep of a matrix contains non-standard pairlist that doesn't end with NILVALUE_SXP
nonnilpairlist <- .Internal(wrap_meta(matrix(integer(), nrow=0, ncol=10), TRUE, TRUE))

save(longseq, wrapvec, factoraltrep, nonnilpairlist,
     file=file.path("data_v3", "altrep.rda"), version=3)
save(longseq, wrapvec, factoraltrep, nonnilpairlist,
     file=file.path("data_v3", "altrep_ascii.rda"), version=3, ascii=TRUE, compress=TRUE)

# for wide data frame (100 columns or more) R 3.5 *sometimes* uses AltRep to store the column names
altrepnames_list <- as.list(c(1, 2, 3))
names(altrepnames_list) <- .Internal(wrap_meta(c("a", "b", "c"), TRUE, TRUE))
altrepnames_df <- as.data.frame(altrepnames_list)
names(altrepnames_df) <- names(altrepnames_list)
save(altrepnames_list, altrepnames_df, file=file.path("data_v3", "altrep_names.rda"), version=3)

# list of vectors variable and list of vectors column (for #82)
listofvec <- list(c(1, 2, NA), c(3, 4), c(5, 6, NA))
listofvec2 <- list(c(1L, 2L, NA), c(3, 4), c(5, 6, NA))
listofvec3 <- list(c(1L, 2L), c(3, 4))
listofvec4 <- list(c(1, 2), c(3, 4,5))
namedlistofvec <- list(A=c(1, 2, NA), c(3, 4), B=c(5, 6, NA))
testdf <- data.frame(a = c("a", "b", "c"))
testdf$listascol <- list(c(1, 2, NA), c(3, 4), c(5, 6, NA, 7))
testdf$listascol2 <- list(c(1, 2), c(3L, 4L), c(5, 6, 7))
for (rdaver in c(2L, 3L)) {
    save(listofvec, listofvec2, listofvec3, listofvec4, namedlistofvec, testdf,
         file=file.path(paste0("data_v", rdaver), "list_of_vec.rda"), version=rdaver)
}

# compact_xxxseq
intseq1 <- 5L:200L
intseq2 <- seq.int(-2L, 1000L, 2L)
realseq1 <- 110:300
realseq2 <- seq(110.5, 300.5)
realseq3 <- seq(110, 300, by=0.5)
save(intseq1, intseq2, realseq1, realseq2, realseq3, file=file.path("data_v3", "compact_seq.rda"), version=3)

# generate files using each of the supported compression types
df <- data.frame(num = c(1.1, 2.2))
rdata_path <- "data_v3"
save(df, file=file.path(rdata_path, "compressed_gzip.rda"), version=3, compress="gzip")
save(df, file=file.path(rdata_path, "compressed_bzip2.rda"), version=3, compress="bzip2")
save(df, file=file.path(rdata_path, "compressed_xz.rda"), version=3, compress="xz")
save(df, file=file.path(rdata_path, "compressed_false.rda"), version=3, compress=FALSE)
```
</details>

#### `minimal`

`df <- data.frame(num = c(1.1, 2.2))`, saved as `df`.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): `DataFrame(num = [1.1, 2.2])`.

- `rdatajl/data_v2/minimal.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.5.2; 130 B; rdata 1.1.0: parse+convert OK; reads as `{'df': num 1 1.1 2 2.2}`
- `rdatajl/data_v3/minimal.rda` — rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 138 B; rdata 1.1.0: parse+convert OK; reads as `{'df': num 1 1.1 2 2.2}`

#### `minimal_ascii`

Same `df` as `minimal`, `save(..., ascii = TRUE)` (uncompressed ASCII).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): same as minimal.

- `rdatajl/data_v2/minimal_ascii.rda` — rda `RDA2` · ascii · v2 · none · R 3.5.2; 194 B; rdata 1.1.0: parse+convert OK; reads as `{'df': num 1 1.1 2 2.2}`
- `rdatajl/data_v3/minimal_ascii.rda` — rda `RDA3` · ascii · v3 · none · R 3.5.2 · enc UTF-8; 202 B; rdata 1.1.0: parse+convert OK; reads as `{'df': num 1 1.1 2 2.2}`

#### `types`

`df` with columns `num = c(1.1, 2.2)`, `int = c(1L, 2L)`, `logi = c(TRUE, FALSE)`, `chr = c("ab", "c")`, `factor = factor(chr)`, `cplx = complex(real = c(1.1, 0.0), imaginary = c(0.5, 1.0))`. Saved as `df` in `.rda`; the `.rds` is the bare data.frame.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): `DataFrame(num = [1.1, 2.2], int = Int32[1, 2], logi = [true, false], chr = ["ab", "c"], factor = categorical(["ab", "c"]), cplx = [1.1+0.5im, 1.0im])`.

- `rdatajl/data_v2/types.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.5.2; 225 B; rdata 1.1.0: parse+convert OK; reads as `{'df': num int logi chr factor cplx 1 1.1 1 True ab ab 1.1+0.5j 2 2.2 2 False c c 0.0+1.0j}`
- `rdatajl/data_v2/types.rds` — rds · xdr · v2 · gzip · R 3.5.2; 210 B; rdata 1.1.0: parse+convert OK; reads as `DataFrame(2, 6) columns=[np.str_('num'), np.str_('int'), np.str_('logi'), np.str_('chr'), np.str_('factor'), np.str_('cplx')] dtypes=['float64', 'Int32', 'boolean', 'string', 'category', 'complex128'] …`
- `rdatajl/data_v3/types.rda` — rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 233 B; rdata 1.1.0: parse+convert OK; reads as `{'df': num int logi chr factor cplx 1 1.1 1 True ab ab 1.1+0.5j 2 2.2 2 False c c 0.0+1.0j}`
- `rdatajl/data_v3/types.rds` — rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 217 B; rdata 1.1.0: parse+convert OK; reads as `DataFrame(2, 6) columns=[np.str_('num'), np.str_('int'), np.str_('logi'), np.str_('chr'), np.str_('factor'), np.str_('cplx')] dtypes=['float64', 'Int32', 'boolean', 'string', 'category', 'complex128'] …`

#### `types_ascii`

Same `df` as `types`. `.rda` is uncompressed ASCII; `.rds` is **gzip-compressed ASCII** (`saveRDS(ascii = TRUE)`).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): same as types.

- `rdatajl/data_v2/types_ascii.rda` — rda `RDA2` · ascii · v2 · none · R 3.5.2; 423 B; rdata 1.1.0: parse+convert OK; reads as `{'df': num int logi chr factor cplx 1 1.1 1 True ab ab 1.1+0.5j 2 2.2 2 False c c 0.0+1.0j}`
- `rdatajl/data_v2/types_ascii.rds` — rds · ascii · v2 · gzip · R 3.5.2; 198 B; rdata 1.1.0: parse+convert OK; reads as `DataFrame(2, 6) columns=[np.str_('num'), np.str_('int'), np.str_('logi'), np.str_('chr'), np.str_('factor'), np.str_('cplx')] dtypes=['float64', 'Int32', 'boolean', 'string', 'category', 'complex128'] …`
- `rdatajl/data_v3/types_ascii.rda` — rda `RDA3` · ascii · v3 · none · R 3.5.2 · enc UTF-8; 431 B; rdata 1.1.0: parse+convert OK; reads as `{'df': num int logi chr factor cplx 1 1.1 1 True ab ab 1.1+0.5j 2 2.2 2 False c c 0.0+1.0j}`
- `rdatajl/data_v3/types_ascii.rds` — rds · ascii · v3 · gzip · R 3.5.2 · enc UTF-8; 204 B; rdata 1.1.0: parse+convert OK; reads as `DataFrame(2, 6) columns=[np.str_('num'), np.str_('int'), np.str_('logi'), np.str_('chr'), np.str_('factor'), np.str_('cplx')] dtypes=['float64', 'Int32', 'boolean', 'string', 'category', 'complex128'] …`

#### `types_decomp`

Same `df` as `types`, `saveRDS(compress = FALSE)` (uncompressed XDR).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): same as types.

- `rdatajl/data_v2/types_decomp.rds` — rds · xdr · v2 · none · R 3.5.2; 436 B; rdata 1.1.0: parse+convert OK; reads as `DataFrame(2, 6) columns=[np.str_('num'), np.str_('int'), np.str_('logi'), np.str_('chr'), np.str_('factor'), np.str_('cplx')] dtypes=['float64', 'Int32', 'boolean', 'string', 'category', 'complex128'] …`
- `rdatajl/data_v3/types_decomp.rds` — rds · xdr · v3 · none · R 3.5.2 · enc UTF-8; 445 B; rdata 1.1.0: parse+convert OK; reads as `DataFrame(2, 6) columns=[np.str_('num'), np.str_('int'), np.str_('logi'), np.str_('chr'), np.str_('factor'), np.str_('cplx')] dtypes=['float64', 'Int32', 'boolean', 'string', 'category', 'complex128'] …`

#### `NAs`

`types` df with row 2 set to NA, row 3 = row 2 but `num = NaN`, and `cplx = complex(real = c(1.1, 1, NaN), imaginary = c(NA, NaN, 0))`. Distinguishes NA_real_ from NaN, and has NA in int/logical/character/factor/complex.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): num = [1.1, missing, NaN]; int/logi/chr/factor = [value, missing, missing]; cplx = [missing, ComplexF64(1, NaN), NaN].

- `rdatajl/data_v2/NAs.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.5.2; 239 B; rdata 1.1.0: parse+convert OK
- `rdatajl/data_v3/NAs.rda` — rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 249 B; rdata 1.1.0: parse+convert OK

#### `NAs_ascii`

Same as `NAs`, uncompressed ASCII (`NA`, `NaN` tokens).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): same as NAs.

- `rdatajl/data_v2/NAs_ascii.rda` — rda `RDA2` · ascii · v2 · none · R 3.5.2; 443 B; rdata 1.1.0: parse+convert OK; reads as `{'df': num int logi chr factor cplx 1 1.1 1 True ab ab 1.1+NaNj 2 NaN <NA> <NA> <NA> NaN 1.0+NaNj 3 NaN <NA> <NA> <NA> NaN NaN+0.0j}`
- `rdatajl/data_v3/NAs_ascii.rda` — rda `RDA3` · ascii · v3 · none · R 3.5.2 · enc UTF-8; 451 B; rdata 1.1.0: parse+convert OK; reads as `{'df': num int logi chr factor cplx 1 1.1 1 True ab ab 1.1+NaNj 2 NaN <NA> <NA> <NA> NaN 1.0+NaNj 3 NaN <NA> <NA> <NA> NaN NaN+0.0j}`

#### `names`

The NAs df with `names(df) <- c("end", "!", "1", "%_B*\tC*", NA, "x")`. The names attr holds a TAB and an NA_character_.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): RData.jl sanitizes them to `["_end", "x!", "x1", "_B_C_", "x", "x_1"]`. The raw R names are the six strings above.

- `rdatajl/data_v2/names.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.5.2; 240 B; rdata 1.1.0: parse+convert OK
- `rdatajl/data_v3/names.rda` — rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 246 B; rdata 1.1.0: parse+convert OK

#### `names_ascii`

Same as `names`, uncompressed ASCII (the TAB is written escaped).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): same as names.

- `rdatajl/data_v2/names_ascii.rda` — rda `RDA2` · ascii · v2 · none · R 3.5.2; 429 B; rdata 1.1.0: parse+convert OK; reads as `{'df': end ! 1 %_B*\tC* NaN x 1 1.1 1 True ab ab 1.1+NaNj 2 NaN <NA> <NA> <NA> NaN 1.0+NaNj 3 NaN <NA> <NA> <NA> NaN NaN+0.0j}`
- `rdatajl/data_v3/names_ascii.rda` — rda `RDA3` · ascii · v3 · none · R 3.5.2 · enc UTF-8; 437 B; rdata 1.1.0: parse+convert OK; reads as `{'df': end ! 1 %_B*\tC* NaN x 1 1.1 1 True ab ab 1.1+NaNj 2 NaN <NA> <NA> <NA> NaN 1.0+NaNj 3 NaN <NA> <NA> <NA> NaN NaN+0.0j}`

#### `envs`

Five environments: `empty.env`, `empty_nohash.env` (both `new.env(parent = emptyenv())`), `empty_child.env` (parent `empty.env`, so a back-reference REFSXP), `a.env` (`a = 43`, parent emptyenv), `b.env` (`b = 48`, parent `a.env`).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): loads without error (no value assertions).

- `rdatajl/data_v2/envs.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.5.2; 148 B; rdata 1.1.0: parse+convert OK; reads as `{'empty.env': REnvironment({}, REnvironment({})), 'empty_nohash.env': REnvironment({}, REnvironment({})), 'empty_child.env': REnvironment({}, REnvironment({}, REnvironment({}))), 'a.env': REnvironment …`
- `rdatajl/data_v3/envs.rda` — rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 157 B; rdata 1.1.0: parse+convert OK; reads as `{'empty.env': REnvironment({}, REnvironment({})), 'empty_nohash.env': REnvironment({}, REnvironment({})), 'empty_child.env': REnvironment({}, REnvironment({}, REnvironment({}))), 'a.env': REnvironment …`

#### `pairlists`

`pl.empty <- pairlist()` (NULL) and `pl <- pairlist(u = 1L, x = c(2.0, 3.0), y = "A", NULL, z = TRUE)`, a LISTSXP with one untagged NULL element.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): loads without error.

- `rdatajl/data_v2/pairlists.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.5.2; 115 B; rdata 1.1.0: parses; conversion fails: TypeError("'list' object is not a mapping")
- `rdatajl/data_v3/pairlists.rda` — rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 123 B; rdata 1.1.0: parses; conversion fails: TypeError("'list' object is not a mapping")

#### `closures`

Closures `test.fun0 <- function() return(NULL)`, `test.fun1 <- function(x) return(x + 1)`, `test.fun2 <- function(x, y) x + y` (with srcref attributes).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): loads without error.

- `rdatajl/data_v2/closures.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.5.2; 285 B; rdata 1.1.0: parses; conversion fails: AssertionError()
- `rdatajl/data_v3/closures.rda` — rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 295 B; rdata 1.1.0: parses; conversion fails: AssertionError()

#### `cmpfun`

The same three functions byte-compiled with `compiler::cmpfun` (BCODESXP).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): loads without error.

- `rdatajl/data_v2/cmpfun.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.5.2; 289 B; rdata 1.1.0: parses; conversion fails: AssertionError()
- `rdatajl/data_v3/cmpfun.rda` — rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 298 B; rdata 1.1.0: parses; conversion fails: AssertionError()

#### `ord`

`x <- factor(c("a", "b", "c"))`, `y <- ordered(x, levels = c("b", "a", "c"))`.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): x unordered with levels a,b,c; y ordered with levels b,a,c; both equal ["a","b","c"].

- `rdatajl/data_v2/ord.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.5.2; 145 B; rdata 1.1.0: parse+convert OK; reads as `{'x': ['a', 'b', 'c'] Categories (3, str): ['a', 'b', 'c'], 'y': ['a', 'b', 'c'] Categories (3, str): ['b' < 'a' < 'c']}`
- `rdatajl/data_v3/ord.rda` — rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 152 B; rdata 1.1.0: parse+convert OK; reads as `{'x': ['a', 'b', 'c'] Categories (3, str): ['a', 'b', 'c'], 'y': ['a', 'b', 'c'] Categories (3, str): ['b' < 'a' < 'c']}`

#### `numdates`

`as.Date(as.numeric(1:4), origin = "2017-01-01")`: a double vector 17168..17171 with class Date.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): 2017-01-02 … 2017-01-05.

- `rdatajl/data_v2/numdates.rds` — rds · xdr · v2 · gzip · R 3.5.2; 84 B; rdata 1.1.0: parse+convert OK; reads as `array([17168., 17169., 17170., 17171.])`
- `rdatajl/data_v3/numdates.rds` — rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 91 B; rdata 1.1.0: parse+convert OK; reads as `array([17168., 17169., 17170., 17171.])`

#### `numdates_ascii`

Same as `numdates`, `saveRDS(ascii = TRUE)` (gzip-compressed ASCII).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): same.

- `rdatajl/data_v2/numdates_ascii.rds` — rds · ascii · v2 · gzip · R 3.5.2; 85 B; rdata 1.1.0: parse+convert OK; reads as `array([17168., 17169., 17170., 17171.])`
- `rdatajl/data_v3/numdates_ascii.rds` — rds · ascii · v3 · gzip · R 3.5.2 · enc UTF-8; 95 B; rdata 1.1.0: parse+convert OK; reads as `array([17168., 17169., 17170., 17171.])`

#### `intdates`

`seq.Date(as.Date("2017-01-02"), by = "day", length.out = 4)`: an *integer*-backed Date, 17168L..17171L.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): same dates.

- `rdatajl/data_v2/intdates.rds` — rds · xdr · v2 · gzip · R 3.5.2; 79 B; rdata 1.1.0: parse+convert OK; reads as `array([17168, 17169, 17170, 17171], dtype=int32)`
- `rdatajl/data_v3/intdates.rds` — rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 85 B; rdata 1.1.0: parse+convert OK; reads as `array([17168, 17169, 17170, 17171], dtype=int32)`

#### `intdates_ascii`

Same as `intdates`, gzip-compressed ASCII.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): same.

- `rdatajl/data_v2/intdates_ascii.rds` — rds · ascii · v2 · gzip · R 3.5.2; 85 B; rdata 1.1.0: parse+convert OK; reads as `array([17168, 17169, 17170, 17171], dtype=int32)`
- `rdatajl/data_v3/intdates_ascii.rds` — rds · ascii · v3 · gzip · R 3.5.2 · enc UTF-8; 95 B; rdata 1.1.0: parse+convert OK; reads as `array([17168, 17169, 17170, 17171], dtype=int32)`

#### `datesNA`

`list(c(numdates, NA), c(datetimes, NA))` where `datetimes <- as.POSIXct("2017-01-01 13:23", tz = "UTC") + 1:4`.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): dates 2017-01-02..05 + missing; datetimes 2017-01-01T13:23:01..04 UTC + missing.

- `rdatajl/data_v2/datesNA.rds` — rds · xdr · v2 · gzip · R 3.5.2; 139 B; rdata 1.1.0: parse+convert OK; reads as `[array([17168., 17169., 17170., 17171., nan]), array([1.48327698e+09, 1.48327698e+09, 1.48327698e+09, 1.48327698e+09, nan])]`
- `rdatajl/data_v3/datesNA.rds` — rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 145 B; rdata 1.1.0: parse+convert OK; reads as `[array([17168., 17169., 17170., 17171., nan]), array([1.48327698e+09, 1.48327698e+09, 1.48327698e+09, 1.48327698e+09, nan])]`

#### `datetimes`

`list(datetimes, datetimes[1], <named datetimes A..D>, <named datetimes[1]>)` (POSIXct with tzone "UTC").

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): elements 3 and 4 carry names A–D / A.

- `rdatajl/data_v2/datetimes.rds` — rds · xdr · v2 · gzip · R 3.5.2; 172 B; rdata 1.1.0: parse+convert OK; reads as `[array([1.48327698e+09, 1.48327698e+09, 1.48327698e+09, 1.48327698e+09]), array([1.48327698e+09]), <xarray.DataArray (dim_0: 4)> Size: 32B array([1.48327698e+09, 1.48327698e+09, 1.48327698e+09, 1.4832 …`
- `rdatajl/data_v3/datetimes.rds` — rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 177 B; rdata 1.1.0: parse+convert OK; reads as `[array([1.48327698e+09, 1.48327698e+09, 1.48327698e+09, 1.48327698e+09]), array([1.48327698e+09]), <xarray.DataArray (dim_0: 4)> Size: 32B array([1.48327698e+09, 1.48327698e+09, 1.48327698e+09, 1.4832 …`

#### `datedfs`

`list(data.frame(date = numdates[1], datetime = datetimes[1]), data.frame(date = numdates, datetime = datetimes))`.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): two data frames with Date + POSIXct columns.

- `rdatajl/data_v2/datedfs.rds` — rds · xdr · v2 · gzip · R 3.5.2; 243 B; rdata 1.1.0: parse+convert OK; reads as `[ date datetime A 17168.0 1.483277e+09, date datetime A 17168.0 1.483277e+09 B 17169.0 1.483277e+09 C 17170.0 1.483277e+09 D 17171.0 1.483277e+09]`
- `rdatajl/data_v3/datedfs.rds` — rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 250 B; rdata 1.1.0: parse+convert OK; reads as `[ date datetime A 17168.0 1.483277e+09, date datetime A 17168.0 1.483277e+09 B 17169.0 1.483277e+09 C 17170.0 1.483277e+09 D 17171.0 1.483277e+09]`

#### `datetimes_tz`

Written with `TZ = America/Los_Angeles`: `list(as.POSIXct("2017-01-01 13:23"), as.POSIXct(..., tz = "CST"), as.POSIXct(..., tz = "America/Chicago"))`. The first has no tzone attribute.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): [1] = 2017-01-01T21:23 UTC; [3] = 13:23 America/Chicago.

- `rdatajl/data_v2/datetimes_tz.rds` — rds · xdr · v2 · gzip · R 3.5.2; 145 B; rdata 1.1.0: parse+convert OK; reads as `[array([1.48330578e+09]), array([1.48327698e+09]), array([1.48329858e+09])]`
- `rdatajl/data_v3/datetimes_tz.rds` — rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8; 151 B; rdata 1.1.0: parse+convert OK; reads as `[array([1.48330578e+09]), array([1.48327698e+09]), array([1.48329858e+09])]`

#### `dfattributes`

A data.frame of four labelled numeric columns (attributes `labels`, `label`, `na_values`, `na_range`, `comment`, `units`, `custom`) plus frame-level `comment` and a POSIXct `collectiontimes` attribute. **Both copies are v3.**

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): metadata comment = "This is a data frame", collectiontimes = 2022-05-25/26 22:05 UTC, v1 label "V1" with labels a=1,b=2,c=3, and so on.

- `rdatajl/data_v2/dfattributes.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8; 406 B; rdata 1.1.0: parse+convert OK
- `rdatajl/data_v3/dfattributes.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8; 406 B; rdata 1.1.0: parse+convert OK

#### `dotdotdot`

`...` captured inside `savedotdotdot(1, 2, 3)` via `get("...", environment())`. Verified by hexdump: a DOTSXP (0x11) whose three elements are REALSXP 1, 2, 3 (no promises).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): an `RData.RDot`.

- `rdatajl/data_v2/dotdotdot.rda` — rda `RDX2` · xdr · v2 · gzip · R 4.1.2; 79 B; rdata 1.1.0: parses; conversion fails: NotImplementedError('Type RObjectType.DOT not implemented')
- `rdatajl/data_v3/dotdotdot.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8; 89 B; rdata 1.1.0: parses; conversion fails: NotImplementedError('Type RObjectType.DOT not implemented')

#### `expr`

`parse(text = "a+b\nx+2")`: an EXPRSXP with `srcref`/`srcfile`/`wholeSrcref` attributes. srcfile is an environment that holds a `parseData` object and POSIXct timestamps.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): an `RExprList`.

- `rdatajl/data_v2/expr.rda` — rda `RDX2` · xdr · v2 · gzip · R 4.1.2; 574 B; rdata 1.1.0: parses; conversion fails: AssertionError()
- `rdatajl/data_v3/expr.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8; 584 B; rdata 1.1.0: parses; conversion fails: AssertionError()

#### `budworm_glm`

A fitted `glm(SF ~ sex * ldose, family = binomial)` on the 12-row budworm data (class `c("glm", "lm")`). It includes formula/terms environments, family closures, qr, and model frames. rdata cannot parse it (it contains UNBOUNDVALUE_SXP, 252).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): `RData.class(budworm) == ["glm", "lm"]`.

- `rdatajl/data_v2/budworm_glm.rda` — rda `RDX2` · xdr · v2 · gzip · R 4.1.2; 6572 B; rdata 1.1.0: parse FAILS: ValueError('252 is not a valid RObjectType')
- `rdatajl/data_v3/budworm_glm.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8; 6681 B; rdata 1.1.0: parse FAILS: ValueError('252 is not a valid RObjectType')

#### `altrep`

`longseq <- 1:1000` (compact_intseq), `wrapvec <- .Internal(wrap_meta(c(1, 2.5, 3), TRUE, TRUE))` (wrap_real), `factoraltrep <- rep(factor(c("A", "B", NA, "C")), each = 5000)` (length 20,000), `nonnilpairlist <- .Internal(wrap_meta(matrix(integer(), nrow = 0, ncol = 10), TRUE, TRUE))` (a wrap ALTREP of a 0×10 integer matrix; the script's comment says it "contains non-standard pairlist that doesn't end with NILVALUE_SXP").

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): wrapvec == [1.0, 2.5, 3.0]; factoraltrep == repeat(["A","B",missing,"C"], inner = 5000); nonnilpairlist is a 0×10 Int32 matrix.

- `rdatajl/data_v3/altrep.rda` — rda `RDX3` · xdr · v3 · gzip · R 3.6.2 · enc UTF-8; 442 B; rdata 1.1.0: parse+convert OK; reads as `{'longseq': array([ 1, 2, 3, 4, ..., 997, 998, 999, 1000], shape=(1000,), dtype=int32), 'wrapvec': array([1. , 2.5, 3. ]), 'factoraltrep': ['A', 'A', 'A', 'A', 'A', ..., 'C', 'C', 'C', 'C', 'C'] Lengt …`

#### `altrep_ascii`

The same four objects saved with `ascii = TRUE, compress = TRUE`: **gzip-compressed ASCII** v3 with ALTREP.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): loads without error.

- `rdatajl/data_v3/altrep_ascii.rda` — rda `RDA3` · ascii · v3 · gzip · R 3.6.2 · enc UTF-8; 405 B; rdata 1.1.0: parse+convert OK; reads as `{'longseq': array([ 1, 2, 3, 4, ..., 997, 998, 999, 1000], shape=(1000,), dtype=int32), 'wrapvec': array([1. , 2.5, 3. ]), 'factoraltrep': ['A', 'A', 'A', 'A', 'A', ..., 'C', 'C', 'C', 'C', 'C'] Lengt …`

#### `altrep_names`

`altrepnames_list <- as.list(c(1, 2, 3))` whose `names` attribute is an ALTREP `wrap_string` of c("a", "b", "c"), plus `altrepnames_df <- as.data.frame(...)` with the same ALTREP names.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): data.frame names == ["a", "b", "c"].

- `rdatajl/data_v3/altrep_names.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.0.0 · enc UTF-8; 224 B; rdata 1.1.0: parse+convert OK

#### `list_of_vec`

`listofvec <- list(c(1, 2, NA), c(3, 4), c(5, 6, NA))`, `listofvec2 <- list(c(1L, 2L, NA), c(3, 4), c(5, 6, NA))`, `listofvec3 <- list(c(1L, 2L), c(3, 4))`, `listofvec4 <- list(c(1, 2), c(3, 4, 5))`, `namedlistofvec <- list(A = c(1, 2, NA), c(3, 4), B = c(5, 6, NA))`, and `testdf`, a data.frame `a = c("a","b","c")` with list columns `listascol` and `listascol2`.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): values as in the R code; namedlistofvec names are "A", "", "B".

- `rdatajl/data_v2/list_of_vec.rda` — rda `RDX2` · xdr · v2 · gzip · R 4.0.4; 292 B; rdata 1.1.0: parse+convert OK
- `rdatajl/data_v3/list_of_vec.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.0.4 · enc UTF-8; 304 B; rdata 1.1.0: parse+convert OK

#### `compact_seq`

`intseq1 <- 5L:200L` (compact_intseq), `intseq2 <- seq.int(-2L, 1000L, 2L)`, `realseq1 <- 110:300` (an integer compact seq despite the name), `realseq2 <- seq(110.5, 300.5)`, `realseq3 <- seq(110, 300, by = 0.5)`. Checked in the bytes: the only ALTREP class in the file is `compact_intseq`. realseq2 and realseq3 are plain REALSXP (`seq()` builds them as `from + 0L:n`).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): intseq1 === 5:200; intseq2 == -2:2:1000; realseq1 === 110:300; realseq2 == 110.5:300.5; realseq3 == 110.0:0.5:300.0.

- `rdatajl/data_v3/compact_seq.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8; 2313 B; rdata 1.1.0: parse+convert OK; reads as `{'intseq1': array([ 5, 6, 7, 8, ..., 197, 198, 199, 200], shape=(196,), dtype=int32), 'intseq2': array([ -2, 0, 2, 4, ..., 994, 996, 998, 1000], shape=(502,), dtype=int32), 'realseq1': array([110, 111 …`

#### `dup_levels`

A factor `dup_levels` whose `levels` attribute contains duplicates (class `labelled`/`factor`). **v2**, R 3.4.4. rdata parses it but pandas rejects the non-unique categories.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): dup_cat[1] == "Paced", dup_cat[2] == "Inferior", dup_cat[end] == "Anterior"; unique levels ["Inferior","Anterior","LBBB","Missing","NoSTUp","OtherSTUp","Paced"].

- `rdatajl/data_v3/dup_levels.rda` — rda `RDX2` · xdr · v2 · gzip · R 3.4.4; 272 B; rdata 1.1.0: parses; conversion fails: ValueError('Categorical categories must be unique')

#### `compressed_gzip`

`df <- data.frame(num = c(1.1, 2.2))`, `save(..., version = 3, compress = "gzip")`.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): `df == DataFrame(num = [1.1, 2.2])`.

- `rdatajl/data_v3/compressed_gzip.rda` — rda `RDX3` · xdr · v3 · gzip · R 3.6.1 · enc UTF-8; 139 B; rdata 1.1.0: parse+convert OK

#### `compressed_bzip2`

Same, `compress = "bzip2"`.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): same.

- `rdatajl/data_v3/compressed_bzip2.rda` — rda `RDX3` · xdr · v3 · bzip2 · R 3.6.1 · enc UTF-8; 161 B; rdata 1.1.0: parse+convert OK

#### `compressed_xz`

Same, `compress = "xz"`.

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): same.

- `rdatajl/data_v3/compressed_xz.rda` — rda `RDX3` · xdr · v3 · xz · R 3.6.1 · enc UTF-8; 180 B; rdata 1.1.0: parse+convert OK

#### `compressed_false`

Same, `compress = FALSE` (raw `RDX3\n` + XDR).

Expected (RData.jl `test/RDA.jl` / `test/RDS.jl`): same.

- `rdatajl/data_v3/compressed_false.rda` — rda `RDX3` · xdr · v3 · none · R 3.6.1 · enc UTF-8; 214 B; rdata 1.1.0: parse+convert OK

## 4. `zurds/` — pedrobtz/zurds fixtures (72 files)

Source: <https://github.com/pedrobtz/zurds/tree/f35d59311ce71ab6ebc97eb173393ac44cb775f2/tests/testthat/fixtures>. MIT (R package
`zurds`, `License: MIT + file LICENSE`). All files were written by **R 4.6.1** with `tools/make-fixtures.R`
(verbatim below). The upstream `manifest.csv` lists a sha256 for every file, and all 72 kept files match it.
This is the only source of zstd, of `.rda` in native binary (`RDB2`/`RDB3`), and of ascii/binary `.rds`
under every compression.

<details><summary><code>tools/make-fixtures.R</code> (verbatim)</summary>

```r
#!/usr/bin/env Rscript
# Writes the fixtures every stage tests against, with R itself as the
# writer (design section 14), and a manifest of what each file is.
#
# Usage: Rscript tools/make-fixtures.R [--full] [outdir]
#
# Without --full it writes the small set checked in under
# tests/testthat/fixtures/: every object once (XDR, version 3, gzip), one
# object across the whole format x version x compression matrix, and the
# .rda and lazy-load files. With --full it writes every object across the
# whole matrix, which CI does on oldrel, release and devel. The manifest
# records the writing R version and each file's SHA-256 (R >= 4.5).

args <- commandArgs(trailingOnly = TRUE)
full <- "--full" %in% args
args <- setdiff(args, "--full")
out <- if (length(args)) args[[1]] else file.path("tests", "testthat", "fixtures")
dir.create(out, recursive = TRUE, showWarnings = FALSE)
unlink(list.files(out, full.names = TRUE), recursive = TRUE)

# zstd is written only where this R was built with it.
compressions <- c("none", "gzip", "bzip2", "xz",
  if (!inherits(try(memCompress(as.raw(1L), "zstd"), silent = TRUE), "try-error")) "zstd")
formats <- c("xdr", "binary", "ascii")
versions <- c(2L, 3L)

# Objects written to every file. Kept small: the matrix multiplies them.
fixture_objects <- function() {
  e <- new.env(parent = emptyenv())
  assign("a", 1L, envir = e)
  list(
    null = NULL,
    logical = c(TRUE, FALSE, NA),
    integer = c(1L, -2L, NA, .Machine$integer.max),
    double = c(1.5, -0, NA, NaN, Inf, -Inf, 1e-300),
    complex = c(1 + 2i, NA, complex(real = Inf, imaginary = -1)),
    character = c("a", NA, "", "café", iconv("naïve", "UTF-8", "latin1")),
    bytes = `Encoding<-`("\xff\xfe", "bytes"),
    raw = as.raw(c(0, 1, 255)),
    list = list(1L, "b", list(TRUE, NULL)),
    named_list = list(x = 1, y = list(z = "deep")),
    # c() rather than 1:3, which would stay a compact (ALTREP) sequence.
    data_frame = data.frame(n = c(1L, 2L, 3L), s = c("x", "y", NA), stringsAsFactors = FALSE),
    factor = factor(c("lo", "hi", "lo"), levels = c("lo", "hi")),
    matrix = matrix(1:6, 2, dimnames = list(c("r1", "r2"), NULL)),
    compact_seq = 1:10,
    pairlist = pairlist(a = 1, b = "two"),
    symbol = quote(sym),
    language = quote(f(x, y = 2)),
    closure = local(function(x) x + 1, envir = globalenv()),
    bytecode = compiler::cmpfun(function(x) x * 2),
    environment = e,
    formula = y ~ x,
    external_pointer = methods::new("externalptr"),
    env_cycle = local({
      e <- new.env(parent = emptyenv())
      e$self <- e
      lockEnvironment(e)
      e
    }),
    promise = local({
      e <- new.env(parent = emptyenv())
      delayedAssign("p", stop("never forced"), assign.env = e)
      e
    }),
    builtin = sum,
    special = `if`,
    namespace_closure = stats::sd,
    package_env = as.environment("package:stats"),
    deferred_string = as.character(1:5),
    s4 = local({
      where <- new.env()
      methods::setClass("ZurdsFixture", representation(x = "numeric"), where = where)
      methods::new("ZurdsFixture", x = 1)
    })
  )
}

# The bytes of one stream in the given format and version.
stream_bytes <- function(obj, format, version) {
  switch(format,
    xdr = serialize(obj, NULL, version = version),
    binary = serialize(obj, NULL, xdr = FALSE, version = version),
    ascii = serialize(obj, NULL, ascii = TRUE, version = version)
  )
}

write_compressed <- function(bytes, path, compression) {
  con <- switch(compression,
    none = file(path, "wb"),
    gzip = gzfile(path, "wb"),
    bzip2 = bzfile(path, "wb"),
    xz = xzfile(path, "wb"),
    zstd = zstdfile(path, "wb")
  )
  on.exit(close(con))
  writeBin(bytes, con)
}

# An .rda: the magic line, then a serialized pairlist of name = value, as
# save() writes. Written by hand only for native binary (RDB3), which save()
# has no argument for; the others go through save() itself.
write_rda <- function(objs, path, format, version, compression) {
  if (format == "binary") {
    magic <- charToRaw(if (version == 2L) "RDB2\n" else "RDB3\n")
    bytes <- c(magic, serialize(as.pairlist(objs), NULL, xdr = FALSE, version = version))
    write_compressed(bytes, path, compression)
  } else {
    env <- list2env(objs)
    save(list = names(objs), file = path, envir = env, ascii = format == "ascii",
         version = version, compress = if (compression == "none") FALSE else compression)
  }
}

ext <- c(none = "", gzip = ".gz", bzip2 = ".bz2", xz = ".xz", zstd = ".zst")
rows <- list()
add <- function(file, kind, object, format, version, compression) {
  rows[[length(rows) + 1L]] <<- data.frame(
    file = file, kind = kind, object = object, format = format,
    version = version, compression = compression, stringsAsFactors = FALSE
  )
}

objs <- fixture_objects()
matrix_objects <- if (full) names(objs) else "named_list"

# Every object once in the default layout, as saveRDS() writes it.
for (nm in names(objs)) {
  f <- paste0(nm, ".rds")
  saveRDS(objs[[nm]], file.path(out, f))
  add(f, "rds", nm, "xdr", 3L, "gzip")
}

# A persistent reference: an environment written through a refhook, as
# saveRDS(refhook =) writes it. In the default layout only.
local({
  f <- "persistent.rds"
  saveRDS(new.env(), file.path(out, f), refhook = function(x) "zurds-ref")
  add(f, "rds", "persistent", "xdr", 3L, "gzip")
})

# CVE-2024-27322-style files: an unforced promise whose code is a call to
# system(), at the top level and nested in a list. R refuses to write a
# promise, so the XDR stream is built by hand: a PROMSXP cell (5) with a
# tag (+1024) for its environment (R_GlobalEnv, 253), its value unbound
# (252), and its code, system("echo zurds"), as a LANGSXP.
local({
  int <- function(...) writeBin(as.integer(c(...)), raw(), endian = "big")
  chars <- function(s) c(int(262153, nchar(s)), charToRaw(s)) # ASCII CHARSXP
  call <- c(int(6, 1), chars("system"), int(2, 16, 1), chars("echo zurds"), int(254))
  promise <- c(int(5 + 1024, 253, 252), call)
  # Written by this R, as the manifest records; readable from R 3.5.0.
  v <- unclass(getRversion())[[1]]
  header <- c(charToRaw("X\n"), int(3, sum(v * c(65536, 256, 1)), 197888, 5),
              charToRaw("UTF-8"))
  nested <- c(int(19, 2, 14, 1), writeBin(1, raw(), endian = "big"), promise)
  for (f in c("cve_top.rds", "cve_nested.rds")) {
    con <- gzfile(file.path(out, f), "wb")
    writeBin(c(header, if (f == "cve_top.rds") promise else nested), con)
    close(con)
    add(f, "rds", sub("[.]rds$", "", f), "xdr", 3L, "gzip")
  }
})

# The matrix.
for (nm in matrix_objects) for (fmt in formats) for (v in versions) for (comp in compressions) {
  f <- sprintf("%s-%s-v%d-%s.rds", nm, fmt, v, comp)
  write_compressed(stream_bytes(objs[[nm]], fmt, v), file.path(out, f), comp)
  add(f, "rds", nm, fmt, v, comp)
}

# .rda files: several objects, every format, both versions, gzip; and the
# default layout across every compression.
rda_objs <- objs[c("integer", "character", "data_frame", "named_list")]
for (fmt in formats) for (v in versions) {
  f <- sprintf("multi-%s-v%d-gzip.rda", fmt, v)
  write_rda(rda_objs, file.path(out, f), fmt, v, "gzip")
  add(f, "rda", paste(names(rda_objs), collapse = " "), fmt, v, "gzip")
}
for (comp in compressions) {
  f <- sprintf("multi-xdr-v3-%s.rda", comp)
  write_rda(rda_objs, file.path(out, f), "xdr", 3L, comp)
  add(f, "rda", paste(names(rda_objs), collapse = " "), "xdr", 3L, comp)
}

# zlib streams with what they decompress to, for the lazy-load decoder:
# fixed and dynamic codes, stored blocks, an empty stream, long matches.
# Written here so the tests need not call memCompress().
local({
  set.seed(1)
  plain <- list(
    empty = raw(0), tiny = charToRaw("a"), text = charToRaw(strrep("lazy-load ", 2000)),
    random = as.raw(sample(0:255, 20000, TRUE)), zeros = raw(70000),
    serialized = serialize(mtcars, NULL)
  )
  saveRDS(lapply(plain, function(p) list(plain = p, zlib = memCompress(p, "gzip"))),
          file.path(out, "zlib.rds"))
  add("zlib.rds", "rds", "zlib", "xdr", 3L, "gzip")
})

# Lazy-load databases written by hand: one entry an unforced promise whose
# code would create a file, as R refuses to write one; then the same layout
# with bzip2 (compressed = 2) and xz (compressed = 3) entries, as
# makeLazyLoadDB() writes them.
local({
  int <- function(...) writeBin(as.integer(c(...)), raw(), endian = "big")
  chars <- function(s) c(int(262153, nchar(s)), charToRaw(s))
  v <- unclass(getRversion())[[1]]
  header <- c(charToRaw("X\n"), int(3, sum(v * c(65536, 256, 1)), 197888, 5),
              charToRaw("UTF-8"))
  call <- c(int(6, 1), chars("file.create"), int(2, 16, 1),
            chars("zurds-lazyload-promise-was-forced"), int(254))
  entries <- list(p = c(header, int(5 + 1024, 253, 252), call),
                  x = serialize(c(1L, 2L, 3L), NULL))
  con <- file(file.path(out, "lazyload_promise.rdb"), "wb")
  vars <- list()
  off <- 0
  for (name in names(entries)) {
    s <- entries[[name]]
    bytes <- c(writeBin(length(s), raw(), endian = "big"), memCompress(s, "gzip"))
    writeBin(bytes, con)
    vars[[name]] <- as.integer(c(off, length(bytes)))
    off <- off + length(bytes)
  }
  close(con)
  saveRDS(list(variables = vars, references = list(), compressed = TRUE),
          file.path(out, "lazyload_promise.rdx"))
  for (f in c("lazyload_promise.rdb", "lazyload_promise.rdx"))
    add(f, "lazyload", "p x", "xdr", 3L, "zlib")
  for (comp in 2:3) {
    base <- paste0("lazyload_c", comp)
    e <- list2env(objs[c("integer", "data_frame", "character")])
    tools:::makeLazyLoadDB(e, file.path(out, base), compress = comp)
    for (f in paste0(base, c(".rdb", ".rdx")))
      add(f, "lazyload", paste(ls(e), collapse = " "), "xdr", 3L, c("bzip2", "xz")[comp - 1])
  }
})

# A lazy-load database, as R CMD INSTALL builds a package's.
ll_env <- list2env(objs[c("integer", "data_frame", "closure")])
tools:::makeLazyLoadDB(ll_env, file.path(out, "lazyload"))
for (f in c("lazyload.rdb", "lazyload.rdx")) {
  add(f, "lazyload", paste(ls(ll_env), collapse = " "), "xdr", 3L, "zlib")
}

manifest <- do.call(rbind, rows)
manifest$writer <- as.character(getRversion())
sha <- if (exists("sha256sum", asNamespace("tools"))) tools::sha256sum else function(f) NA_character_
manifest$sha256 <- unname(sha(file.path(out, manifest$file)))
write.csv(manifest, file.path(out, "manifest.csv"), row.names = FALSE)
message(nrow(manifest), " fixtures written to ", out)
```
</details>

#### Single objects (`saveRDS()` defaults: XDR, v3, gzip, R 4.6.1)

| file | R object (from `fixture_objects()`) | rdata 1.1.0 |
|---|---|---|
| `null.rds` (44 B) | `NULL` | parse+convert OK: `None` |
| `logical.rds` (52 B) | `c(TRUE, FALSE, NA)` | parse+convert OK: `masked_array(data=[True, False, --], mask=[False, False, True], fill_value=True)` |
| `integer.rds` (59 B) | `c(1L, -2L, NA, .Machine$integer.max)` | parse+convert OK: `masked_array(data=[1, -2, --, 2147483647], mask=[False, False, True, False], fill_value=-2147483648, dtype=int32)` |
| `double.rds` (78 B) | `c(1.5, -0, NA, NaN, Inf, -Inf, 1e-300)` (NA_real_ vs NaN, signed zero) | parse+convert OK: `array([ 1.5e+000, -0.0e+000, nan, nan, inf, -inf, 1.0e-300])` |
| `complex.rds` (67 B) | `c(1 + 2i, NA, complex(real = Inf, imaginary = -1))` | parse+convert OK: `array([ 1.+2.j, nan+0.j, inf-1.j])` |
| `character.rds` (81 B) | `c("a", NA, "", "café", iconv("naïve", "UTF-8", "latin1"))` (UTF-8 and LATIN1-flagged CHARSXPs) | parse+convert OK: `array(['a', None, '', 'café', 'naïve'], dtype=object)` |
| `bytes.rds` (56 B) | ``Encoding<-``("\xff\xfe", "bytes")`` (BYTES-flagged CHARSXP) | parse+convert OK: `array([b'\xff\xfe'], dtype='¦S2')` |
| `raw.rds` (48 B) | `as.raw(c(0, 1, 255))`. Verified by hexdump: RAWSXP (0x18), length 3, bytes `00 01 ff` | parse FAILS: NotImplementedError('Type RObjectType.RAW not implemented') |
| `list.rds` (70 B) | `list(1L, "b", list(TRUE, NULL))` | parse+convert OK: `[array([1], dtype=int32), array(['b'], dtype='<U1'), [array([ True]), None]]` |
| `named_list.rds` (107 B) | `list(x = 1, y = list(z = "deep"))` | parse+convert OK: `{np.str_('x'): array([1.]), np.str_('y'): {np.str_('z'): array(['deep'], dtype='<U4')}}` |
| `data_frame.rds` (138 B) | `data.frame(n = c(1L, 2L, 3L), s = c("x", "y", NA), stringsAsFactors = FALSE)` | parse+convert OK: `DataFrame(3, 2) columns=[np.str_('n'), np.str_('s')] dtypes=['Int32', 'string'] n s 1 1 x 2 2 y 3 3 <NA>` |
| `factor.rds` (101 B) | `factor(c("lo", "hi", "lo"), levels = c("lo", "hi"))` | parse+convert OK: `['lo', 'hi', 'lo'] Categories (2, str): ['lo', 'hi']` |
| `matrix.rds` (110 B) | `matrix(1:6, 2, dimnames = list(c("r1", "r2"), NULL))` | parse+convert OK: `<xarray.DataArray (dim_0: 2, dim_1: 3)> Size: 24B array([[1, 3, 5], [2, 4, 6]], dtype=int32) Coordinates: * dim_0 (dim_0) <U2 16B 'r1' 'r2'  …` |
| `compact_seq.rds` (99 B) | `1:10` (ALTREP compact_intseq) | parse+convert OK: `array([ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], dtype=int32)` |
| `deferred_string.rds` (134 B) | `as.character(1:5)` (ALTREP deferred_string) | parse+convert OK: `array(['1', '2', '3', '4', '5'], dtype='<U1')` |
| `pairlist.rds` (78 B) | `pairlist(a = 1, b = "two")` | parse+convert OK: `{'a': array([1.]), 'b': array(['two'], dtype='<U3')}` |
| `symbol.rds` (53 B) | `quote(sym)` | parse+convert OK: `'sym'` |
| `language.rds` (74 B) | `quote(f(x, y = 2))` | parse+convert OK: `RLanguage(elements=['f', 'x', 'y'], attributes={})` |
| `formula.rds` (142 B) | `y ~ x` (LANGSXP `~` with attributes `class = "formula"` and `.Environment`) | parse+convert OK: `RLanguage(elements=['~', 'y', 'x'], attributes={'class': array(['formula'], dtype='<U7'), '.Environment': REnvironment({}, REnvironment({})) …` |
| `closure.rds` (88 B) | `local(function(x) x + 1, envir = globalenv())` | parses; conversion fails: AssertionError() |
| `bytecode.rds` (201 B) | `compiler::cmpfun(function(x) x * 2)` | parses; conversion fails: AssertionError() |
| `environment.rds` (80 B) | env with `a = 1L`, parent `emptyenv()` | parse+convert OK: `REnvironment({'a': array([1], dtype=int32)}, REnvironment({}))` |
| `env_cycle.rds` (81 B) | locked env `e` with `e$self <- e` (self-reference via REFSXP) | parse+convert OK: `REnvironment({'self': REnvironment({'self': UnresolvedReference(references={<id>: None, <id>: None, <id>: REnvironment({}), <id>: None, <id> …` |
| `promise.rds` (137 B) | env (parent emptyenv) holding an unforced promise `delayedAssign("p", stop("never forced"))`. R-written, unforced PROMSXP inside an ENVSXP | parse FAILS: ValueError('252 is not a valid RObjectType') |
| `external_pointer.rds` (48 B) | `methods::new("externalptr")` | parse+convert OK: `RExternalPointer(protected=None, tag=None)` |
| `builtin.rds` (49 B) | `sum` (BUILTINSXP) | parse+convert OK: `RBuiltin(name='sum')` |
| `special.rds` (49 B) | `` `if` `` (SPECIALSXP) | parse+convert OK: `RBuiltin(name='if')` |
| `namespace_closure.rds` (443 B) | `stats::sd` (closure whose env is NAMESPACESXP stats) | parses; conversion fails: AssertionError() |
| `package_env.rds` (67 B) | `as.environment("package:stats")`. Verified by hexdump: PACKAGESXP (0xf8) + STRSXP `"package:stats"` | parse FAILS: ValueError('248 is not a valid RObjectType') |
| `s4.rds` (125 B) | `new("ZurdsFixture", x = 1)` (S4SXP, class attr with package attr) | parse+convert OK: `namespace(x=array([1.]), class=array(['ZurdsFixture'], dtype='<U12'))` |
| `persistent.rds` (63 B) | `saveRDS(new.env(), refhook = function(x) "zurds-ref")`. Verified by hexdump: PERSISTSXP (0xf7) + STRSXP `"zurds-ref"` | parse FAILS: ValueError('247 is not a valid RObjectType') |
| `zlib.rds` (23783 B) | `set.seed(1)`; named list `empty`/`tiny`/`text`/`random`/`zeros`/`serialized`, each `list(plain = <raw>, zlib = memCompress(plain, "gzip"))`. Plain raws: `raw(0)`, `charToRaw("a")`, 20,000 bytes of `"lazy-load "`×2000, 20,000 random bytes, `raw(70000)`, `serialize(mtcars, NULL)` | parse FAILS: NotImplementedError('Type RObjectType.RAW not implemented') |

#### Format × version × compression matrix (`named_list`, 30 files)

Object: `list(x = 1, y = list(z = "deep"))`. Each file is `write_compressed(stream_bytes(obj, fmt, v), path, comp)`:
`serialize(obj, NULL, xdr = …, ascii = …, version = v)` bytes written through `file()`/`gzfile()`/`bzfile()`/`xzfile()`/`zstdfile()`.
**Verified:** within each format/version pair, the decompressed payloads of all five compressions are byte-identical
(sha256). The xdr-v3 payload equals the payload of `named_list.rds`. rdata reads every one as
`{'x': array([1.]), 'y': {'z': array(['deep'])}}`.

| file | format | bytes | rdata 1.1.0 |
|---|---|---|---|
| `named_list-ascii-v2-bzip2.rds` | rds · ascii · v2 · bzip2 | 106 | parse+convert OK |
| `named_list-ascii-v2-gzip.rds` | rds · ascii · v2 · gzip | 98 | parse+convert OK |
| `named_list-ascii-v2-none.rds` | rds · ascii · v2 · none | 138 | parse+convert OK |
| `named_list-ascii-v2-xz.rds` | rds · ascii · v2 · xz | 144 | parse+convert OK |
| `named_list-ascii-v2-zstd.rds` | rds · ascii · v2 · zstd | 94 | parse+convert OK (zstd decompressed first) |
| `named_list-ascii-v3-bzip2.rds` | rds · ascii · v3 · bzip2 | 118 | parse+convert OK |
| `named_list-ascii-v3-gzip.rds` | rds · ascii · v3 · gzip | 109 | parse+convert OK |
| `named_list-ascii-v3-none.rds` | rds · ascii · v3 · none | 146 | parse+convert OK |
| `named_list-ascii-v3-xz.rds` | rds · ascii · v3 · xz | 152 | parse+convert OK |
| `named_list-ascii-v3-zstd.rds` | rds · ascii · v3 · zstd | 102 | parse+convert OK (zstd decompressed first) |
| `named_list-binary-v2-bzip2.rds` | rds · binary · v2 · bzip2 · little-endian | 108 | parse+convert OK |
| `named_list-binary-v2-gzip.rds` | rds · binary · v2 · gzip · little-endian | 97 | parse+convert OK |
| `named_list-binary-v2-none.rds` | rds · binary · v2 · none · little-endian | 146 | parse+convert OK |
| `named_list-binary-v2-xz.rds` | rds · binary · v2 · xz · little-endian | 136 | parse+convert OK |
| `named_list-binary-v2-zstd.rds` | rds · binary · v2 · zstd · little-endian | 103 | parse+convert OK (zstd decompressed first) |
| `named_list-binary-v3-bzip2.rds` | rds · binary · v3 · bzip2 · little-endian | 122 | parse+convert OK |
| `named_list-binary-v3-gzip.rds` | rds · binary · v3 · gzip · little-endian | 106 | parse+convert OK |
| `named_list-binary-v3-none.rds` | rds · binary · v3 · none · little-endian | 155 | parse+convert OK |
| `named_list-binary-v3-xz.rds` | rds · binary · v3 · xz · little-endian | 148 | parse+convert OK |
| `named_list-binary-v3-zstd.rds` | rds · binary · v3 · zstd · little-endian | 110 | parse+convert OK (zstd decompressed first) |
| `named_list-xdr-v2-bzip2.rds` | rds · xdr · v2 · bzip2 | 111 | parse+convert OK |
| `named_list-xdr-v2-gzip.rds` | rds · xdr · v2 · gzip | 96 | parse+convert OK |
| `named_list-xdr-v2-none.rds` | rds · xdr · v2 · none | 146 | parse+convert OK |
| `named_list-xdr-v2-xz.rds` | rds · xdr · v2 · xz | 136 | parse+convert OK |
| `named_list-xdr-v2-zstd.rds` | rds · xdr · v2 · zstd | 103 | parse+convert OK (zstd decompressed first) |
| `named_list-xdr-v3-bzip2.rds` | rds · xdr · v3 · bzip2 | 123 | parse+convert OK |
| `named_list-xdr-v3-gzip.rds` | rds · xdr · v3 · gzip | 107 | parse+convert OK |
| `named_list-xdr-v3-none.rds` | rds · xdr · v3 · none | 155 | parse+convert OK |
| `named_list-xdr-v3-xz.rds` | rds · xdr · v3 · xz | 148 | parse+convert OK |
| `named_list-xdr-v3-zstd.rds` | rds · xdr · v3 · zstd | 109 | parse+convert OK (zstd decompressed first) |

#### Multi-object `.rda` (10 files)

Each file holds four objects, saved in this order: `integer` = `c(1L, -2L, NA, .Machine$integer.max)`;
`character` = `c("a", NA, "", "café", iconv("naïve", "UTF-8", "latin1"))`;
`data_frame` = `data.frame(n = c(1L, 2L, 3L), s = c("x", "y", NA), stringsAsFactors = FALSE)`;
`named_list` = `list(x = 1, y = list(z = "deep"))`. The `xdr`/`ascii` files come from
`save(list = names(objs), ascii =, version =, compress =)`. The `binary` ones are `"RDB2\n"`/`"RDB3\n"` +
`serialize(as.pairlist(objs), NULL, xdr = FALSE, version = v)`, gzip-compressed. The payloads of
`multi-xdr-v3-{none,gzip,bzip2,xz,zstd}.rda` are byte-identical once decompressed.

- `zurds/multi-ascii-v2-gzip.rda` — rda `RDA2` · ascii · v2 · gzip · R 4.6.1; 252 B; rdata 1.1.0: parse+convert OK
- `zurds/multi-ascii-v3-gzip.rda` — rda `RDA3` · ascii · v3 · gzip · R 4.6.1 · enc UTF-8; 262 B; rdata 1.1.0: parse+convert OK
- `zurds/multi-binary-v2-gzip.rda` — rda `RDB2` · binary · v2 · gzip · R 4.6.1 · little-endian; 257 B; rdata 1.1.0: parse+convert OK
- `zurds/multi-binary-v3-gzip.rda` — rda `RDB3` · binary · v3 · gzip · R 4.6.1 · enc UTF-8 · little-endian; 272 B; rdata 1.1.0: parse+convert OK
- `zurds/multi-xdr-v2-gzip.rda` — rda `RDX2` · xdr · v2 · gzip · R 4.6.1; 255 B; rdata 1.1.0: parse+convert OK
- `zurds/multi-xdr-v3-bzip2.rda` — rda `RDX3` · xdr · v3 · bzip2 · R 4.6.1 · enc UTF-8; 277 B; rdata 1.1.0: parse+convert OK
- `zurds/multi-xdr-v3-gzip.rda` — rda `RDX3` · xdr · v3 · gzip · R 4.6.1 · enc UTF-8; 266 B; rdata 1.1.0: parse+convert OK
- `zurds/multi-xdr-v3-none.rda` — rda `RDX3` · xdr · v3 · none · R 4.6.1 · enc UTF-8; 535 B; rdata 1.1.0: parse+convert OK
- `zurds/multi-xdr-v3-xz.rda` — rda `RDX3` · xdr · v3 · xz · R 4.6.1 · enc UTF-8; 292 B; rdata 1.1.0: parse+convert OK
- `zurds/multi-xdr-v3-zstd.rda` — rda `RDX3` · xdr · v3 · zstd · R 4.6.1 · enc UTF-8; 262 B; rdata 1.1.0: parse+convert OK (zstd decompressed first)

rdata 1.1.0 reads `multi-*.rda` as: `{'integer': masked_array(data=[1, -2, --, 2147483647], mask=[False, False, True, False], fill_value=-2147483648, dtype=int32), 'character': array(['a', None, '', 'café', 'naïve'], dtype=object), 'data_frame': n s 1 1 x 2 2 y 3 3 <NA>, 'named_list': {np.str_('x'): array([1.]), np.str_('y'): {np.str_('z'): array(['deep'], dtype='<U4')}}}`

## 5. Seurat objects (`seurat/`)

### 5.1 `seurat/pbmc_small.rda` — SeuratObject's bundled example (real data)

- **Source:** CRAN <https://cran.r-project.org/src/contrib/SeuratObject_5.4.0.tar.gz> (current release; once
  superseded it moves to `https://cran.r-project.org/src/contrib/Archive/SeuratObject/SeuratObject_5.4.0.tar.gz`).
  Tarball md5 `bd4458cced57f2ebe9f11a07d206821e` (matches CRAN `PACKAGES`), sha256
  `11a748f7845115312021ee4e1e4c88f5a37546aa7d066f61652f09d81908f256`. Only `SeuratObject/data/pbmc_small.rda`
  was extracted. Published 2026-04-11. It is byte-identical (git blob `17849a29e456f5f03d9ee432571a685e139da1bd`) to
  [satijalab/seurat-object@b9a11fa `data/pbmc_small.rda`](https://github.com/satijalab/seurat-object/blob/b9a11fa46acc2142e142f08743b48c73a45b2742/data/pbmc_small.rda).
- **License:** MIT (SeuratObject authors). The data is a subset of 10x Genomics' 3k PBMC dataset (`R/data.R`).
- **Format:** `.rda`, magic `RDX2`, XDR, serialization **v2**, **bzip2**, writer **R 4.0.3**, 61,056 B
  (271,310 B decompressed). One object, `pbmc_small`.
- **rdata 1.1.0:** parse + convert OK (warnings only: missing constructors for Seurat/Assay/DimReduc/Graph/…).

Structure (all values below were read with rdata):

- `class(pbmc_small) == "Seurat"` (S4). Slots in serialized order: `assays, meta.data, active.assay, active.ident,
  graphs, neighbors, reductions, images, project.name, misc, version, commands, tools`.
- `@version`: package_version `4.0.0`. `@project.name` "SeuratProject". `@active.assay` "RNA".
- **Dimensions: 230 genes × 80 cells.** One assay, `RNA`, of class `Assay` (v3-style). Slots: `counts, data, scale.data, key,
  assay.orig, var.features, meta.features, misc`. `@key` "rna_". `@assay.orig` and `@misc` are the NULL-slot symbol.
  - `@counts`: dgCMatrix, Dim 230×80, **4,456 non-zeros**, total sum 19,633.
    Rownames start `MS4A1, CD79B, CD79A, HLA-DRA, TCL1A`, last `S100B`. Colnames (cells) start
    `ATGCCAGAACGACT, CATGGCCTGTGCAT, GAACCTGATGAACC, TGACTGGATTCTCA, AGTCAGACTGCACA`, last `CTTGATTGATCTTC`.
    The first non-zeros of cell 1 (`ATGCCAGAACGACT`) are CD79B=1, HLA-DQB1=1, **LTB=3**, SP100=1, CXCR4=1
    (`@i[0:5] = 1,5,8,11,…`, `@p[0:4] = 0,47,99,149`). The first non-zeros of the last cell (`CTTGATTGATCTTC`) are HLA-DRA=7, HLA-DQB1=1, NCF1=1.
    colSums of the first 5 cells are 70, 85, 87, 127, 173.
  - `@data`: dgCMatrix with the same pattern (4,456 nnz). Cell 1: CD79B = 4.968820743663249, LTB = 6.062788033933216.
  - `@scale.data`: dense double 20×80, `[1,1] = -0.1934951510503384`.
  - `@var.features`: 20 genes starting `PPBP, IGLL5, VDAC3, CD1C, AKR1C3`.
  - `@meta.features`: data.frame with columns `vst.mean, vst.variance, vst.variance.expected, vst.variance.standardized, vst.variable`.
- `@meta.data`: data.frame of 80 rows, row.names = cell names (character). Columns:
  `orig.ident` (factor, level "SeuratProject"), `nCount_RNA` (double: 70, 85, 87, 127, 173, …),
  `nFeature_RNA` (integer: 47, 52, 50, 56, 53, …), `RNA_snn_res.0.8` (factor, levels "0","1"),
  `letter.idents` (factor, levels "A","B"; first values A, A, B, A, A), `groups` (character: g2, g1, g2, g2, g2, …),
  `RNA_snn_res.1` (factor, levels "0","1","2").
  **Consistency checks that hold for all 80 cells:** `colSums(counts) == nCount_RNA` and non-zeros per cell `== nFeature_RNA`.
- `@active.ident`: factor with levels "0","1","2", counts **36 / 25 / 19**, named by cell.
- `@graphs$RNA_snn`: class `Graph` (dgCMatrix subclass with `@assay.used`), 80×80, 6,328 non-zeros.
- `@reductions`: `pca` (DimReduc, key "PC_", cell.embeddings 80×19, colnames PC_1…, `[1,1] = -0.7740370788151435`,
  feature.loadings 20×19, stdev[1] = 2.7868782266357597, `@jackstraw` is a JackStrawData with
  empirical.p.values 20×10) and `tsne` (DimReduc, key "tSNE_", cell.embeddings 80×2, `[1,1] = 0.8675977221985863`).
- `@commands` (10 SeuratCommand S4s): `NormalizeData.RNA, ScaleData.RNA, RunPCA.RNA, BuildSNN.RNA.pca, FindClusters,
  RunTSNE.pca, JackStraw.RNA.pca, ScoreJackStraw.pca, ProjectDim.RNA.pca, FindVariableFeatures.RNA`. Each has `@time.stamp`
  (POSIXct), `@call.string`, and `@params` (a named list). `FindClusters@assay.used` is the NULL-slot symbol. `@neighbors`, `@images`,
  `@misc` and `@tools` are empty lists.

### 5.2 `seurat/v5/` — Seurat v5 `Assay5` objects (cellgeni/py8rds tests)

- **Source:** <https://github.com/cellgeni/py8rds/tree/3ade6f8e2e6682343e48d146a710867849d7b5a8/tests> (MIT, © 2024 Cellular
  Genetics Informatics). Generated by `tests/create_test_data.R` with Seurat 5.3.0 and R 4.5.2. All three are `.rds`, XDR, v3,
  gzip, native encoding UTF-8. The data is synthetic, but the objects are genuine Seurat v5 objects:

```r
set.seed(1)
m <- matrix(rpois(20*12, 2), 20, 12, dimnames=list(paste0("g",1:20), paste0("c",1:12)))
m <- as(m, "dgCMatrix")
obj <- CreateSeuratObject(m)
obj$batch <- rep(c("B","A","B","C"), 3)
obj[["RNA"]] <- split(obj[["RNA"]], f = obj$batch)
obj <- NormalizeData(obj)
saveRDS(obj,'seu_split.rds')
write.csv(as.matrix(LayerData(JoinLayers(obj), "data")), "seu_split_joined_data.csv")

# split layers with different features
obj <- CreateSeuratObject(CreateAssay5Object(counts = list(m[1:15, 1:6], m[6:20, 7:12])))
saveRDS(obj,'seu_split_diff_features.rds')
write.csv(as.matrix(LayerData(JoinLayers(obj), "counts")), "seu_split_diff_features_joined_counts.csv")

# layers sharing cells, they are not split layers and cannot be concatenated
obj <- CreateSeuratObject(CreateAssay5Object(counts = list(raw = m, filtered = m[1:15, 1:6])))
saveRDS(obj,'seu_overlapping_layers.rds')
```

The common structure: `class "Seurat"`, `@version` 5.3.0, the same 13 slots in the same order as pbmc_small. `@assays$RNA` has class **`Assay5`**
with slots `layers, cells, features, default, assay.orig, meta.data, misc, key`. Layers are dgCMatrix objects with
**NULL dimnames**. Their cell and feature names live in `@cells` / `@features`, which are `LogMap` objects: logical matrices
whose dimnames are (names × layer names), and each column marks the members of a layer. `@meta.data` row.names are `c1…c12`.

| file | layers (Dim, nnz) | `@cells` / `@features` LogMap | other |
|---|---|---|---|
| `seu_split.rds` (2,016 B) | `counts.B` 20×6 (109), `counts.A` 20×3 (52), `counts.C` 20×3 (55), `data.B` 20×6 (109), `data.A` 20×3 (52), `data.C` 20×3 (55) | 12×6 (c1…c12) / 20×6 (g1…g20) | `@default` 3L. meta.data `orig.ident, nCount_RNA, nFeature_RNA, batch` (batch = B,A,B,C,…; nCount_RNA 45, 35, 39, 45, 38, …). `@commands$NormalizeData.RNA` (LogNormalize, scale.factor 1e4). `data.B@x[0:2]` = 5.408167787478715, 5.408167787478715 |
| `seu_split_diff_features.rds` (1,093 B) | `counts.1` 15×6 (81), `counts.2` 15×6 (79) | 12×2 / 20×2 | `@default` 1L. nCount_RNA 30, 25, 32, 28, 26, … |
| `seu_overlapping_layers.rds` (1,260 B) | `counts.raw` 20×12 (216), `counts.filtered` 15×6 (81) | 12×2 / 20×2 | `@default` 1L. nCount_RNA 75, 60, 71, 73, 64, … |

**Ground truth CSVs** (written by R's `write.csv`, so the header is `"","c1",…,"c12"` and rows are `"g1",…`):
`seu_split_joined_data.csv` is the 20×12 joined `data` layer, and `seu_split_diff_features_joined_counts.csv` is the 20×12 joined counts.
**Verified:** rebuilding the joined matrices from the split layers plus the LogMaps (read with rdata's parser) reproduces
all 240 CSV values. `data` matches with max abs diff 4.4e-15 (the CSV has 15 significant digits). For diff_features, the 180
covered entries match exactly and the 60 uncovered entries are 0 in the CSV.

### 5.3 `seurat/scRep_example.rda` — scRepertoire's example (real data, closures inside)

- **Source:** <https://github.com/ncborcherding/scRepertoire/blob/49459e4127ca2976c5fb75a40ecf2388d159c7e9/data/scRep_example.rda>
  (scRepertoire 2.6.1, MIT). "A Seurat Object of 500 Single T cells … from 4 patients with acute respiratory distress"
  (`R/data.R`).
- **Format:** `.rda`, `RDX3`, XDR v3, **xz**, writer R 4.3.0, UTF-8. 208,112 B (3,682,665 B decompressed).
- **rdata 1.1.0:** parses; high-level conversion fails (AssertionError on the closures below). Facts were read from the parse tree.
- `@version` 4.9.9.9091 (a Seurat v5 pre-release), but the assay is the v3-style `Assay`. **2000 genes × 500 cells.**
  `@assays$RNA@counts` is a dgCMatrix with 140,880 non-zeros and total 752,418. Genes start `TRBV7-2, HSPA1B, HSPA1A, TRBV4-1, CCL4`
  (last `BHLHE40`). Cells start `P17B_AAACCTGCAACACGCC-1, P17B_AAACGGGTCTCCAGGG-1, …` (last `P20L_TCTGGAAGTGGAAAGA-1`).
  The first non-zeros of cell 1 are TRBV10-3=7, TRAV38-2DV8=7, TRBV6-5=1, GNLY=1, DNAJB1=1. `@data` is identical to counts here
  (not log-normalized). `@scale.data` is 0×0. `@var.features` has 2000 genes.
- `@meta.data` columns: `orig.ident` (character "P17B", …), `nCount_RNA` (1685, 919, 1946, 773, 3053, …), `nFeature_RNA`
  (286, 217, 357, 209, 482, …), `mito.genes`, `RNA_snn_res.0.8` (factor, 17 levels in *character* sort order "1","10",…),
  `seurat_clusters` (factor, levels "1"…"17"). `colSums(counts) == nCount_RNA` and nnz per cell `== nFeature_RNA` for all 500 cells.
- `@reductions$umap`: cell.embeddings 500×2, key "umap_", `[1,1] = -5.478442098363359`. `@graphs` is empty.
- `@commands`: `RunUMAP.RNA.mnn, FindNeighbors.RNA.mnn, FindClusters, FindVariableFeatures.RNA`.
  **`FindVariableFeatures.RNA@params$mean.function` and `$dispersion.function` are closures (CLOSXP)**, which makes this a good
  real-world test for skipping functions inside an S4 object.

## 6. SingleCellExperiment objects (`sce/`)

### 6.1 `sce/mouseBrainSubsetSCE.rda` — primary SCE (real data; counts + logcounts + colData + reducedDims)

- **Source:** <https://github.com/compbiomed/singleCellTK/blob/6723890f56292752cb75397a988cc9721ec7ff71/data/mouseBrainSubsetSCE.rda>
  (singleCellTK 2.18.0, `License: MIT + file LICENSE`). Its documentation says: "A subset of 30 cells from a single cell RNA-Seq experiment from Zeisel, et al.
  Science 2015 … 15 of the cells were identified as oligodendrocytes and 15 … as microglia" (GSE60361).
- **Format:** `.rda`, `RDX2`, XDR **v2**, **bzip2**, writer R 4.3.1. 96,123 B (805,866 B decompressed). One object, `mouseBrainSubsetSCE`.
- **rdata 1.1.0:** parse + convert OK.
- `class "SingleCellExperiment"` (attr `package = "SingleCellExperiment"`). Slots in order: `int_metadata, int_elementMetadata,
  int_colData, rowRanges, colData, assays, NAMES, elementMetadata, metadata`. `int_metadata$version` = 1.8.0.
  `colData`, `int_elementMetadata` and `elementMetadata` use the **old class name `DataFrame`**; `int_colData` and the
  nested reducedDims/altExps/rowRanges metadata use `DFrame`.
- **Dimensions: 2314 genes × 30 cells.** Gene names live in `rowRanges@partitioning@NAMES`, starting `Tshz1, Nr2f2, Sub1, Ptprf, Cers5`
  (rowRanges is an empty `CompressedGRangesList`). **rowData has no columns.**
- `assays` (`SimpleAssays` → `SimpleList@listData`): **`counts`** is a *dense integer matrix* 2314×30 with dimnames.
  `counts[1:3,1:3] = [[0,0,1],[0,0,0],[0,6,15]]` (row-major listing); colSums of the first 3 cells are 3461, 8730, 10406; total
  179,068; max 2218. The first non-zeros of cell 1 are Gm9846=6, Akap7=2, Ugp2=2, Fnta=2, Dynll2=1.
  **`logcounts`** is a dgCMatrix with 33,212 non-zeros (the same pattern as counts), and **`logcounts == log2(counts + 1)` holds everywhere** (verified).
  For example, cell 1 Gm9846 = 2.807354922057604 = log2(7).
- `colData`: 30 rows, rownames `X1772063061_G11, X1772067064_C10, X1772062113_D09, X1772062111_A12, X1772071040_C12, …`.
  Columns: `tissue` (factor: ca1hippocampus/sscortex), `group` (int), `total.mRNA.mol` (int: 4301, 11097, 13051), `well`, `sex`
  (int, −1/1), `age` (int: 31, 24, 26), `diameter` (double: 7.97, 0.0, 9.71), `level1class` (factor: microglia/oligodendrocytes;
  the first 3 are oligodendrocytes), `level2class` (factor, 9 levels Mgl1…Pvm2; first values Oligo5, Oligo5, Oligo6).
- `reducedDims` (in `int_colData$reducedDims@listData`): `PCA_counts` 30×30, `PCA_logcounts` 30×30 (colnames PC1…),
  `TSNE_counts` 30×2, `TSNE_logcounts` 30×2 (no colnames). Row names are the cell names. `PCA_counts[1,1:2]` =
  -4.2357692400463005, -8.900234959548614. `TSNE_logcounts[1,1:2]` = -40.69132164741808, -36.7019764443467. altExps: none.

### 6.2 `sce/scExample.rda` — SCE with rowData (real data)

- **Source:** <https://github.com/compbiomed/singleCellTK/blob/6723890f56292752cb75397a988cc9721ec7ff71/data/scExample.rda> (MIT).
  It is a subset of 10x pbmc4k: "390 barcodes and top 200 genes … 195 empty droplets, 150 cells, 45 doublets".
  **The object inside is named `sce`.**
- **Format:** `.rda`, `RDX3`, XDR v3, **bzip2**, writer R 4.0.2, UTF-8, 44,473 B. rdata: parse + convert OK.
- **200 genes × 390 barcodes.** One assay, `counts`: a dgCMatrix with 44,372 nnz and dimnames (rownames = Ensembl IDs, **colnames NULL**).
  `@x[0:3]` = 161, 47, 41. Cell 1's first non-zeros are ENSG00000251562=161, ENSG00000205542=47, ENSG00000177954=41. colSums of the first 3 are 1915, 2598, 3347.
- **rowData** (`rowRanges@elementMetadata`): `feature_ID` (ENSG00000251562, ENSG00000205542, …) and `feature_name`
  (MALAT1, TMSB4X, RPS27, B2M, EEF1A1, …).
- colData: rownames `pbmc_4k_GACCAATTCCTAGAAC-1, …`, columns `cell_barcode, column_name, sample` ("pbmc_4k"), `type`
  (table: EmptyDroplet 195, Singlet 150, Doublet 45; first values Singlet, Singlet, Singlet).
- No reducedDims or altExps. Edge case: `int_metadata` has **duplicated names**
  (`version, spike_names, size_factor_names` appear twice).

### 6.3 `sce/simple_sce.rds` — modern SCE as `.rds`, with altExp (real data)

- **Source:** <https://github.com/BiocPy/rds2py/blob/3e797173292992bd843c4bf5240f077b90a42645/tests/data/simple_sce.rds> (MIT).
  Generated by `tests/data/generate_files.R`:
  `sce <- scRNAseq::ReprocessedAllenData("tophat_counts"); saveRDS(sce[1:100, 1:100], "simple_sce.rds")` (Tasic et al. 2016 mouse visual cortex).
- **Format:** `.rds`, XDR v3, gzip, writer R 4.4.0, UTF-8, 86,567 B. rdata: parse + convert OK.
- **100 genes × 100 cells.** `int_metadata` names `version` (1.26.0) and `mainExpName`. One assay, `tophat_counts`: a dense integer
  matrix with **no dimnames**. `[1:3,1:3] = [[234,486,690],[170,0,144],[0,0,0]]`; colSums of the first 3 are 7082, 12414, 5699.
  Rownames from `rowRanges@partitioning@NAMES` start `0610007P14Rik, 0610009B22Rik, 0610009L18Rik`. rowData has no columns.
- colData: 22 columns (`NREADS` 13743900, …; `RALIGN`; … `driver_1_s`, `dissection_s`, `Core.Type`, `Primary.Type`, `Secondary.Type`,
  `Animal.ID`, `passes_qc_checks_s`). Rownames are SRA run IDs: `SRR2140028, SRR2140022, SRR2140055, …`.
- `int_colData`: `reducedDims` is empty. **`altExps$ERCC`** is a `SummarizedExperimentByColumn` wrapping a nested SingleCellExperiment.
  `colPairs` is empty. `int_elementMetadata$rowPairs` is present. `metadata`: `SuppInfo` (a data.frame of 379 rows × 53 columns, including NA
  integers) and `which_qc` (character, 15).

## Appendix A — per-file index (all data files)

Format columns come from the magic bytes and headers (after removing gzip/bzip2/xz/zstd); the last column is what rdata 1.1.0 does with the file. sha256 is of the file as stored.

| path | bytes | sha256 | format | rdata 1.1.0 |
|---|---|---|---|---|
| `rdata/generated/test_altrep_compact_intseq__ascii__version_2.rda` | 3969 | `0a46540acf083875a5053f5c1736dadb735d4a91e8a39201963b367c013cff8a` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq__ascii__version_2.rds` | 3916 | `ee4bd5b8756dfd97e74e09a21f3615d92862b7b07392bbb5def8b8b7045cb377` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq__ascii__version_3.rda` | 162 | `f1c71958268d9c760bd6df10c932f0f4d63deabe94aeaec076de7f5f2ac2beda` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq__ascii__version_3.rds` | 109 | `c42d79bbc6a53b2b3f5f81eb9ff3a0ea59a870d73b0794a85540e88ac4a264ce` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq__binary__version_2.rds` | 4022 | `0431150e647a493cb0a15568ad874f47c80225ccfc2887f678dc00f9a3624d6a` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq__binary__version_3.rds` | 133 | `900bd864c2332c8658c888cc528b1a64d50e7cae139163671c0a2029b903447c` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq__xdr__version_2.rda` | 1842 | `64f6b54a12125d1a6bbfe73010357e89967f2a23c22de8538238695684b4a5d4` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq__xdr__version_2.rds` | 4022 | `dc8c561592d235360b3b427aba77665b9c3ffb5146988f31a2a02386d9581e32` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq__xdr__version_3.rda` | 127 | `56a0b535b79b5f874e524ccdac4f522674e1fa31f42a47bfc9177cb4672f3899` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq__xdr__version_3.rds` | 133 | `be811d54490dc6a91ec7acc48994a35df50be086e8901ad75a40f7d3e9e1a1ca` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq_asymmetric__ascii__version_2.rda` | 115 | `9b198147cad4ccfad50f029bb993babaff5d7e54a2ec0f9cfd9bc06fdca4da4e` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq_asymmetric__ascii__version_2.rds` | 51 | `c514098a1b70e1c727dff9cd145fcc83478a339470197138bb8543f22c425908` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq_asymmetric__ascii__version_3.rda` | 172 | `7d84a2020479132036e36db64ed9bead413c6d5b4dc02757d558165c54834c6c` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq_asymmetric__ascii__version_3.rds` | 108 | `bf471316fb3fd9b58e1be1c169b9568f84b7c456623f3a607b75ec989b0e8983` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq_asymmetric__binary__version_2.rds` | 66 | `e22a17c909130b3d07e50d4a51d28e442c67ef76ac5a156245e2680be68ed0eb` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq_asymmetric__binary__version_3.rds` | 133 | `563fc88449534195f00d9aac4b22e8dc49a8dee01ba38e223332d860c875e5a8` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq_asymmetric__xdr__version_2.rda` | 123 | `90f87dd369c6fcd1ea61970aa194bfa492b358f4193444ee8c10504aca5be2e5` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq_asymmetric__xdr__version_2.rds` | 66 | `884ff709ed301b53be37ea30150b9b39c6354a031e8c7fdd1a112019be0693fe` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq_asymmetric__xdr__version_3.rda` | 141 | `8fc4a615272c30fa7385a831b076ff5e9fe034e72b202eec72922c6c0ca42bdc` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_intseq_asymmetric__xdr__version_3.rds` | 133 | `69e6ea38a3ddbda5137298e744aff69d39eee35a77a320bc2e2b7797f03f2d53` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq__ascii__version_2.rda` | 3970 | `1d71e5fb94f6160e14f9d13a67196c01f6533bcd5ab363cb385ad13a4995cc63` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq__ascii__version_2.rds` | 3916 | `b7c50ad34bb57061406e601a3d944c214a710b757d5e2871b8764c2344aaebe3` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq__ascii__version_3.rda` | 3978 | `b4754ff81ece789a50492b84ac09ebb5d1da30ce387f28c45ee8903bc6d7ecd3` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq__ascii__version_3.rds` | 3924 | `e0ba6ff9f1b36ca5e15df6f9864c2b3de11387aed20be3488c85a6c2d5cb0589` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq__binary__version_2.rds` | 8022 | `653e5711d5924a4b6eb417369e489b47e23ac920806cc957999d377eb8b69790` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq__binary__version_3.rds` | 8031 | `543ee54b2ea2ff90935e2a0990453c5ec72762e58816df8fc01e2d529ef07d51` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq__xdr__version_2.rda` | 2077 | `edf954877ddcd0b185f3bd5b3b1cd1ec90db5cef93b38459101173b478fab7e6` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq__xdr__version_2.rds` | 8022 | `9f86fe431953303425e180658303c2459d29c163a05ac3722bbb49729fe51f41` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq__xdr__version_3.rda` | 2086 | `81cb449c4dc62ca26fb2fbf15cc0e4589cf8d9adfff7363919497d1634c41841` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq__xdr__version_3.rds` | 8031 | `23d727b3ff0421dba40f7e157f9a75daec7e3f0b4bc8defd150304956d8aed07` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq_asymmetric__ascii__version_2.rda` | 116 | `49a98ea21afe0afd635805fbb17f755c380def3fe54847645b2f55333a5d1489` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq_asymmetric__ascii__version_2.rds` | 51 | `23ae7c728ab600242ad522e83dfe720059439955237bc32b75cabd1c06db23c6` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq_asymmetric__ascii__version_3.rda` | 124 | `9c330d2edced8917d549a0077241e280437d27bd04ee5df0d4755ba960d64c56` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq_asymmetric__ascii__version_3.rds` | 59 | `1c65f47f605f75e35e1780e850dc4f62b3da6da069adc6e10474228748b0a04d` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq_asymmetric__binary__version_2.rds` | 110 | `0a1ab53d19749567bedca3b4c1d8463fb1fc6956758886a942a7b456905f9dd5` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq_asymmetric__binary__version_3.rds` | 119 | `2a668d8de2e3fc6adc8fd581dae7b14fec65106559aedd082c570a56d2fac046` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq_asymmetric__xdr__version_2.rda` | 129 | `de9e5d37ba0d6a3a05dfbf8db510170de4474d1e79075ef82a19bd8a01781d5e` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq_asymmetric__xdr__version_2.rds` | 110 | `b435836b032db09f7e3609ffc7731162820affe6a5d801cfa750dc1d41387e26` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq_asymmetric__xdr__version_3.rda` | 138 | `696e0875b0705be6682fe16e8854d954e7edf089a755b06eaeea7e5ee02489c2` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_compact_realseq_asymmetric__xdr__version_3.rds` | 119 | `8f7ae5c1521c7342cf0dc8415fe99c79f6568ee31a14ddd023287331bf646360` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_deferred_string__ascii__version_2.rda` | 208 | `f1ea3cbe168df36558f8265fa523621fc437596cdf731499b917b186e0bd778f` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_deferred_string__ascii__version_2.rds` | 154 | `a0d212013b51e2e7e5bc9435f49df140b1cea41c4cf0c397bc7b91a46ab21790` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_deferred_string__ascii__version_3.rda` | 217 | `6d427ee5be6d894b81d394b92816040d717a15e1da862995f42596d15b4702d8` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_deferred_string__ascii__version_3.rds` | 163 | `a1f72f65809547c02c37b570569ae1378a01d37976730fb7eda77910c7b8ab0e` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_deferred_string__binary__version_2.rds` | 135 | `5a12877eb73382da1275d0d2b138503dd694083db747c8ec32fcd88516d2d488` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_deferred_string__binary__version_3.rds` | 198 | `7e29f54637afbc039b5c49e799d453c281acf59e90c938daa32baaf3867784cf` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_deferred_string__xdr__version_2.rda` | 131 | `5ca2b05e8277efd3e90c389c8211b7f156648299c3555adf11facea036e23274` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_deferred_string__xdr__version_2.rds` | 135 | `d94a09eef71b5e33cf5e4047f934a0272682058bee31d1cc5a9f5cf14c544c76` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_deferred_string__xdr__version_3.rda` | 175 | `ee6fe3105a6fc9e764202066e38b871e3c9f608d454f7cfd3946721dc8a22d11` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_deferred_string__xdr__version_3.rds` | 198 | `0853b3467910e48567f07130815b51984b0659224a964fe46ad5163b8615df0b` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_logical__ascii__version_2.rda` | 76 | `57510c772256970f4f84227ae825fb000a92822318d84a40397d140fcb08fc02` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_logical__ascii__version_2.rds` | 25 | `ac8ae76d3c1c808bc7d19a164917873ce68935d48cddca38906d5de4a1a8f2f1` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_logical__ascii__version_3.rda` | 162 | `b0b493694be57db482d3ce127ed7de74b174d120153caf902ec8a111eee4159d` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_logical__ascii__version_3.rds` | 111 | `76d8e640f758ec607b36ab3443e8ff1a6e897adc57067076b31c5a0cf5aaa5db` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_logical__binary__version_2.rds` | 26 | `ddf24cf85936ed42be8a2176d5ca270529e0770322ac98c56c50240c71e57acf` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_wrap_logical__binary__version_3.rds` | 131 | `2de444f51a1dd5bc4194788e699f155d1050cf6a77e2182929c499b1a910baec` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_wrap_logical__xdr__version_2.rda` | 82 | `17a910061fb09964eee3301251d73967ecd551a04f7aae2ff219ff07bf437280` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_logical__xdr__version_2.rds` | 26 | `2e9c39a5da10b25a9a44cbaa142637dea462570d423a008421050e4aa6136bc5` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_logical__xdr__version_3.rda` | 125 | `48817891c74732b3c5d56d6fe741be002c50f47393f20720fcce0687867d6ae3` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_logical__xdr__version_3.rds` | 131 | `bb0bc5a137e34936abc2e4a21819aa8cb78c98acce7e6286a195981b356d0828` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real__ascii__version_2.rda` | 73 | `82508269edf52c9364c0bb0e71eb5cce15a1c821faa0dd41eaa78f45d2a416a0` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real__ascii__version_2.rds` | 25 | `8b349b3906636673a8100e482aaf4d09dbebacb2025a79796d8ef4060e789ed7` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real__ascii__version_3.rda` | 155 | `82b4b9eae0c3060285215e7fd722c06cbe2760500657ec7b786cc0acf6bc2781` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real__ascii__version_3.rds` | 107 | `eb33a90ccadfb8cde9e0abc8e2c745ef064948797971a5746bafa0ab47517da2` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real__binary__version_2.rds` | 30 | `76c49d3df967cbeef859b2cae3811ed0ac5d0da06026bcfc223ecf443c72164c` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real__binary__version_3.rds` | 132 | `e0923e23c9bd35871f510c41f23ac360b031db0d2e577df1190b91b6a05234de` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real__xdr__version_2.rda` | 82 | `8d5da7a42f4314479b5ea1e32e121e7cb2a96ef2770de0b82e6c9ea04448ec10` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real__xdr__version_2.rds` | 30 | `56ab21274a9c72d969520abce74fc57102fe2c5b2de6480c343b08ccd05d7743` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real__xdr__version_3.rda` | 124 | `c92a64caca3090587c857bfc5730419587f22ff7ccb4ecaaae26187903134146` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real__xdr__version_3.rds` | 132 | `728544874f62cc71e67b758417dbb464a9bd9a4a7c6fdc3bb78877360a8d65b7` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_attributes__ascii__version_2.rda` | 131 | `8c4b5dca27e51be0bc59c2f2b88e79a55b5f9da62cd3053cfa8bee9bf3abe7e5` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_attributes__ascii__version_2.rds` | 72 | `5862f08e9f8c7ddabcab7d59b782d3de4b5d5a50d203f0d138efacfe3a5b0a4a` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_attributes__ascii__version_3.rda` | 208 | `0a84995eb935c040828ae892cfd8f168fb9689f01128faa78bb5ef50e8cb2157` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_attributes__ascii__version_3.rds` | 149 | `efd06eb62aeac6159d6f4d903db32e937790059231604cff1b277e2d28735345` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_attributes__binary__version_2.rds` | 88 | `4e376213b5b908a3128dea381828540171d2c7fe19b65b7144f27436c1f48a92` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_attributes__binary__version_3.rds` | 186 | `3547f812396262941d5855e88969685757a94333e0ef6733758b66735697f14d` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_attributes__xdr__version_2.rda` | 117 | `58553761092bd6eed4170ddee474e79bbbd74ff1f2cf0b82c35954035410d3eb` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_attributes__xdr__version_2.rds` | 88 | `3d9237d6bb1b3bb045f1b6b718e006bc0eef233fa594dfdd3d8ead37d2474abc` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_attributes__xdr__version_3.rda` | 156 | `45e6d13e6d8703e267bfadf65de1174078eb908619ceb29dd4d2d417599058be` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_attributes__xdr__version_3.rds` | 186 | `1df4518478076a710c5fda4b30e1f7ce728691aea2118eab786e17cf05583ade` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_class_attribute__ascii__version_2.rda` | 139 | `3571393000b550b864958f0c860d246730842ec1467c9a3299c4bc7a9eed6060` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_class_attribute__ascii__version_2.rds` | 75 | `eda719086d4842a8df71971dcfbe741a7367ef4705152d6093252da7ca3fa444` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_class_attribute__ascii__version_3.rda` | 216 | `d9318b8037a450907483f821808dec03a68478d2473c62a693090e19c87750ca` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_class_attribute__ascii__version_3.rds` | 152 | `0bc596de6e89a06b01e6870e554c2aee7cf6177aa6472ff621d183b977abc96f` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_class_attribute__binary__version_2.rds` | 91 | `ef241a6bf7970431650a1c79610c626bbf23434e012671b5ee93a0dd4e5cd326` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_class_attribute__binary__version_3.rds` | 189 | `0ac55e46957a9eec6040f718a2f9a971b20d23ec5119aa5fee577c7668152d1f` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_class_attribute__xdr__version_2.rda` | 124 | `1f4a021182c159dc8eecb02739d48a941a20af11e7273f907dd8355221c4fd92` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_class_attribute__xdr__version_2.rds` | 91 | `17e3cad5a9eb35ea64a3efe2c3ad7e70bc9633a8dc21babae732f9f70175ac1e` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_class_attribute__xdr__version_3.rda` | 163 | `db87d73b066abcb300581b567711d3d6088647e986ac51fdb0550e6df24bf031` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_real_class_attribute__xdr__version_3.rds` | 189 | `eacf6a040b73492e7490b2da58d17756e6bcb528f17bc58c4f09e8880f9106b6` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_string__ascii__version_2.rda` | 88 | `fe9f1f355b88f5b612b5574d316b5b8a564ced7d934d79183161099bc4051a38` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_string__ascii__version_2.rds` | 38 | `d70fd0a027ba7ba530b2184e64da7ffdb96c2bdbd25d4410640b19c868dc373b` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_string__ascii__version_3.rda` | 173 | `db7f5fd5b009f64055b76715fafd66d79413404d0f754ef5391a982dbbd64ef1` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_string__ascii__version_3.rds` | 123 | `c04424cac68f7f0c84606bdba9f91bce425b7c1693c0b25b432f11a5459284fc` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_string__binary__version_2.rds` | 35 | `736028fcc51b14327aa4a573389c95ba3acedbdf4a2c7917c9c58b2c7c43aa31` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_wrap_string__binary__version_3.rds` | 139 | `509ae383f640627c423c5e8d43ce02b8e48bca6e8e0440d5cdc1348fd51cdc5d` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_altrep_wrap_string__xdr__version_2.rda` | 88 | `ad8a633efad7be71e4c7c5c8ffea642b679b4e3d00bbe1f70b4ed42b599a759c` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_string__xdr__version_2.rds` | 35 | `05b5dcdc36d7132e9e7d476bd688ea9753f92d282a4137a96d3f199cff39605a` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_string__xdr__version_3.rda` | 129 | `baad1bd52578ee3c205eeeb503276044b0db3d868c1bafc6e31e9f35a258b6f5` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_altrep_wrap_string__xdr__version_3.rds` | 139 | `4b2bd3c016a92f2deab1f808b715c8115870c7a564a91f9e3748e170d731f72c` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_ascii_characters__ascii__version_2.rda` | 199 | `5d83f181e791c62c7566afaf3249ca559c66e24eb6e9a543acb89e073bcaf5fe` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_ascii_characters__ascii__version_2.rds` | 151 | `c799ff532e1f581b68c6a40e69d18f81f66693097f8a4e4853a2ffd1b8ae0f06` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_ascii_characters__ascii__version_3.rda` | 207 | `dead6b82c8c197062d181b3aef0486576cb88a1988b7766c0ccf27dd3ff28a5d` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_ascii_characters__ascii__version_3.rds` | 159 | `7d8677d021b42ca657a23551644385b713c3f07d394f33bb9bc4788cb00232de` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_ascii_characters__binary__version_2.rds` | 132 | `3a3ef5046a50dc6a73ec5c296ea56dc5e288161ccb61bbee1e8d5b55c753a37e` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_ascii_characters__binary__version_3.rds` | 141 | `4d8627b9d7c99e9f3eb173a59f56780d3433d01ce282478b5c38880ff24b6d16` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_ascii_characters__xdr__version_2.rda` | 182 | `fad1474586ca66428967a54fc6120bee9163ba85592650c22725686531b32a26` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_ascii_characters__xdr__version_2.rds` | 132 | `83a75107eaa0ebaf0a298e44f217b02638b81dfa59ecd278ef5df7c3cfaceabb` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_ascii_characters__xdr__version_3.rda` | 191 | `a08f2570c57b8211713f94f477bfeb208829c3b0ed2f3cbcb87e4d3a48ea86b4` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_ascii_characters__xdr__version_3.rds` | 141 | `9ca991b356cc11af17b1809f1eb7ac3eb25df2643e2dd8fbae30522290439ced` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_builtin__ascii__version_2.rda` | 65 | `be2a1b9b3c48f24251d56ec99460d2a391521f9d47c1782ee8d42c485b0b3969` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_builtin__ascii__version_2.rds` | 26 | `172340fa3bdd9a8b0a02711fb7532b7ae16873e81bca7ff9fa5beccf0d56f3dd` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_builtin__ascii__version_3.rda` | 73 | `514e8ae8a0ad07f4e149043e437387064228597c6127fe34513ff2286dff7d68` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_builtin__ascii__version_3.rds` | 34 | `4348153d556e07e8a89b6c55d006f300e450d10427083e9610490768ab3bf1f5` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_builtin__binary__version_2.rds` | 25 | `226e7e9867d8af20036022b4bf880e185aea9ac3f848b2601e8ded2da30d6134` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_builtin__binary__version_3.rds` | 34 | `22479ec2f7782e403b5600fdb74d5bc9ce3189ed155ceebe4b36bd4164c038f3` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_builtin__xdr__version_2.rda` | 74 | `a1d84889bf1679d153e78aef5ce5ea7ba3424e2f8a8635ece86db60875c0436f` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_builtin__xdr__version_2.rds` | 25 | `219a294fd858e891e00a6cf0b2e5a7879c999fa18c197ce6ae0a2c026934e2e0` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_builtin__xdr__version_3.rda` | 82 | `1e3f097087295b906c06f6188a0f7822b243e2b48f6c3c0fc7a6c8808ac370b3` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_builtin__xdr__version_3.rds` | 34 | `d74d790e5928ae11f6dc9a791ee7ca2f7d6f448b34b2bd35a67a48fb749f4fc8` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_complex__ascii__version_2.rda` | 84 | `3d5d4f8eff0d4e106d422010380af088f5b67de13d9f515cec48ef384b43e485` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_complex__ascii__version_2.rds` | 45 | `f6499ddc2e7fc8dc8057ab6097c1264785006d5f5d1eb35a09fac9faf6c89750` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_complex__ascii__version_3.rda` | 92 | `196fc4a4f4dec54a559e2b67f867d701cc131388add278aa3c856876b71ae48e` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_complex__ascii__version_3.rds` | 53 | `a9a4e4a229ca092e5d780eb29bf89834bed80dc4294c7ce6f7e842543ecfe7b7` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_complex__binary__version_2.rds` | 102 | `2da8580fbd46ae65d347309b0e88468219654f647b651c7cc3e8fe56451087ca` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_complex__binary__version_3.rds` | 111 | `af111bfbf3380c8adb3eed532bb3aba6c142419ed8bf79b9b1737e0d3b6297c4` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_complex__xdr__version_2.rda` | 91 | `0c32815d58b022bc7d56dcdcb69fc992928af270bad77a4d55ef75683e869536` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_complex__xdr__version_2.rds` | 102 | `4bf9611b86c5d01b7e41eef8093bcdec92d41a441e55c38e1637603b0011967f` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_complex__xdr__version_3.rda` | 100 | `05a94b079a93c98aeef4e485e05e4a6be08c9971ae597a59a137256ce8587b16` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_complex__xdr__version_3.rds` | 111 | `a7376b25d9188e3c4769565fa992652e130fd84ad4ca917815e8f5890ac9bfa4` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe__ascii__version_2.rda` | 319 | `0046d5ed85969d0b7a1069e42886f2f9af72531d964f45dabdbc526d196c029e` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe__ascii__version_2.rds` | 277 | `08f38b714175fa5de18413691a20e4777b854dd1019ed7c2e1ab1b08862fb0b3` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe__ascii__version_3.rda` | 327 | `3192820465e005c6670de4acf2aa26052804cbb8be6b1c53e833d8e1ebb05ea8` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe__ascii__version_3.rds` | 285 | `af7347428e9ea4f8dff5b6cec5839f36172163ecc446c7caa44821a4b71ec433` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe__binary__version_2.rds` | 291 | `e35dd8300d83853c7452ff2f3ce188b50705075e2f1b630100c20fa75bcce399` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe__binary__version_3.rds` | 300 | `824557ac1073ee5d43f9968b5893709c03b6af145bd96979c2c558777fb50748` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe__xdr__version_2.rda` | 176 | `aa02ebb30936c375508888ed9e58d4af9d8d2a88320a3a8763c8326529f9c390` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe__xdr__version_2.rds` | 291 | `df39d3a0fd990bc99bf95b7161689df0574466e7d0070cd2e36893a8e9c66f29` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe__xdr__version_3.rda` | 187 | `afeaefd448fd496fe951e1bface1f328d801999d0c2b43a3db2ebc7d7587b021` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe__xdr__version_3.rds` | 300 | `058ce55dd25512803e045b8dea028c37ea62c9f451cb121d1865bd7695ad188d` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes__ascii__version_2.rda` | 360 | `5b75f1e6ce0888fadae5ce4ad7a57f1e82aa4c0b852dbd2114e2aa72ac7b3ff4` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes__ascii__version_2.rds` | 312 | `0840d721fd3f3b19580c9fac31c74534f3c6d5ad1f23387803198e7e88777769` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes__ascii__version_3.rda` | 368 | `d000ea7063924b7759b0605d8ef92f0ac6bc1f18c0f1f43e551e1ce1a8310c60` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes__ascii__version_3.rds` | 320 | `2e975675e11ca636bc4a20b303cf6895a03375cf13a5d801353454613c20b22a` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes__binary__version_2.rds` | 371 | `b43f697749646e2c56b0e9b9b1fe8df5bef77e015854cc6fa3ef32215bf52a48` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes__binary__version_3.rds` | 380 | `fd2011249d5700663b396ab53fe68999b08469312110ad0a103cdb6948d2f2b7` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes__xdr__version_2.rda` | 239 | `7ebf90c30541eb7779559a61addbc5a8789e29609be95d0e06fbe3cd87dc8d72` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes__xdr__version_2.rds` | 371 | `3e09ff256e0063d51b066c93b06f205a173354ac560b2bca03c0e2a439824a87` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes__xdr__version_3.rda` | 249 | `41d5fb6fdebf82a5534f8ace54519a26eea58e8ff4437815b2de8f5bb0512b73` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes__xdr__version_3.rds` | 380 | `8067498269129ba9c315ad51a7cbfb76c1056875c5ad7643bd6447ccf12d1a1d` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes_with_na__ascii__version_2.rda` | 387 | `80f7ea5aead382516b4f1bcdac73f578df453aa6b96b0ab7c970eb1f8b660a45` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes_with_na__ascii__version_2.rds` | 331 | `b979a18f2f4364792b7792963e23943ce549a591e93eefcd231ee5778c0f7869` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes_with_na__ascii__version_3.rda` | 395 | `4469197578d4cdcb6e3ee2c37f38cbc48815539f92d4925369404c2ffb01bc93` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes_with_na__ascii__version_3.rds` | 339 | `3b895926dd296a3e976769c5436a5fe794011c4b0f01e37b1c9effe52327fe75` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes_with_na__binary__version_2.rds` | 411 | `7eaf94645fe57ce6844f053f386611cea88298214ac169d7b4de797237d62cfe` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes_with_na__binary__version_3.rds` | 420 | `2027e590f8e533534587ef8811fd9638a7cab030d89d2fd48c4c6c43a15fe8af` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes_with_na__xdr__version_2.rda` | 264 | `13dc4a66f96660cf1c1011a0b0fe733bfc4d67625cc9fb98a57394d4201d230a` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes_with_na__xdr__version_2.rds` | 411 | `7d3ea6b87b75bc8defbbe6901bf9106eb4e35375c026b3612767661657377fe4` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes_with_na__xdr__version_3.rda` | 273 | `10524520acd6c33cd517c03dfaa714585baafbd9d0ab3ffe8554704f8ce9da9c` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_dtypes_with_na__xdr__version_3.rds` | 420 | `74d1661ead6e396c1dbfa8656767d9d540de8e47c791cfc709a15092c3fd7d77` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_float_with_na_nan__ascii__version_2.rda` | 247 | `dd8f0e0ace2ace0814bcb93a685a635e770f417ffd8dff7206dd2611fbe1ea18` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_float_with_na_nan__ascii__version_2.rds` | 188 | `22d82b78ba4f29a391868ce438af056cb0a2c412d11966b43b482e593c00bcf7` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_float_with_na_nan__ascii__version_3.rda` | 255 | `03c4e5e91d2836b8c4d96391246d3d826a808b1422fc0ed6fad5a6113d37d39f` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_float_with_na_nan__ascii__version_3.rds` | 196 | `069361035e7dae2d9b4be0b0b19e283f0ab6fa14570c209355749cf695c513e9` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_float_with_na_nan__binary__version_2.rds` | 220 | `b97a2a0c02d33c502dfcd5b1e773af8eb612e5b732b2ca9993937e1ed2f90aa9` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe_float_with_na_nan__binary__version_3.rds` | 229 | `8f613f8421aaf2a2daad44996c03ee7dbc9a777f4459be1bc32f696905abac47` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe_float_with_na_nan__xdr__version_2.rda` | 175 | `6d101120f31709fa0d608f28438b87e989373f0d9231bee7211bfe80bcab8153` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_float_with_na_nan__xdr__version_2.rds` | 220 | `2dd96438ab662d5319055f17185ff43e10a496f45f28f64a29c60298651ba4d0` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_float_with_na_nan__xdr__version_3.rda` | 184 | `fd7189fa804f5ccd28b22e3c3f2bfbf06be80e3bc97a79fa9b90f226defa8d3e` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_float_with_na_nan__xdr__version_3.rds` | 229 | `d64e5dbfe62bea8ba598cf62055d120ff65d41187ee7bec14b8752b6781ce13f` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_int_rownames__ascii__version_2.rda` | 222 | `8de52976f1d82145f86315f901d36adf94a617b7536aeb4e25fb088594f77e00` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_int_rownames__ascii__version_2.rds` | 168 | `6ffc9656509d4ce83158738fb89ba839d086fc5a8fe5c1cca455916e5fe56c4b` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_int_rownames__ascii__version_3.rda` | 230 | `85ea975fecaa0dbdbb5f93978a7527e75715cd6ae8ab66e43756948fcd8938e3` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_int_rownames__ascii__version_3.rds` | 176 | `d5fb624e5ec7c7f08eb39c628a0319cc5a39346eabfae1b3a345715150501f61` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_int_rownames__binary__version_2.rds` | 191 | `8152f92af9e1550a4f86ed89649a8465b56dd616188ac824e53f0be2d6106d4e` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe_int_rownames__binary__version_3.rds` | 200 | `8eba6c5f3bf54f99c7fb8470810577d2695a104bd9948106cc3780b8d6702f80` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe_int_rownames__xdr__version_2.rda` | 149 | `02045252f9fda2a7cc428a0fd8615da1e34b341c0fd194728554fd1408d26062` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_int_rownames__xdr__version_2.rds` | 191 | `b1aacdd7321c1ede66d47ec655dc49d30b73b1208bacb5ee9414e35a8b857c16` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_int_rownames__xdr__version_3.rda` | 157 | `fe4c413d7ff33c8aac8e6993c45f75d3d5b2074db2a045b8f05a2b8e9556df56` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_int_rownames__xdr__version_3.rds` | 200 | `3645c90bfe08dc058fedbf4560d5aad75ba8bf23ea0f543324f699323ca3cc2d` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_range_rownames__ascii__version_2.rda` | 224 | `e726c8f1ea017e582d0c192c26e36f11f56b83908d3190fdf1adc534263ada3a` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_range_rownames__ascii__version_2.rds` | 168 | `76148b824fbff32e62a024d2b117eea2fd9b7cd7f010ee4d329b4024b6472bed` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_range_rownames__ascii__version_3.rda` | 301 | `a39572c9335a8779b6b00e45abd50143bcb0fa87adad01211a518c8fd9843f2d` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_range_rownames__ascii__version_3.rds` | 245 | `d9ede9841feb49d882368a46bfe55f09b39dcb90feed2ad6aefc0986d9bc8cce` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_range_rownames__binary__version_2.rds` | 191 | `5138e4b16ef4cf5a8d05de7a08146346af634ceb6c44d74042ff825d53caebde` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe_range_rownames__binary__version_3.rds` | 290 | `c073330ca213a1a5828d289c2f10270948887f05b0ec21164ea7c8ab99ecc7f3` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe_range_rownames__xdr__version_2.rda` | 150 | `834b1df9fa007478d13614d335756a5ff170ca790246332f05e4d7531171a0f9` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_range_rownames__xdr__version_2.rds` | 191 | `66374759ec9d3e6c5a699a1da5fffef0c63ab2bf39ee065c09e40cb662961130` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_range_rownames__xdr__version_3.rda` | 196 | `0234f279dace9bf0fdc50795b24382fed0f346ff16cf8c322a05009b09f79505` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_range_rownames__xdr__version_3.rds` | 290 | `5f9399f444db8c6c13e407bb9788dddf31da100687fba4aa2d79185aa80e1a58` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_rownames__ascii__version_2.rda` | 390 | `35b5d0f6ab8d30597e1f19e26fe12b8baa61f2555150c5f2db1a13df7a132266` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_rownames__ascii__version_2.rds` | 339 | `b4b1e91adf511393f95f1a89d5908e7e9acf30029845a9e9005186a7b2bfebb6` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_rownames__ascii__version_3.rda` | 398 | `c35ae9e5f0ca54a8d9e874b54c0d854c5a10e476cf769980fa5def73cc8002ae` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_rownames__ascii__version_3.rds` | 347 | `5617565d0dba1232bf0c8f3a0d5ab6ab87163554db78409e108c77ff2f7fa012` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_rownames__binary__version_2.rds` | 338 | `e61f0f10d85d3df3f7df83cb922d6f6332e6ed78bfbc75318d1062e2ef48f89b` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe_rownames__binary__version_3.rds` | 347 | `9a60931419ead39d30415a836c8d6f6fe33cb35b38ae43011833f8d1ae3b3f90` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_dataframe_rownames__xdr__version_2.rda` | 206 | `aefec81e22fc55c0b6096a4eb0effc7613a45ee403786fed58b95691d3674b23` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_rownames__xdr__version_2.rds` | 338 | `6ed147deec1b6e8f741cbd4fb2aed29cc7470f9bd5b6338a0cc5b4deee0ac5db` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_dataframe_rownames__xdr__version_3.rda` | 216 | `b3888e4d4d95ec0fd180ed6ba72bd145663f3bf3a9f4acc95076b2e4f04d42c1` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_dataframe_rownames__xdr__version_3.rds` | 347 | `1f772877073c038174e224b1c214ef522a5938166fef69aa8563673a40388a68` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_dataframe__ascii__version_2.rda` | 181 | `de1c22e720613622b0b11f7ef71a0358c838d7ff6fc1b2a263c26dcfead7dd77` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_dataframe__ascii__version_2.rds` | 134 | `57259c3d30baceb984c8440f29d5d4b33c79ca60a3b7407f706024cb8f79c13f` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_dataframe__ascii__version_3.rda` | 189 | `bb963cd3162255cc3045fdc164fd1d213c0e773fd1e841ee3f1818cfbd8423c9` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_dataframe__ascii__version_3.rds` | 142 | `d5a07a865e46a3c18c60b4b9b64a9d8021197fec6fe7269d28cc399ec157d2da` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_dataframe__binary__version_2.rds` | 135 | `b3810c9a50d40707ef914257feaeb23a6f17f52c71d7d5b37effcc8d11ddd08d` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_dataframe__binary__version_3.rds` | 144 | `52269cbdba3e147213cc57002fde9a2e4416a602139550b8e5b041c222fb7900` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_dataframe__xdr__version_2.rda` | 117 | `c43955c0d8dac4c7efb27adb4d9750a4d93fba8caad1bf2a39157f0af9631cce` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_dataframe__xdr__version_2.rds` | 135 | `bf4b0a1e97823b3bd7b540ce0161dc74884da2fcc86f232cab50e3c7f33438c2` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_dataframe__xdr__version_3.rda` | 125 | `31571022524d596ca467bf09c43a58a447f2c0c482ea75f338e5da2f2d56216a` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_dataframe__xdr__version_3.rds` | 144 | `3bedb55c2e1580c296a3c757d068d2245c7af864091962ca0022b40945d224d7` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_dataframe_without_names__ascii__version_2.rda` | 168 | `4cc5ac99d3cd9fba210fe9b1384b44cc351094db0e15eb7a97f892f3708fa6ae` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_dataframe_without_names__ascii__version_2.rds` | 107 | `9f41d640a59dc0f840e8fc591491d4e3e5a82da426274b9a7088f0c9dc351542` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_dataframe_without_names__ascii__version_3.rda` | 176 | `428063c7fe6aa83c9b788baecd8f53de54f6a46c875a31fda47118b231198407` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_dataframe_without_names__ascii__version_3.rds` | 115 | `75044662db997380f8f18bfa508f42b9743b4b0789fe05649d6ee085d51ac9f5` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_dataframe_without_names__binary__version_2.rds` | 106 | `b4350a31f5e9e800994646288acbf312cd9513d6ef0aacdfb74630350fa00e16` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_dataframe_without_names__binary__version_3.rds` | 115 | `2c21787929e5f3dba2f2bee3e3e3393ec03229c0da95d062c9f502d1a105e20d` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_dataframe_without_names__xdr__version_2.rda` | 124 | `7dc0b72ee2147267a4a458f9f66283ee4bbfdbf4df3f242358bbb9f6d0656294` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_dataframe_without_names__xdr__version_2.rds` | 106 | `331b36e22785dbec196fc5ac4b602ae2bab8ed6a06b4662c5582b2686bcd2dfe` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_dataframe_without_names__xdr__version_3.rda` | 132 | `31e634be850418dec281aac53be7382c4b5251d77de7a1b92b11a620afcbafe3` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_dataframe_without_names__xdr__version_3.rds` | 115 | `4ca31d32ed2395147b1294e934b154b9d7be249a97bdca63ccc6699fcaabcb82` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_function__ascii__version_2.rda` | 866 | `9e4533bb6dea145e26749f31e81aa9f0c4cdaa042efcdd4de87002ac7f42b4e0` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_function__ascii__version_2.rds` | 816 | `bd4b9902a8e9cc871c07eb3ac51d0079570bc22e184aa11220c804fb4808483b` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_function__ascii__version_3.rda` | 874 | `f3eec7b4f806401828353cb0d6471c683df5e65dd25c74a7e208c48012c132c7` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_function__ascii__version_3.rds` | 824 | `75a4366f64a7b2aac580dfc16803ff28364300a55a7910b6ace33f6d2cd9d8a9` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_function__binary__version_2.rds` | 876 | `64957fc796dad16f039a30e3717893f8e3ddb198747111709ac6dccf6fa3c311` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_function__binary__version_3.rds` | 885 | `8e3edbca859dd4d08e509945c222a85dff144badc491310b7300307e7569b936` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_function__xdr__version_2.rda` | 316 | `a4415f26dd40ee62ff2191cda24f8be945857eea210dbceac51b8cdff4f28a12` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_function__xdr__version_2.rds` | 876 | `bacd65d95960bc0b8721e855621a733a24a77e3cd94b94de91891d629e01d718` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_function__xdr__version_3.rda` | 325 | `4dd2eda34090ea6fffb0bb28844541cae3986d59f88a6ad5d2cd0cc7538838d8` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_function__xdr__version_3.rds` | 885 | `beda09e9d1a66bd5badc8900c4b8b3785483e00fd42f79791ce3960c42eecff4` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_function_uncompiled__ascii__version_2.rda` | 655 | `2c3fb74c9fd18d44738c1a3e0762157877d01044db89c1462d0926fd5c526038` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_function_uncompiled__ascii__version_2.rds` | 595 | `bdba2df9f710a4f1739a16cde8efbbf7a9d6b386d185d9f3b3bc459136b91453` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_function_uncompiled__ascii__version_3.rda` | 663 | `49ab0a596f690508ebf7fae720c426b57e64b8488160c81752d04095b1eacde3` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_function_uncompiled__ascii__version_3.rds` | 603 | `aa8363ae7a19b64e41c9ac2a8a194f37a352624ab669600517e8ef6561724e2f` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_function_uncompiled__binary__version_2.rds` | 613 | `5ba704142797ade25265955ff2d9874c0b0b7249ee3a885316016bb518ebd3ea` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_function_uncompiled__binary__version_3.rds` | 622 | `cbd3e03ea129864a8ee95951723b2e5205021aa01b7f2785d65c76203066be47` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_function_uncompiled__xdr__version_2.rda` | 265 | `4f1dc707593dd7fa5f08b8625db32cec114a23495329eda48872a237f82eebe4` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_function_uncompiled__xdr__version_2.rds` | 613 | `23f9100b5362c8421a70b0767dec643520d03be25bef5479eb9095236bdd6f18` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_function_uncompiled__xdr__version_3.rda` | 274 | `9db2f6049d8d684062c8713354d17e5df7bed375da297becf175c88573247abf` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_function_uncompiled__xdr__version_3.rds` | 622 | `29b76a9e6cc8b57e498d2dd896c52371e1aa8adda6245909c6d4f4cdd2a0034d` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_list__ascii__version_2.rda` | 65 | `a664f96a75c7ba3ea1e10248bd652ee6a2b5e21d473c513894468b5043e8d730` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_list__ascii__version_2.rds` | 23 | `f8788c7ebff3553b70e8ee3714247f2cccf09540c35f40b1a9bdb572d804e785` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_list__ascii__version_3.rda` | 73 | `738c7398f9ae724a669b55ba44e23d5c6121974db4fd7f03c5b47bce564cecaf` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_list__ascii__version_3.rds` | 31 | `5d68375796ca67535465a93f968f342bd68f1b8cf99017ef82dca57f9ff0a77a` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_list__binary__version_2.rds` | 22 | `14624eb2263f32e79ea4092d07e609aac6914f3ea1b86bb31f2caed2072452be` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_list__binary__version_3.rds` | 31 | `4def5215a86e359daa0341b7c89f1239e46416ce732a84ac5cda5b57b7a19bd2` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_list__xdr__version_2.rda` | 72 | `d4500848e96ce642ee284c25dca23d96ee0868bd5d0482acf8b9565e485aa4cc` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_list__xdr__version_2.rds` | 22 | `ce44c051f9d0df29de930c2e3ab547bcbd3903c8dcd505800e2ac0abf4ca4e21` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_list__xdr__version_3.rda` | 81 | `a502eb7ce59bfb324149a89458733c032e73bf55e5bdd5a6f49d8d6e2042534d` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_list__xdr__version_3.rds` | 31 | `0f612445faa9b6a7ee4c77ed90b03bd52c25cc84a6b833e5e3cae8784e218447` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_named_list__ascii__version_2.rda` | 103 | `da845ab59f835ead9b0cd8ee5facc25e63e06e51383cff9792b622193c246c94` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_named_list__ascii__version_2.rds` | 55 | `d0bf9e57468b4185fa80a3928cf60e62be02f4ab6a18b6e0292662253f31f062` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_named_list__ascii__version_3.rda` | 111 | `41e8656e4c7fe609b7304c56a77c4f922a3e32c39dd59bd996a2a08cc1d5a89a` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_named_list__ascii__version_3.rds` | 63 | `b3da26848d045f25c5530312fbc9ef1067590b83bfe71d7f82e36cb4bd989f33` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_named_list__binary__version_2.rds` | 55 | `a304044e2afd7db9b17d7a3149050d059fa59c1e502a43601e4e2fbe74afa48e` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_named_list__binary__version_3.rds` | 64 | `ee8774eb27eda9b6b4f6d27ec1e915415add6032035d28c1db477cd92c035261` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_named_list__xdr__version_2.rda` | 92 | `c5adc063f44db6bdfc45ba1212cc6838bdd29f01556ef3d8d76d1a8ea8e56772` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_named_list__xdr__version_2.rds` | 55 | `fde0f26566cb3c7ad89cc5cb6b5cf2cacf234765e573130472e13915d39df47a` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_named_list__xdr__version_3.rda` | 102 | `3515c026c9ae6532419ff8e9e383fb8a0ea3a319f75bd362243c7f46448a82cc` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_named_list__xdr__version_3.rds` | 64 | `58778c97659e0ce2241523bc783f76d6d0c456769eee89f957390bfef63bf9ca` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_string__ascii__version_2.rda` | 77 | `c56f228befa2cec53b63d7e4cf60af1098099eefd2e8eef7aaa2df7f1a7d6d5b` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_string__ascii__version_2.rds` | 33 | `e3dc82c978707db2f8878771eafb3433c4466ef53abf5f05965e7f6a0cabec3e` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_string__ascii__version_3.rda` | 85 | `ecfb0e02320566593c3a26cd3f6ec7f0e10dd358a44ea1725567447a64961abc` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_string__ascii__version_3.rds` | 41 | `b8f92a3832ef9ef9c6f925114b146e59fc8b1f4437a6ee480efc257526504b64` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_string__binary__version_2.rds` | 30 | `ca0d31dc10452fa1833233c556ae4462246610feb90f5e6bc8030923f9bb3cde` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_string__binary__version_3.rds` | 39 | `717c7b213521ba8e1b47d8df04a7878f378a4494c6aa94e7bc09c9e24b236cf4` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_empty_string__xdr__version_2.rda` | 75 | `8e88383b5a275679986fb3e509de067a9d30fa9d9296245dca1e8ce9f322a527` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_string__xdr__version_2.rds` | 30 | `240f5e7014da5e38c49b713c3d85bd37ba802903e90beac66042cc36cc82e938` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_empty_string__xdr__version_3.rda` | 84 | `08937e3ba82a914f10b6541794430ef91d1546fc6d72fc6093958bfc2213305b` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_empty_string__xdr__version_3.rds` | 39 | `01b61250966afb942898d8fa796de68dfb6b37aca71df28f584fcc1dca8c5788` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_emptyenv__ascii__version_2.rda` | 62 | `75602ff8604d27da23fe48cecc924bbc7223efc6d2e43b605a6a95099e01a104` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_emptyenv__ascii__version_2.rds` | 22 | `3843b2da14e6ac0b644a4f913d7bd84e6f3753c8277de2efecd07b8717571414` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_emptyenv__ascii__version_3.rda` | 70 | `d80a7ee5ca39bbb2335c04957984b054127debd5b58928eb04624dead8c50e2a` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_emptyenv__ascii__version_3.rds` | 30 | `2f33f133256e4dd0557d7cf745e93c74cc615bd3fb75638a5e62d2f7e650fca9` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_emptyenv__binary__version_2.rds` | 18 | `5e20efa2397e876f7d709006b9a943e0917872bfadf83dbe54eef338f6c37bf1` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_emptyenv__binary__version_3.rds` | 27 | `5426adb0284529eb061b975e5d5386c5bbf91b0948aeb8a352c914c9341a0324` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_emptyenv__xdr__version_2.rda` | 69 | `f1180664b77a9b46254ceded1b5449b90d65e6da275be36ffef47cfed71c6b25` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_emptyenv__xdr__version_2.rds` | 18 | `5cc302cc519fe9b8cedd013399eb4e7248ad0da86618b42a8ca4193dfb9fea10` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_emptyenv__xdr__version_3.rda` | 78 | `199522d52638da79dfffc10d42c39ae6f487d27b5cae97fc7a72ec4763b76251` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_emptyenv__xdr__version_3.rds` | 27 | `778df215cfcbf52fcf6c6fa3c66de3c7665b6062fbc0f8f4b253d0cb5f635127` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_bytes__ascii__version_2.rda` | 86 | `d72ae65b255c2b631e6eccfa70c07fd23b226fd0b9f181a48f97bb9e183032ad` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_bytes__ascii__version_2.rds` | 40 | `8f89febf56b60614bda498f6151a111cd613f85cabd1ca0b1f67ab396d6ff430` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_bytes__ascii__version_3.rda` | 94 | `fb80a6e135b8d3a298fe574f9807f8cec0b907e20840d8935dde25c56e812e70` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_bytes__ascii__version_3.rds` | 48 | `86f0c8aa162bad5c2a4ed868ff549837b92bea28ff983541d848f371f97c472c` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_bytes__binary__version_2.rds` | 36 | `77c92ba2c62c6288446737ce7de9a42e4415166a03b8be027981a61b2a61d30b` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_encoding_bytes__binary__version_3.rds` | 45 | `1d8eadf6cbf0fc409f3f68a99026354712940873603689fd9ab7990a1d0e418c` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_encoding_bytes__xdr__version_2.rda` | 87 | `bd4c5cb60db6bd0b790005c181c1be65eedb5d74c302ed2f706fe382e70af6f8` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_bytes__xdr__version_2.rds` | 36 | `ea2864fd3417f5ea7d62447efb69bf8e996e9170fea73cbfc703342f97abbdeb` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_bytes__xdr__version_3.rda` | 96 | `ab9b3e875428263ace3588bd1acdaa1d2bbab08ee4af0a942e4f81f4beb5681a` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_bytes__xdr__version_3.rds` | 45 | `bc5b38de5e674465345846929790ac80828312c6a7c47c67b077ee3bb8d231f4` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_latin1__ascii__version_2.rda` | 90 | `9cc8dc3701aa11e9b81314632c05379351170c64b77b5677a66d94ede6e1a298` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_latin1__ascii__version_2.rds` | 43 | `4305f203f34f46f2d2d6ea9f3cb488ac897812ac68adc92cebcaa7982c9e6139` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_latin1__ascii__version_3.rda` | 98 | `ac15933f34a5df96df97c0043463977f855fa595270f8e8f2d2fe369a3d3b858` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_latin1__ascii__version_3.rds` | 51 | `3885db52726508b982c28a6b0b4ccc58d5875eae2b92ffedbcb6df73e8d4c6b1` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_latin1__binary__version_2.rds` | 35 | `e281db48ffde3857e6207877477922dad10033ca6e4b9d4f3ae839cb1841722a` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_encoding_latin1__binary__version_3.rds` | 44 | `7e570b740e9b02e4806c149739de882243a33eeadb85b7eb1ce155d967dc94a9` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_encoding_latin1__xdr__version_2.rda` | 89 | `5089795613c2397fd2ee3d45ae60a6b0bad6ec2a54c7ea4015ce2d49421ea56b` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_latin1__xdr__version_2.rds` | 35 | `dc4fd1d2dedb2cc192c7268c00777f637b5a6350970f3fd7ebcf25a078ca015d` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_latin1__xdr__version_3.rda` | 98 | `10c2412100dceabb40467ff11b9fa0430e2eb714430c93ce4c18a01d8289d19d` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_latin1__xdr__version_3.rds` | 44 | `d11bf2e27e9c4aae7233a4363916a8708cbebfadd7f20d049c0b1b710ebf8165` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_unknown__ascii__version_2.rda` | 95 | `4424fd2fa2b80b9a2b8d463f03ebc26699f3db421088bb1c7344144e7ca5550e` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_unknown__ascii__version_2.rds` | 47 | `fabd0b45a30ee20cbda0357d9e7a1b96e4b043b4789e6dc6183db6105812843f` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_unknown__ascii__version_3.rda` | 103 | `d15adde5f45df2ab9ad2185f25621de55c76bc4d13218ec6afd91a57c357b1a1` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_unknown__ascii__version_3.rds` | 55 | `84b65cb77e064f05f4175687ea31eaea903e364a7e3eebe4473193aeebc52228` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_unknown__binary__version_2.rds` | 37 | `41dadbcddf762d1d6908ece9c657157fbb9a3dbd8fa54f047e219f6da8bccad5` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_encoding_unknown__binary__version_3.rds` | 46 | `ded6af3429f71af5060eb2926cd424d8655820a2b7469ccbf72b84b6bc271a82` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_encoding_unknown__xdr__version_2.rda` | 91 | `02ba1afd698c8ede3e7bc2be220036c01ddeccc09d7fabbd296fb57a31e89bd2` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_unknown__xdr__version_2.rds` | 37 | `d48a4503b6234b0c535806e84ca59b907e175477422594e0c1b5218b4aa8d210` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_unknown__xdr__version_3.rda` | 100 | `5402ccad2507ccbc5a3be608cad6a6f339f1a4a16f37c77fa886974a6e5abebd` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_unknown__xdr__version_3.rds` | 46 | `3b43608f565ae9263cb235d17665fa99a47bee77efb757bc7af7a5258dfd1a7e` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_utf8__ascii__version_2.rda` | 140 | `3fff9f62c0e90b2c7364056b0b397e138fb91c6c5a62854378b77374d3d76a63` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_utf8__ascii__version_2.rds` | 95 | `2cddaf0274a9fe0bcf87eefbcc4c1fd3d0b5ad98d4faedda2e96f7bab735eb2e` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_utf8__ascii__version_3.rda` | 148 | `2bf493dfd3386a20ba8ef8779242d1a76bf2c3802f462ecde485ff02eda87171` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_utf8__ascii__version_3.rds` | 103 | `255e62a5f584ca327837258b4398631c95285d7119024c847223c3026a84974b` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_utf8__binary__version_2.rds` | 53 | `e36d016517d62c06b905d71b2aa272882ed21cf0971023dd90319eee1e13f614` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_encoding_utf8__binary__version_3.rds` | 62 | `7394b5b06ed51a93ed7594cf38ec308aac004b3c238ee46086fc82cf6624d8de` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_encoding_utf8__xdr__version_2.rda` | 106 | `1c73d22a4dd011fd5356d668859255c3beb9a97fc82ca9f310c84007475dbb5a` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_utf8__xdr__version_2.rds` | 53 | `e04f7e67fe488c7102b7ee8bf818c4462e0a6729225915388755060a4da98b08` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_encoding_utf8__xdr__version_3.rda` | 115 | `bc08756b6b8d6e6e184df1b37f4b5aa1803e20c634cd9e4f375138b56ca9b62e` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_encoding_utf8__xdr__version_3.rds` | 62 | `5eba7291b7c2f6aa6ba1493e00b16eb60fedbb9ac9d36905c8e98bd50efac7b0` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_environment_global_argument__ascii__version_2.rda` | 257 | `2bbb072ab7cad358be07315ff9ce8d2884e03efa8c1d4bf06c70293dc046707c` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_environment_global_argument__ascii__version_2.rds` | 198 | `0b9141ac42b89dae071edda310cf37f89d98af1e3b3d2512296fc9480058194e` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_environment_global_argument__ascii__version_3.rda` | 265 | `b1961babaeba43c2a84adcdf074eff199104c476877d3896b15c4124e6bd629b` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_environment_global_argument__ascii__version_3.rds` | 206 | `5419f54261597d4f85d978c9460c90b5becea5fc7739f828197cb8e21ba9c7b2` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_environment_global_argument__binary__version_2.rds` | 200 | `55ff6ae6d2668f82bb5bbed44bbcafde6851892910aa47af6265b90f16e7e02a` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_environment_global_argument__binary__version_3.rds` | 209 | `a0d1e0ab36aa6e7bef55163052980df112b1b89bd8abbe1760026c8127e9ab68` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_environment_global_argument__xdr__version_2.rda` | 117 | `6dbb6ea49af9517f3f5e285edd7d934b4510f8e3f135b611d6565aa448f52b32` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_environment_global_argument__xdr__version_2.rds` | 200 | `b2e8645266cee50982b61577a9a250f00d393dfe1ccfb07d1bef20d0acf1ff36` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_environment_global_argument__xdr__version_3.rda` | 127 | `b5db2324f93c4319df6a673fc841fbe19822dcefd93814c96c268465348d4112` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_environment_global_argument__xdr__version_3.rds` | 209 | `f2a75da97a087e977465120be6622b81b904d70eb28aa915e5d8fb04ffcf6fbb` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_environment_global_default__ascii__version_2.rda` | 256 | `1b92e08ba064c7206d2067f4a621af0b3be7ba4aa4f05f55d446e609b4c67c93` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_environment_global_default__ascii__version_2.rds` | 198 | `0b9141ac42b89dae071edda310cf37f89d98af1e3b3d2512296fc9480058194e` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_environment_global_default__ascii__version_3.rda` | 264 | `9a802110ef4d6271ff863d7f85a5ec048cd5dfc19c03f8e52c59489275f44571` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_environment_global_default__ascii__version_3.rds` | 206 | `5419f54261597d4f85d978c9460c90b5becea5fc7739f828197cb8e21ba9c7b2` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_environment_global_default__binary__version_2.rds` | 200 | `55ff6ae6d2668f82bb5bbed44bbcafde6851892910aa47af6265b90f16e7e02a` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_environment_global_default__binary__version_3.rds` | 209 | `a0d1e0ab36aa6e7bef55163052980df112b1b89bd8abbe1760026c8127e9ab68` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_environment_global_default__xdr__version_2.rda` | 118 | `ae9517aa0e85ea7ee468cbdf396f1e6d39e55e3524b36c3b1217e2409d818e96` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_environment_global_default__xdr__version_2.rds` | 200 | `b2e8645266cee50982b61577a9a250f00d393dfe1ccfb07d1bef20d0acf1ff36` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_environment_global_default__xdr__version_3.rda` | 129 | `308b672e72f29f3f370e8d5218c71fbfeb14cd15b673cb0d2ee360f88dd35810` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_environment_global_default__xdr__version_3.rds` | 209 | `f2a75da97a087e977465120be6622b81b904d70eb28aa915e5d8fb04ffcf6fbb` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_expression__ascii__version_2.rda` | 124 | `f31ece8bfa2ad7546400a1e94b77d7ca71773766da34d244725d0735fa8d8b90` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_expression__ascii__version_2.rds` | 82 | `417bb7a1d9b8541c4f0ad43af8da5030026c7b97f067691d5e2a5d3e9a8fb6af` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_expression__ascii__version_3.rda` | 132 | `3b03a3a55f98d9f3c8fb46de3b1804bf316442b4a291fae967211136042bc304` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_expression__ascii__version_3.rds` | 90 | `f2235465f3974c9dfd3d499caa9657073ce1a31b60d68e5a8600387db85ad849` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_expression__binary__version_2.rds` | 87 | `3d28946bae6210ee957e5493ff2402610eb93e69458c8e942ba89109b7b32737` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_expression__binary__version_3.rds` | 96 | `503745d16c30dcd6f2f4d05ff6be10582ef95c5db04d5c39d360f5490f20127f` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_expression__xdr__version_2.rda` | 101 | `2d9c17af65397180f15307261771b884e9f53ceb41aa3ad91a5e833e4c017608` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_expression__xdr__version_2.rds` | 87 | `bc254ea4127371c3703613d6d65923b6f86d6cabc02dcb74802f1f4e2df0fbc2` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_expression__xdr__version_3.rda` | 110 | `0f015780da322b3dda3427110d8b90abeac1838863e86c1ca5b91a37fa69a46b` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_expression__xdr__version_3.rds` | 96 | `39b9c0e9f9f9a647f45707034f0a8d98966c5d15eb20c25164e99a7e35361dfd` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_factor__ascii__version_2.rda` | 165 | `a083ae45ba308040b3dd031bc53473257486a92740ef86aafbd076b6ecf57495` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_factor__ascii__version_2.rds` | 127 | `f193f1095fb02b271dd67041d99d6941df1421efbe2331b41a564f169d94e3c8` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_factor__ascii__version_3.rda` | 173 | `5592f845356d895e3ef3dcd664151f2adef6618fc676f8c136806645817437d8` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_factor__ascii__version_3.rds` | 135 | `ee4679e267e20f5094447a47860c30f397a5c556bbdbbc37a1d0d15382e55d02` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_factor__binary__version_2.rds` | 129 | `77015842e776ee758389ff4c9eb839524b22035b0c5f5140377431d442406685` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_factor__binary__version_3.rds` | 138 | `9c7d575ffc1fc20daf3a0f878a731bcf71ad25f6996eb97f6e8b4a8bcb619a0b` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_factor__xdr__version_2.rda` | 112 | `54d4aec8482f501ad628f9244ab26017142c6f451c708c185fdb3ed9e1a74f49` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_factor__xdr__version_2.rds` | 129 | `98c7300fecc7f932f9dc6222984480a13f7e4a3e62218a8ff0ad382a6d338bb9` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_factor__xdr__version_3.rda` | 120 | `45e4215d4174b87d6b262107880190116210850b92095dbdd1eece9792a87bae` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_factor__xdr__version_3.rds` | 138 | `ba386c023a12ce023c84336631e58fae27f611d1cd35de1bf3d3db9332c9a3f1` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_file__ascii__version_2.rda` | 181 | `6a1236af1a14914c813d594aa53e82f30e153169eced4575c8b0d34e26b0d8b3` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_file__ascii__version_2.rds` | 146 | `e2dc8a5fcd1fbf734543f3d10947eafccdb2b6f4dde5bca0d19556161b5979c1` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_file__ascii__version_3.rda` | 189 | `7a56071b622c6aa9bbfafcfc79abeecb79c1a864dcfcb14131c63c8e554b35a3` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_file__ascii__version_3.rds` | 154 | `a42278ed617e8a4461f2b2dd616f66df961ed0ca181f7c47d131902f5a143a80` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_file__binary__version_2.rds` | 142 | `e4c20ff35648fe33750b0bdb77b6cb96f54ef9284c1b3e9460514387004e09c6` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_file__binary__version_3.rds` | 151 | `50a9405c111c44c69d5f4c5ca9f5ef361defd959a8ec68b06082b602b5ec973c` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_file__xdr__version_2.rda` | 118 | `b29538f1058720eea01f6ab5c19b205ed640a4dc58d0aaabba165a5b66e7e878` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_file__xdr__version_2.rds` | 142 | `71be127e1abb0cf5d7233ca8f601701563f9133cc16f4757b27eb23e90dc0519` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_file__xdr__version_3.rda` | 127 | `8d6bb54b77a2d151b34d4d1a353ff3936ee800a8d5d66dfd66d62413104530e1` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_file__xdr__version_3.rds` | 151 | `7ea33a9c5a81bf51502b5b5dbb3ea6037842dc9ed294c76295aeabf70e28cfd8` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_full_named_matrix__ascii__version_2.rda` | 306 | `44aa415bfa4ef7a222b022608c4d45347962782985f769da5ec6785452309cd8` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_full_named_matrix__ascii__version_2.rds` | 257 | `f044427b3174243bbb1d4c646dedc7f22a9c2b3d51530a2d13e485fa4cdf6310` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_full_named_matrix__ascii__version_3.rda` | 314 | `1cc64ecf07986cae5ecb259b16349331efd8d65407b1e279496f86315b9b5d84` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_full_named_matrix__ascii__version_3.rds` | 265 | `6ab01f12a92be5b379589200791320568d1792e85e4194c6327da15dcb92abdc` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_full_named_matrix__binary__version_2.rds` | 268 | `d687a67c2baf33583d96ea8270f521c455283fc8962146d398140f75d9cb6869` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_full_named_matrix__binary__version_3.rds` | 277 | `82aa6d816a34325077915e5a1a5b81ff94d5a04251e272168f16c93f40784670` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_full_named_matrix__xdr__version_2.rda` | 169 | `11a3272fbf7ef212b20e9fcea227fc889af42ef7f1154b6b92d3082eb305e984` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_full_named_matrix__xdr__version_2.rds` | 268 | `7631cc5b6c279ea85c38fa9fd893bf676ea82c31443fe2269998837f881c3974` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_full_named_matrix__xdr__version_3.rda` | 178 | `4405056c6a6e5dcd23f0382030c73a847c12f7a93b3e51796b5f122ee808e7ac` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_full_named_matrix__xdr__version_3.rds` | 277 | `d6c648eb78b94e25c309ab567dd4e8f34a684ec9f1bf70f664d0cd4b03a34a8d` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_function__ascii__version_2.rda` | 1079 | `f81c0d38f486f185a314940db18f360444ef4a345da8d7c28a213dabd7976366` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_function__ascii__version_2.rds` | 1034 | `2663f10ec441fbfb1ffae51c55d3c584ef4442a6d2688e2d9c07251931fc19bb` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_function__ascii__version_3.rda` | 1087 | `7594afd0a7195149ba4682482c275233faafeddaae638ce5ae54e5c45688a057` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_function__ascii__version_3.rds` | 1042 | `84d898210a93e139f286144bd5d4b71ea82c5d24c5ee8d61c9db404489d05585` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_function__binary__version_2.rds` | 1149 | `01f2454b2041da84a6e653ddff7f2f044bd7d87de5c4959791c62b74abc77d43` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_function__binary__version_3.rds` | 1158 | `4f806b6dfb6d988b4cdc46f224dca98d4c54ab36cb3fd98dbe6008b76f47c8d3` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_function__xdr__version_2.rda` | 375 | `87b7c9b202330d9b3b656c712e6e2e57287e735d989956cb8f4879a7289a4504` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_function__xdr__version_2.rds` | 1149 | `b2f90764415a6c98bbbefa6535d49c9739fdc29a52f8772f70fd02f598ff6f88` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_function__xdr__version_3.rda` | 384 | `3b6b297e2102e6788b743403e40a65606d95c7c4218248d6c3a5077f42d76bbd` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_function__xdr__version_3.rds` | 1158 | `d937e0512323f3f2d92a3a9c64a37d05a55e94851628850073dc9ee53dc56a7e` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_function_arg__ascii__version_2.rda` | 1300 | `98b801d7b04cbe6e91d5806609ae939a4b99858c0070f57aa026cc014fb2cc69` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_function_arg__ascii__version_2.rds` | 1250 | `bd3ff9f718c2600ab727442f5016363fa2e758e4cb686bad7dff485126f981cc` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_function_arg__ascii__version_3.rda` | 1308 | `e07957d9e53c3a116da624a3edc694b578b688565b225925967a2b150d737ee2` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_function_arg__ascii__version_3.rds` | 1258 | `cb39480a1da06d4f5376b62f979e14be6dcd304ea18b0e13a212c5cc2f83ed24` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_function_arg__binary__version_2.rds` | 1405 | `4043c55a84dc28a8d986729f1f0ee03f5f9f920bc317f2e4221bc5ba70f7d5de` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_function_arg__binary__version_3.rds` | 1414 | `e8dedffa336074d942eaacde4923565f92bdcd3186570c9b842a85c378f7898c` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_function_arg__xdr__version_2.rda` | 401 | `aba0f30b4c12e69a27603d095163cb4459769a5651695d84abb6cbc9c41745ec` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_function_arg__xdr__version_2.rds` | 1405 | `16409419064fc40703692d3daec705b3190f6ee5627af6532d1e7b42ec2ca619` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_function_arg__xdr__version_3.rda` | 409 | `4c584cb00fc5a0cd232d963edb46aeb016ad70311b2bf7bb832142ccb6380fad` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_function_arg__xdr__version_3.rds` | 1414 | `dde2eb78bb5938cc28b894f2f5779d2bef6fda6d38e3d9d6e4117d86617f6c5d` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_half_named_matrix__ascii__version_2.rda` | 189 | `3ee99ff1f37bb05f83936dbe50fbf194adf6721d2a0a45c6d777e5e1578e744b` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_half_named_matrix__ascii__version_2.rds` | 140 | `1aa194cf390e27b1afa40197bb4c46145ef60d7cddef818f9fb66aafd30d9464` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_half_named_matrix__ascii__version_3.rda` | 197 | `d76a0482f86f8ee3587d8c35c6425331a52955bb0d56ed3b7deea1166fc438c7` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_half_named_matrix__ascii__version_3.rds` | 148 | `e82484d43a6dacbcaa56580795a37d8c82ca2702d39748b9a35cf2143b0ff3fe` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_half_named_matrix__binary__version_2.rds` | 157 | `0c2358c0dc97ae6f4f6500ae5d6ea98604cd9a5dbc0b34517d724c47099daeaf` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_half_named_matrix__binary__version_3.rds` | 166 | `e77041af371408a9d0445f0029ce6b40db701be3c57281edf5c4ff20fc0e3ed3` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_half_named_matrix__xdr__version_2.rda` | 137 | `16b3008c408666995c2976563f11c6aed255dc48a1543fd6d75c6c06fec70956` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_half_named_matrix__xdr__version_2.rds` | 157 | `54f47e2fd6fb8fb28c4d6373fba723f4ab48a2f58d5be1f2b908512d3b0bf09b` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_half_named_matrix__xdr__version_3.rda` | 146 | `218148298eb1d0a765a6f7f4c436f6f3c48dd85c4b1e63dbff1eea90a769d08f` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_half_named_matrix__xdr__version_3.rds` | 166 | `025cf33ec7a191e1722d6dd3556169b9e0832232ead3bed6a6882df05adafb59` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_list__ascii__version_2.rda` | 129 | `8c9c2b4cf68e639642199416e467d58db57d38e1ee4e1aa19d85940856c556ab` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_list__ascii__version_2.rds` | 94 | `d35ebeab01c9b914c50c474705fa50d30481df454321c22f304cd5077a8794ff` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_list__ascii__version_3.rda` | 137 | `d2567c30b2314efa2bd832edfe473300a61ee84d778585ed7389c113af4461f2` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_list__ascii__version_3.rds` | 102 | `0009f895b8322ee2f0658b76fdb55343796cd8f31f4eed56d3c1cc77328db54a` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_list__binary__version_2.rds` | 115 | `5c141a9aaf8580ab2f55933a553f8ce4147357d4456ce6497fbf7808329824ad` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_list__binary__version_3.rds` | 124 | `781185f598778ab8d98dc6e191c2d830fd8f9888e34d574232ef5b8a8621e7fc` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_list__xdr__version_2.rda` | 104 | `4f00fb912fd2b9e3deeff441e0061f427abf8e05553c764e04763fc1b9f6e93f` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_list__xdr__version_2.rds` | 115 | `e1f37ae6d6d1a5eebcf42bd1ec0234364983fbe2a6d872e6907b581bd44ad9fd` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_list__xdr__version_3.rda` | 114 | `d51fca28d7bd37d15e6eeac45c53dec043e35cbc8b1b1243c6ccc9cb36b93694` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_list__xdr__version_3.rds` | 124 | `49ab90f0e79147dcf22cb71677637ef5835776f4f20013475ddd9cefb5b72d05` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_list_attrs__ascii__version_2.rda` | 146 | `019cb298052df0e284cbbb9cf74f3c4a7be1152b3912406dfd1dce254bf30db5` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_list_attrs__ascii__version_2.rds` | 104 | `d958e2a2f54cd88a0c0599fd47e8457e45b0590bc63ab29f2b323aacd6c6a1d5` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_list_attrs__ascii__version_3.rda` | 154 | `3e29835aaa1bba91c56d14317613614b5c814e1679d8aa150afe014b41e697e3` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_list_attrs__ascii__version_3.rds` | 112 | `5809ec4e277a8d88809ca69f82344acc2ad7645631d9a7edfe858f64f51dc812` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_list_attrs__binary__version_2.rds` | 111 | `f4acae3a914d2c00a498c47a17ed749615348ca7250da397d54f8b3db3522a80` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_list_attrs__binary__version_3.rds` | 120 | `128cd1adb918c26977392a8bceae479c5e4b560a53e6bf6c0dfe54b4a1ab6a0f` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_list_attrs__xdr__version_2.rda` | 109 | `68acf6687903ee4aff397586699e211a5834e841b7b144079b1c78aa70a67634` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_list_attrs__xdr__version_2.rds` | 111 | `114b28c21a69763a950c1fa89f4b1f213337395967b070c9d63775efb8b57e54` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_list_attrs__xdr__version_3.rda` | 119 | `a6c29f5d68ff8b50f67ba94e97b44de6a0c9edcfff1b2c570c4c834c259c8b48` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_list_attrs__xdr__version_3.rds` | 120 | `5b1935ce31161ee5f02ad743f263e4e4018cb3cc96b356fa4660ea8d0703afab` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_logical__ascii__version_2.rda` | 72 | `90852da9ad6070b76bb9a24c5dd08ca4569bf03032a4c0f24b0a62319aac30c9` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_logical__ascii__version_2.rds` | 33 | `41119cc1d794be28171a5acc042ac2d1e92a42491d06d4ffc87e7325c5476ec0` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_logical__ascii__version_3.rda` | 80 | `7d5b7be919e62855b78b5999becc7b9172fb3c5991f5bc926bd009aba56773da` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_logical__ascii__version_3.rds` | 41 | `baa3d9eaf5adf875ba7db2d05edd86edd2ae0b813ba375f41f6c8e7bc1bf969e` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_logical__binary__version_2.rds` | 42 | `ddc8556f76711febb6f81656b50f4586e4ebdd047dc16e8c8863a24c470af679` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_logical__binary__version_3.rds` | 51 | `8bfb314e431e76c5975ca03cceb7295ddcc55d6ad5d26a7da1233202387395a3` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_logical__xdr__version_2.rda` | 75 | `cd7c643c8296035ae40658773e6d21aa898ec8c39d81ded04f22a80e1971a6ed` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_logical__xdr__version_2.rds` | 42 | `5e22ff27b87c1babd6f65b981033f8eaed1bd94285d909ed552069b1a24a7fb2` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_logical__xdr__version_3.rda` | 84 | `85ec39b0105591a15cc1801ff6279b132f16bfbf86be74e4acf71646e649740d` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_logical__xdr__version_3.rds` | 51 | `43969e56b88182af36e49ee810655040e3a6ee625a5f3f3dea0d815905fc6f0f` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_matrix__ascii__version_2.rda` | 107 | `338921057b56df9b7e4211e0f688dbd4ff34951dcad7c1f3ab973d7a4cce4665` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_matrix__ascii__version_2.rds` | 69 | `4795ed07eb1e9de26daf48d6c1763254af9191e45b00628ffb539226a3127489` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_matrix__ascii__version_3.rda` | 115 | `f90cd7c80916f65b3109d7c8056126fb69b456b5ccacdbf6d8bd80a9c8895098` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_matrix__ascii__version_3.rds` | 77 | `159ac0d6e6f5700735e6637faa149fce1f161512c2ba81fdd6f851bdf9c0b5be` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_matrix__binary__version_2.rds` | 85 | `3b29fd266b310be6a3b495a1978631fa5b669149d9be5b14667cdbaf29ef683f` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_matrix__binary__version_3.rds` | 94 | `d55b1572689d9fd0673813c773d36db793ad54b86c6f5102fcd186a9a6965f87` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_matrix__xdr__version_2.rda` | 97 | `48f13829102e7e0f3db9ff9384ef385c29e1e03afa1932842de7b89e7c90ea0a` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_matrix__xdr__version_2.rds` | 85 | `72254ca0df053341c3bb74a37f4de608573dbd44086e60377996d1d44de29ccb` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_matrix__xdr__version_3.rda` | 106 | `284c618d86e883fce7fa77553941b2cc4072f18775a049708ecb2b544c01b0ee` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_matrix__xdr__version_3.rds` | 94 | `255cff6320744aa837b814e5640db0e220cea4c1ffa796235ad1aa1332c1447b` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_minimal_function__ascii__version_2.rda` | 631 | `ae5707036fb08a32a5ef8d555fc1649c97f012acb7924711c13d1b1469733085` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_minimal_function__ascii__version_2.rds` | 582 | `3ba13991d9188957a5a7a55deb944cb5575e10c482beb258d34bf07c31144cdc` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_minimal_function__ascii__version_3.rda` | 639 | `d4331ca3eae76fd7856a1e737062b3ca62d86c0f508c5ba906f90f6b9f600df9` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_minimal_function__ascii__version_3.rds` | 590 | `81b0852462e56689c0ed0225c68a01c6a6002fea1d469d1cdcb9954567d91faa` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_minimal_function__binary__version_2.rds` | 616 | `2d824ac03b1e17142e03064415ea1089cab1736966c2f2e9ebb1765bd8558111` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_minimal_function__binary__version_3.rds` | 625 | `df9b2e09117b22a75954a651a2623a86b0d26b5c2d51f32c4f26e97bf1d5377b` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_minimal_function__xdr__version_2.rda` | 268 | `3c5c036ad7f304b62352b286189f7f59f90f5d78ebef6924f267391e2be33a7e` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_minimal_function__xdr__version_2.rds` | 616 | `79da3407f49e41f6e80d979e2e53356658c0a882c7ffaa3d285f69a0569177f4` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_minimal_function__xdr__version_3.rda` | 277 | `4ec519bda52384dd8f7778cbf55554c2cf25deee334f58d1c6b79d1acd21e642` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_minimal_function__xdr__version_3.rds` | 625 | `82e74e6fd99e624f61dd86fb1c2d625411e2d53f6446822e5f02e19c288b07f4` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_minimal_function_uncompiled__ascii__version_2.rda` | 432 | `275163ca4277c6dbc59d98d7d83b3ed7f7e976c8df4a8defe9f2b606ec9e2eba` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_minimal_function_uncompiled__ascii__version_2.rds` | 373 | `e8b49d499a2e400cc9425b9f6ce49241b15649f8d28e11ec461fd7a9531d6f00` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_minimal_function_uncompiled__ascii__version_3.rda` | 440 | `a64f13b41d94e846df9e666a4c6ca5af3d31b9528e5e63645eda5e532bbdc4ca` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_minimal_function_uncompiled__ascii__version_3.rds` | 381 | `40929d40f2526a8cb8008ef4beba0c586c80461041f0ebd660d41b7e22017e40` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_minimal_function_uncompiled__binary__version_2.rds` | 369 | `a9e40b1d28db5bbe9344eef48a08a80e6d80b0e94db4d217ab7719be16a3af0f` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_minimal_function_uncompiled__binary__version_3.rds` | 378 | `9dbad235563e65334e7b159d9243008652600375ffb538a7109db06b1be222c5` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_minimal_function_uncompiled__xdr__version_2.rda` | 212 | `cefa844714db94eeb5e4d3979ace0fe8af4456db9dd2a0b8390fa69c3af55393` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_minimal_function_uncompiled__xdr__version_2.rds` | 369 | `9b5926d4b3d253dff69f75a408c976fea0ed99c5436f85fe49b1e02fb713f958` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_minimal_function_uncompiled__xdr__version_3.rda` | 220 | `c32395ba536642620c618b2f3b20ffbc3e4c1bb7eae908a8d504f2d38b66583a` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_minimal_function_uncompiled__xdr__version_3.rds` | 378 | `5ff1659afeabcac694145b5f39fbda310cb08d55b75251cfde61f06eca082bd1` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_na_string__ascii__version_2.rda` | 69 | `3d590d811d5dea5f6a9294f04c7db812f7664526bb7e17b75fdac0141e84f669` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_na_string__ascii__version_2.rds` | 28 | `516729685033a804c0fef5c65392c0bec90dc7c9ac6b0e1f7082b905573e01df` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_na_string__ascii__version_3.rda` | 77 | `913248e1f11592c8dddc3ce287dc157e2ff01c6aea46043f243e9f43a12eec39` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_na_string__ascii__version_3.rds` | 36 | `cf6412d9459631d37412233164ff90c23ff898f2965432b4cb62335f89193571` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_na_string__binary__version_2.rds` | 30 | `3dc3f6344ea49ccaf6a7326c091925a6cc51cdd15df2f0ca1ac134f90bb87591` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_na_string__binary__version_3.rds` | 39 | `001de737c0e9a624b858a984b2d5709656d6e62fe8f75120ae069534d94ac9b7` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_na_string__xdr__version_2.rda` | 78 | `9bf41d9a0bd9430b8d4f449f148951f748b0f50fa2861fba94e9cb254433baa5` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_na_string__xdr__version_2.rds` | 30 | `563c986109f2ed49cb088d6f9d4d93d10a77e2dd6e87176813167d03bb68dd51` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_na_string__xdr__version_3.rda` | 87 | `1b13d22964328eb5e7642e89512257a1d4d89b52c543bdb59bafc7c074e451a1` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_na_string__xdr__version_3.rds` | 39 | `fcd055ef281bdc803c0fba12a1c43f0fe84a8f40733513aed3339de9bef3f227` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_named_matrix__ascii__version_2.rda` | 233 | `52040f76ed28911602a07f28d914d201028ccd79c6f7c1ad7f506684eb88e35d` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_named_matrix__ascii__version_2.rds` | 189 | `0a71a90fa7de12b5aa4933252a37b0e0c6323f1a12675da88808a26d0399aacc` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_named_matrix__ascii__version_3.rda` | 241 | `2ef0a659d8a678656383d760f5bb161e2fa4b601b6c494a1d29f6579f425f4c1` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_named_matrix__ascii__version_3.rds` | 197 | `2444898a45267e5e990311c192d3f8ae11a327e8a30412ef7ba0eb507a71c677` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_named_matrix__binary__version_2.rds` | 203 | `0fe33945b75b4a2b0eea7c507cd2672b25823aedca22429d378679af50182c5e` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_named_matrix__binary__version_3.rds` | 212 | `7ad7082461a3084635c362b9922c3fd86c60f8bbfd21e1ac99658c563cd9f30d` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_named_matrix__xdr__version_2.rda` | 146 | `90d6822f4d1f387b49cc2f9fc48a9ac94e2266a79b9c226561306eeb08a5af89` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_named_matrix__xdr__version_2.rds` | 203 | `f5811f8427c462f5009b1f37addc47b904974b3da8d14d0f1a6c9117f318e874` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_named_matrix__xdr__version_3.rda` | 155 | `11d7afa502378f24d25e3d720c77134248ee573118924c429514f69c79ea0e47` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_named_matrix__xdr__version_3.rds` | 212 | `ba0509ceead5eb0e778dec3fe503f78219b4c267db183d545ee04f8e5237bf0a` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_named_vector__ascii__version_2.rda` | 138 | `eae9e844a7b28b7700815deb9fb468d2b1c188871c2a3c823e6cb549b1eeb635` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_named_vector__ascii__version_2.rds` | 94 | `5a4277a006e2177093f1371892d7b7595f49118ca0f84c2b892d85d56e3c4642` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_named_vector__ascii__version_3.rda` | 146 | `03d92249075e799cd8661da2a601753f6349719b05ed32e281b5abc39c4dbca8` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_named_vector__ascii__version_3.rds` | 102 | `35281bec75c5f0abaad510b50b68c9d8c22999298d1733df960d6f3d8e252cd4` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_named_vector__binary__version_2.rds` | 106 | `b141af6e4d6c3cbea4a7e12caf9ace8211f27f88a0337940a8d8831fee870ff3` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_named_vector__binary__version_3.rds` | 115 | `638c99e54ac3ff928d70c37c1ae3e7ce17862e7af91f73323e08725f83db2caf` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_named_vector__xdr__version_2.rda` | 109 | `8cb072fec879631954829ed19b29f40a53bba76fa2a30a29806c4a0eb545ead6` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_named_vector__xdr__version_2.rds` | 106 | `7d76109ffac0c7c47f54a0ebd25b0d149c78e015a8020dc239f13151af04dad0` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_named_vector__xdr__version_3.rda` | 119 | `c7d1fb3065cbbbd9eb1be32ef5e3ed5ebfae1d5c880dd905d8b29f2693c2b724` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_named_vector__xdr__version_3.rds` | 115 | `7fb6e7d9d4ac177b5a53bda77dc3f7e2fdedc6d9456333b851ba3da9d1f9cf70` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_namespace__ascii__version_2.rda` | 97 | `5b2fe064b152d00b42381a572a80ddd8e7771ee6256fc6d0e677802a03d977f3` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_namespace__ascii__version_2.rds` | 56 | `6c0497f7ddeee5adcf2af788ec85407cc692e31c417f43f582e644f08c489aea` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_namespace__ascii__version_3.rda` | 105 | `43af45a9639d35b16a236d278fd22335ca092e664c0bd292a2814061afc557ac` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_namespace__ascii__version_3.rds` | 64 | `9b3ccb36b3ae5016cff2bff4ce89b4dd322abec703d455c1a7bcde5516318cf8` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_namespace__binary__version_2.rds` | 52 | `f1e73c2f916f876f961fa10ea4f901952139bfd00861280a77767558f52d3f76` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_namespace__binary__version_3.rds` | 61 | `85f1c1aa83571c0dcdf3b0c99297a4fe49e60d4a3ea12d63e5ea9492bbb5b602` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_namespace__xdr__version_2.rda` | 89 | `0e49e927f724b2c43107a5d1d5df9d9781a5075378e37e4b1fa0484492024564` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_namespace__xdr__version_2.rds` | 52 | `2c2ddea2a7b86515efa12795c3bc1636670f3a5ca5ff24b15de6eab2bc988662` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_namespace__xdr__version_3.rda` | 98 | `13f0510d4a8074f1d137b4c0f3a1478935573dd1f159d6b6a1a722e0f41b0f6d` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_namespace__xdr__version_3.rds` | 61 | `90a88654a67dab6c9985b680b6e1a09409d90ec0fd49c4478011537c90a59a59` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_nan_inf__ascii__version_2.rda` | 80 | `3560a714d24444953932c823a36385b9d71d239990ff93f89e00f04fa19ebfdd` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_nan_inf__ascii__version_2.rds` | 41 | `f5969be516922b431fe399e5a5fff307d1cf87602e1bda30505d9fe07b457625` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_nan_inf__ascii__version_3.rda` | 88 | `859fb3babe937aad0f85112a79173a6140da9a818ecc24d8df207d4161b947c4` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_nan_inf__ascii__version_3.rds` | 49 | `e65572a8a6a29b1c453a3b9db3867d1321f1f747c3fac3383b9b00475cef91c0` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_nan_inf__binary__version_2.rds` | 62 | `328f416a37657ac36680940bf050919afadbb2f5581e081cd36767056f082553` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_nan_inf__binary__version_3.rds` | 71 | `760bf7ad80f19e60567be7224870fdb65be1ba0d32a96d9e9430d00a680c09e1` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_nan_inf__xdr__version_2.rda` | 86 | `ed7f26041db8007783897c6ffacbed2cb17e850dbf76b1605803fb2a4a8163b7` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_nan_inf__xdr__version_2.rds` | 62 | `ba5d0c4c3d3c349d775699d43ead4bbb1e128b9152bbe752694d7966c1fc73a1` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_nan_inf__xdr__version_3.rda` | 95 | `4dc85596fbd2059500ef9fc63a8a6cd31d32ccab4325a1f2432714fe28c81e20` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_nan_inf__xdr__version_3.rds` | 71 | `0581d1ae7106cb10ed28a4abcc835be8a8bbbee1dffcfd51fdb9b524966d797c` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_nullable_int__ascii__version_2.rda` | 78 | `0f26ca7a3a05a84ffa9121841c9c8801b711338674ceaba3b668e59a915a4d78` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_nullable_int__ascii__version_2.rds` | 34 | `de7e81d52aebe81a79bed4657e1d5deb0563a898d1ec442775f84acdc3945ab6` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_nullable_int__ascii__version_3.rda` | 86 | `9d4f5fdb9fff97405f1db31ef53287130855b54b2094199e611f27db675cb5d0` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_nullable_int__ascii__version_3.rds` | 42 | `1756103a7fa2573558ea661c8fa8d7b4caafa14a2ccc70925c66e85ad5fab399` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_nullable_int__binary__version_2.rds` | 34 | `b812661ecb3114c1bfa686c0bcf27ccb24293d67e5736dc263f546e8cc6f492d` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_nullable_int__binary__version_3.rds` | 43 | `d859ff9f554a1038960559c164d3b3c0541280961f26cf7005ddee85e79c69e0` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_nullable_int__xdr__version_2.rda` | 85 | `7dbbc5abf0bc1ae9eec51d95fb01207f336941c6bfa7df9fc7895a516c806e42` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_nullable_int__xdr__version_2.rds` | 34 | `587d7768328ace438edee98f8759eb3bdb088b966329812fc1332c44606fec94` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_nullable_int__xdr__version_3.rda` | 94 | `8c319f3522ce99fa2453ac8d5409042b545677c2c8f7594a212be0c1daa9382e` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_nullable_int__xdr__version_3.rds` | 43 | `6e8f5b3abeea4c97d1f572e647a53cd2d882ead79841ecae10a000697d962832` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_nullable_logical__ascii__version_2.rda` | 78 | `63260c18d431eeb5773dc43455ccc4bc47f309063112b2980a768747d2b3a1dd` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_nullable_logical__ascii__version_2.rds` | 30 | `e3684a99b15736de2d75bfb2c8ebb87f18e0c13230afda9cfc3235f4f5056902` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_nullable_logical__ascii__version_3.rda` | 86 | `5bc29fec6318ab03f00d6abc9ecb69811459ec8217d57b1d62a04e878549f7e6` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_nullable_logical__ascii__version_3.rds` | 38 | `f0713fab82742f9e7a99fa82f61e2ee543325e3dbf6500b8b64fdf332ad5f875` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_nullable_logical__binary__version_2.rds` | 34 | `031de9a6653350715653083e6bc3cef0c85e134f39359fd6d669f6a67e873379` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_nullable_logical__binary__version_3.rds` | 43 | `e02b0018837220ba5ee194a6376b496e90dd62872070cc63ecce6012db439b15` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_nullable_logical__xdr__version_2.rda` | 85 | `e0e81b1deba8b70bc56e98a39592b9bca628c41564151fb46fabc6f83a1a6ab2` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_nullable_logical__xdr__version_2.rds` | 34 | `2c2a1894155590b3df84ca2e6c1e299c60345912bc05c1a74808848b2288e7c3` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_nullable_logical__xdr__version_3.rda` | 93 | `60f4b543b7a1d06916f234586d3b895d3f1b6ce12f7930af17e40e4301fef410` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_nullable_logical__xdr__version_3.rds` | 43 | `88a3e857abe44cb037129f4b052347624ba3d42ddfcaad1349c05271b29adb2a` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_open_file__ascii__version_2.rda` | 70 | `86cef3381f9f52824b06932aadac7d9b63a940daca10214e5d9a4a37e6698d8d` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_open_file__ascii__version_2.rds` | 29 | `4115f17a717220a9a6636929fa4373a2067dfe86be166e79b0a6105f6c2e6d1e` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_open_file__ascii__version_3.rda` | 78 | `0d959e855c441cbc58905b33704062cd3f37d748f2387b40d95131f344f58b16` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_open_file__ascii__version_3.rds` | 37 | `bf68453715157d6edc14c0de615f0dba7641ef6aafac21a78d83d81dff50daae` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_open_file__binary__version_2.rds` | 46 | `6597254e74df53e0de448d67d9713fccc2753bacbf3312694ca65a08e2aee841` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_open_file__binary__version_3.rds` | 55 | `375c90f7fbeea4b880ba6513439310d941e10578714d3320263dec2ad3644a7b` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_open_file__xdr__version_2.rda` | 82 | `67dbe0e80eed9abbc1804bd03b621069bc569a22a9becf664615a9a0677748ad` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_open_file__xdr__version_2.rds` | 46 | `e26c872255ba303ad3e2ba2ad33a193223f8e73a7068ef271711ef80931a8094` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_open_file__xdr__version_3.rda` | 91 | `246b5b66002a68d904493dd34a2d47d6f63655fc0293e505e6e8fc8433a530cd` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_open_file__xdr__version_3.rds` | 55 | `f6775a7bbd19e59301b91c1ff1b19e9f18a96378215ea87eb01777a94f5a7424` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_s4__ascii__version_2.rda` | 229 | `faf5d687d7be14f4911ff6b7a89100bffefac01ff606df810de7b341d2dad6e3` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_s4__ascii__version_2.rds` | 196 | `b6df1296e2dec43a6ef85669ce17045d2894ab6f18221b55974f29e83dd564b3` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_s4__ascii__version_3.rda` | 237 | `a188b640d85e3857af67db31edd0d69f8e7c34708b5e9fe76e96eddf2b3068e0` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_s4__ascii__version_3.rds` | 204 | `d9e19a32487aef6fcf11672e86678ed46f52804c16314742d3f94b11c49e7063` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_s4__binary__version_2.rds` | 195 | `1730da3f61e650b718147c2f4d5a710cf2c6090885ad9b74581cc42e7e5e4f1d` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_s4__binary__version_3.rds` | 204 | `c8d8769c2ce20fc110c8a5a1140c795d74a3d7ceebaf57fc44bc6b444ba20b68` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_s4__xdr__version_2.rda` | 139 | `0cdc8729c917466d5d67fcffd9f2cf7d16b277374d74bfdbba0e99f0cbd38b05` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_s4__xdr__version_2.rds` | 195 | `0ed64e3c35845d2ad55807a885ce0feeecf6221b38c5709ea971a78636794529` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_s4__xdr__version_3.rda` | 148 | `cc1690e5f21c35a2cc54badac021ce224ffbf35f8273e36285ffbb14d6b541e8` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_s4__xdr__version_3.rds` | 204 | `b597c2d9726ba5b872507509de1097605d4b32ddeff80b5e37552ed6d0f75b47` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_ts__ascii__version_2.rda` | 170 | `f63b21fca71431b0465c80913719776520952abe1937538f9ae887ea4d2b7fd5` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_ts__ascii__version_2.rds` | 137 | `39e21531145868c70154857107f98d0628b21a2e8b82b51bca6d0f3ab3dcdb46` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_ts__ascii__version_3.rda` | 178 | `02eb363cabe115a2f23262ed5ff00b90c47b9fb7d1405c1d692cadfe94e8ce20` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_ts__ascii__version_3.rds` | 145 | `6e1eec76f37038bd08226045096ce6957eab2ce24b539102eaae9f6a1a289049` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_ts__binary__version_2.rds` | 140 | `606cd99afd71cfe6b1ebb4da1f4f7dbef748c586090e40ee7c7d9e7d99896371` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_ts__binary__version_3.rds` | 149 | `e9641b13410b212c8588efdb4f78b5fab923270db29e9f14e778f67fbba6c2c0` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_ts__xdr__version_2.rda` | 121 | `f01fd46ee0a9899090892f1baea54ade847cb099ab471d365d1cbb657ee0b6f5` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_ts__xdr__version_2.rds` | 140 | `2274c4b15e5443c6e74f31e6515a980241dcc2a4fb9d9c5600a07ed2724236a4` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_ts__xdr__version_3.rda` | 128 | `73b598e731a938ed02a89e0a113779c7675449a4736f71d698d8a0a1f940a411` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_ts__xdr__version_3.rds` | 149 | `b75595c23fed3d28273f5d816ce46c6f07c3538366f2845bddca583b57c9983c` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_vector__ascii__version_2.rda` | 67 | `dea09f464121e12c476544a7dcb54f80eb66638cd128e802396700801deef00d` | rda `RDA2` · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_vector__ascii__version_2.rds` | 29 | `4115f17a717220a9a6636929fa4373a2067dfe86be166e79b0a6105f6c2e6d1e` | rds · ascii · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_vector__ascii__version_3.rda` | 75 | `ef6eb3c39e6537c9f1ef3f39f9c0c891735f08ffa159f65bff5c324818a6d570` | rda `RDA3` · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_vector__ascii__version_3.rds` | 37 | `bf68453715157d6edc14c0de615f0dba7641ef6aafac21a78d83d81dff50daae` | rds · ascii · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_vector__binary__version_2.rds` | 46 | `6597254e74df53e0de448d67d9713fccc2753bacbf3312694ca65a08e2aee841` | rds · binary · v2 · none · R 4.4.3 · little-endian | parse+convert OK |
| `rdata/generated/test_vector__binary__version_3.rds` | 55 | `375c90f7fbeea4b880ba6513439310d941e10578714d3320263dec2ad3644a7b` | rds · binary · v3 · none · R 4.4.3 · enc UTF-8 · little-endian | parse+convert OK |
| `rdata/generated/test_vector__xdr__version_2.rda` | 80 | `e058cce48e4c2bcab987c49c58e3f15e3c080a223005714de8ca2460f65775b5` | rda `RDX2` · xdr · v2 · gzip · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_vector__xdr__version_2.rds` | 46 | `e26c872255ba303ad3e2ba2ad33a193223f8e73a7068ef271711ef80931a8094` | rds · xdr · v2 · none · R 4.4.3 | parse+convert OK |
| `rdata/generated/test_vector__xdr__version_3.rda` | 88 | `3492d3c2eb47a8288339e7ff39648367f8c3ebf91d930403737f0261ffdeb574` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/generated/test_vector__xdr__version_3.rds` | 55 | `f6775a7bbd19e59301b91c1ff1b19e9f18a96378215ea87eb01777a94f5a7424` | rds · xdr · v3 · none · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_altrep_compact_intseq.rda` | 126 | `d773f3cde82fee925465d642934bc10228246bda10af928f40cf078d1dc9d5df` | rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_altrep_compact_intseq_asymmetric.rda` | 141 | `ba8c46f8d8ba676661edffe147069b45aca9f031f58bf00d0233b7dc54ce30c0` | rda `RDX3` · xdr · v3 · gzip · R 4.3.1 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_altrep_compact_realseq.rda` | 128 | `8fbf3cc99ea1cfc3855c28264393a5066f6b4019d9369028c0faaa537fa2a94d` | rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_altrep_compact_realseq_asymmetric.rda` | 144 | `db80ded405384e1ec76570f13f961f9073293b032603092799b436d03ff69626` | rda `RDX3` · xdr · v3 · gzip · R 4.3.1 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_altrep_deferred_string.rda` | 174 | `cfa639d4a966b218bf99695c1f3453a17ddbefe4f375a5d56ac3436f2d43ef8a` | rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_altrep_wrap_logical.rda` | 125 | `7ff919db9d340eff21e8df5b1bcd6ca64b8327d1a6a1b1d948af3c8d1df809c4` | rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_altrep_wrap_real.rda` | 125 | `67c75478700d810910337e643cc99e8da65d677f7d2c5a29480031e2eaf144e1` | rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_altrep_wrap_real_attributes.rds` | 119 | `b3800526b74793407e15d879795cfd8a68551221af45db19242f01bebb491d6a` | rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_altrep_wrap_real_class_attribute.rds` | 124 | `001311fcec69e296494b4558a540e00387b031a3d6973786ef23863433d1e64a` | rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_altrep_wrap_string.rda` | 130 | `bd1b5ac1aa9664c6efb74310dfa01ce1911e3c7579b34427ca8ea6e2f399d8ff` | rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_ascii_chars.rds` | 159 | `f530dadf27bfe67bf728dd1d74778e7ce207d6bc17cc3c544b8c76206d0a25c6` | rds · ascii · v3 · none · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_chars.rds` | 154 | `fde1a9155309af3f20ce7a9365baee6b59486f10def2d3efce426c063ef90a0e` | rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_empty_str.rds` | 41 | `01aadd213a882a3d77f727b61e4484b299f1cbedb86796d29b0e454a5336a0a8` | rds · ascii · v3 · none · R 4.3.2 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_na_string.rds` | 36 | `742591fcd07c907c67ffef9da37b1ef5d4f14b29da17300a9ff62d209138cdd0` | rds · ascii · v3 · none · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_nan_inf.rds` | 46 | `854ccf43e0945518ec756db1ab6e13fe7d49525af152334edc77da46b4316aa7` | rds · ascii · v3 · none · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_v2.rda` | 109 | `e622a2f2c75f77b9afc903d59dd5c7639c29f19cb3751761e78c736200b49986` | rda `RDA2` · ascii · v2 · none · R 4.3.2 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_v2.rds` | 79 | `d481b4682ce67f9b0a6bc35eae5d6f14307c2549d0f97e47824b003e24585908` | rds · ascii · v2 · none · R 4.3.2 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_v3.rda` | 117 | `7aaaf8d89e5f56dfddca243bedcc688c2eb0d65ccf645a8eed9546c1da5c21dc` | rda `RDA3` · ascii · v3 · none · R 4.3.2 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_v3.rds` | 87 | `88a414f91ef50b79494856157280f5fe07a0ce3d20574e2c5f401f48febef2e7` | rds · ascii · v3 · none · R 4.3.2 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_win_v2.rda` | 105 | `37fa06af3dd3261c559618e0bfe971d90928c6dc5e14c93cd4b156606fcc9ac0` | rda `RDA2` · ascii · v2 · none · R 4.3.2 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_win_v2.rds` | 99 | `f574308cd0e7087e0c4507fc2bfa8a5e4e5366ce816a1854e9359e137407b57e` | rds · ascii · v2 · none · R 4.3.2 · CRLF | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_win_v3.rda` | 114 | `8ce7aff8073b9da6f52d07a15684eaa2300ed7ef120484df57c9ec3c3d51c45b` | rda `RDA3` · ascii · v3 · none · R 4.3.2 · enc CP1252 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ascii_win_v3.rds` | 110 | `66c62eea1d731d15ed596383b5b42442b75dbd2f8085500aca77714024e86e96` | rds · ascii · v3 · none · R 4.3.2 · enc CP1252 · CRLF | parse+convert OK |
| `rdata/legacy-1.0.0/test_builtin.rda` | 79 | `e6955e3f329e9aa8f62f3072ecd60226046f235eedea2df9ad85b2f3401271e2` | rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_complex.rda` | 89 | `b028bf34ad7f372546f50855633fc35eccfe42e646a3cfdfda2a988879f1995d` | rda `RDX2` · xdr · v2 · gzip · R 3.0.2 | parse+convert OK |
| `rdata/legacy-1.0.0/test_dataframe.rda` | 176 | `5345babc49481158280c55956fdf42f1dd51d5f94f501aa132e7067bdf3daf47` | rda `RDX2` · xdr · v2 · gzip · R 4.1.3 | parse+convert OK |
| `rdata/legacy-1.0.0/test_dataframe.rds` | 153 | `8c5c8c1851d9ae0ba1b62d5daf02c6c73202d47d9daabbfa063b520747e8eeee` | rds · xdr · v2 · gzip · R 4.1.3 | parse+convert OK |
| `rdata/legacy-1.0.0/test_dataframe_dtypes.rds` | 217 | `c9263f52ac1f7c6ba933d19e9a7648fa84e16d79da312f6086ac552326b86459` | rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_dataframe_dtypes_with_na.rds` | 235 | `2b58456d75b56d2e132d734b2d6aa8f6e3c1745babdbe138ea821f2a06ba7b7e` | rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_dataframe_float_with_na_nan.rds` | 147 | `87c96c64c40b08b8e367bb2da7d9ec192473afd56bb5d3e8c071aec9db0ddb05` | rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_dataframe_int_rownames.rds` | 123 | `b5e5e9bec6d9c2f8b23152c69c413d8fdbd273802057a593802b764ca5ac1851` | rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_dataframe_range_rownames.rds` | 163 | `51e4e2a7976396ea4992157fab128fdf207b085c9323b43785e9ab6fda5369e9` | rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_dataframe_rownames.rda` | 216 | `20439c87d6048ce6b7e8168f901fa66a1182e6816f190ba5848f30a5003bfe18` | rda `RDX3` · xdr · v3 · gzip · R 4.1.1 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_dataframe_v3.rda` | 187 | `48cc0f52171bfd791fad3e8909612bc092ce83267c94c11a811716bb5a9d1d34` | rda `RDX3` · xdr · v3 · gzip · R 4.1.3 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_dataframe_v3.rds` | 164 | `7a1fe98c4d32926d7fc818e12066b1d496daf2bcad529b55fd67a8e0f48798cb` | rds · xdr · v3 · gzip · R 4.1.3 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_empty_dataframe.rda` | 125 | `31571022524d596ca467bf09c43a58a447f2c0c482ea75f338e5da2f2d56216a` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_empty_dataframe_without_names.rda` | 132 | `31e634be850418dec281aac53be7382c4b5251d77de7a1b92b11a620afcbafe3` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_empty_function.rda` | 323 | `3063c8c5f46c46dcc7f2d78bca3a1c6d7ab1468cb695d290f5ecb660b67fc5ec` | rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_empty_function_uncompiled.rda` | 271 | `47e658679c6342d1341b1ce07b359820859d237413693ac1d02e9041cc29bf12` | rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_empty_list.rds` | 45 | `683997ad9ca8e42fa8cf2597d1be738c96af0afdd78caa4a75e91d47bffc0694` | rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_empty_named_list.rds` | 69 | `a944cc1a456509685a54ba08913709606e4882bf6283f86e1360f1f2cc780ac4` | rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_empty_str.rda` | 72 | `eb6e82f13aebf40ed54159907299172f3bdc1e5ec341f23698dd209501ae3e3a` | rda `RDX2` · xdr · v2 · gzip · R 3.6.3 | parse+convert OK |
| `rdata/legacy-1.0.0/test_emptyenv.rda` | 69 | `08934eb7135a21013ae5bfe0a0c5c8d768c741abca7ac4fc44447db46d717f7b` | rda `RDX2` · xdr · v2 · gzip · R 3.4.4 | parse+convert OK |
| `rdata/legacy-1.0.0/test_encodings.rda` | 175 | `13896374d10820add3584cb526fd016a73ea4b2990eaaa240e9df94e3f25574c` | rda `RDX2` · xdr · v2 · gzip · R 3.6.3 | parse+convert OK |
| `rdata/legacy-1.0.0/test_encodings_v3.rda` | 184 | `5349f49887068979aeaa8c64e6e30cb8b5f3b31ebaf123761c7f9da0428439d3` | rda `RDX3` · xdr · v3 · gzip · R 3.6.3 · enc CP1252 | parse+convert OK |
| `rdata/legacy-1.0.0/test_environment.rda` | 103 | `d8ddf888469eec9cce59bbe184893aa70e0c62daca8e00bdf9752d17df3949d7` | rda `RDX2` · xdr · v2 · gzip · R 3.4.4 | parse+convert OK |
| `rdata/legacy-1.0.0/test_expression.rda` | 100 | `022548ff86a41dbca2f721b82b067e2360e3b2dd833d50e319a7723d7ef65af9` | rda `RDX2` · xdr · v2 · gzip · R 3.0.2 | parse+convert OK |
| `rdata/legacy-1.0.0/test_factor.rds` | 100 | `223949b3c7f1dcd94efc39b111166e76c6018ed3091a3fde33f981f9f2d7378b` | rds · xdr · v3 · gzip · R 4.1.3 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_file.rda` | 124 | `ff1c4b9d899d0226d0916d1c8422b104e7eba6ad7f3e2946f45121855e8ab00b` | rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_full_named_matrix.rda` | 184 | `b82b56c01a3b2419a82c337dd56551f0a1f002b7a464f0d900f6320856e71927` | rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_full_named_matrix.rds` | 154 | `268f4a22806a4d74bc8926b321f461c0b534805776fd912b14662645bb4f20a2` | rds · xdr · v3 · gzip · R 4.1.3 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_function.rda` | 382 | `d4b3190ba11b1af56388b12d5a6ae25b36e3dcbad5aaddc1e5d1b5a551021192` | rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_function_arg.rda` | 409 | `e8b6d7f612ecd659b5c298e26a950d919087ad7246adbaebbb2fb5e2019f7973` | rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_half_named_matrix.rda` | 152 | `41a46f6de5812b02d318f7ee7cdf1f5b0608749b484f8a1175d4d5afa318f464` | rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_list.rda` | 103 | `ba94f7b0d4c72ec0c7e26c836258438a2bf4fb689271b36e40bb618c4c5a085e` | rda `RDX2` · xdr · v2 · gzip · R 3.0.2 | parse+convert OK |
| `rdata/legacy-1.0.0/test_list_attrs.rda` | 111 | `199d520ec6c4d59eb1addaf1e1721ad43214c78fd00bd117de0b2596f7d7cbaf` | rda `RDX2` · xdr · v2 · gzip · R 3.4.4 | parse+convert OK |
| `rdata/legacy-1.0.0/test_logical.rda` | 73 | `a86b2a5322097cdf03d8ce01873d88d0b986efb892d0490a02e05c66e7a71edc` | rda `RDX2` · xdr · v2 · gzip · R 3.0.2 | parse+convert OK |
| `rdata/legacy-1.0.0/test_matrix.rda` | 102 | `e3f1269605b80dfe61ae3199d2e43a1ccb9a7a41db5fb0b65be07bd421919da8` | rda `RDX2` · xdr · v2 · gzip · R 3.0.2 | parse+convert OK |
| `rdata/legacy-1.0.0/test_minimal_function.rda` | 275 | `c011d0d7ced25008f735cb09c03f12af230c02d248c15a10cd00c97c4d403a77` | rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_minimal_function_uncompiled.rda` | 217 | `ea9ab23d9c10bbdf36448f22af42a506f1ecc77af548910a6c610b2f272b6d5d` | rda `RDX3` · xdr · v3 · gzip · R 4.2.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_na_string.rda` | 78 | `808ee0b33252fa3322499116e5febf2bab62ed0bfb86f6bb49c9a26af3340472` | rda `RDX2` · xdr · v2 · gzip · R 3.6.3 | parse+convert OK |
| `rdata/legacy-1.0.0/test_named_matrix.rda` | 161 | `42cbe5804b074fb346e8d01cbe7de355d41c2e86bdeceabe79ad38a24f5b4c72` | rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_named_vector.rda` | 119 | `c7d1fb3065cbbbd9eb1be32ef5e3ed5ebfae1d5c880dd905d8b29f2693c2b724` | rda `RDX3` · xdr · v3 · gzip · R 4.4.3 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_nan_inf.rds` | 57 | `d49439c23ee38da33538f7fbdc4b513fce46a56f1b7696d855a5f12ee8918125` | rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_nullable_int.rda` | 94 | `591cc734320f3aa4426e3a27e2184a89c1e4914ace6ec83dcb177bbcffc9f779` | rda `RDX3` · xdr · v3 · gzip · R 4.3.1 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_nullable_logical.rda` | 93 | `6d7b43b3e159737e94566d58f877c02f3f9090bae4f1bb587fe9287448f99683` | rda `RDX3` · xdr · v3 · gzip · R 4.3.1 · enc UTF-8 | parse+convert OK |
| `rdata/legacy-1.0.0/test_s4.rda` | 139 | `c84966335541868484dda2cff5c0435e8042a2ab30980f870ead943697631bc1` | rda `RDX2` · xdr · v2 · gzip · R 3.4.4 | parse+convert OK |
| `rdata/legacy-1.0.0/test_ts.rda` | 119 | `1b1425fdc28224a80a214c623d7d15cc2ea2fa30c23755bd71a83ccca014972c` | rda `RDX2` · xdr · v2 · gzip · R 3.0.2 | parse+convert OK |
| `rdata/legacy-1.0.0/test_vector.rda` | 78 | `62534179c059fb050c4f022327ae628ed35fb3f0f76ad6fac513eb39b5cad30e` | rda `RDX2` · xdr · v2 · gzip · R 3.0.2 | parse+convert OK |
| `rdatajl/data_v2/NAs.rda` | 239 | `d059d531a75030c9c72e87577fd5b1d8a48922e48a1648d3ffb34cad261c4d39` | rda `RDX2` · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/NAs_ascii.rda` | 443 | `09c18e42c263b3b6da359be062ab591598c9cbf63131e0d2cbfde3c9476b6bff` | rda `RDA2` · ascii · v2 · none · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/budworm_glm.rda` | 6572 | `661cd64e5916b33dc7a172ce23c7176114acdcc6a66c7c7e28485896f344d100` | rda `RDX2` · xdr · v2 · gzip · R 4.1.2 | parse FAILS: ValueError('252 is not a valid RObjectType') |
| `rdatajl/data_v2/closures.rda` | 285 | `be3f1b2cc8d346d477f7c172a8fd65042ec2bb0989e82e3b3e1b066ada6c5dfe` | rda `RDX2` · xdr · v2 · gzip · R 3.5.2 | parses; conversion fails: AssertionError() |
| `rdatajl/data_v2/cmpfun.rda` | 289 | `3463836db03d31f8cc48046b5f43ca0c79222e8833b08f3a7aad8c0b06cbca08` | rda `RDX2` · xdr · v2 · gzip · R 3.5.2 | parses; conversion fails: AssertionError() |
| `rdatajl/data_v2/datedfs.rds` | 243 | `20c9e4d937c8a690b1f25d077a42aa3f10cae199a115ffe44735dfb80f012291` | rds · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/datesNA.rds` | 139 | `21528ddc3f9c39dd0e44f91b52cee650f853f2778bfc73590d0344f033f3eab1` | rds · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/datetimes.rds` | 172 | `57c21d8c47f4904e2492fe8d02319e54e644965525d2ace9f5b2990fcf39409b` | rds · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/datetimes_tz.rds` | 145 | `2e767626b95be5377304c1fde1d344301a8d76ed78c654dd51742197c3e175b4` | rds · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/dfattributes.rda` | 406 | `78a0a243d7b0171b984e4fffe6b1a7850c11193e688f535837f315870c5c7b66` | rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v2/dotdotdot.rda` | 79 | `afc1d841b8d72c197a30e446e793ba369352c761b82b65c303506c96ca2fe334` | rda `RDX2` · xdr · v2 · gzip · R 4.1.2 | parses; conversion fails: NotImplementedError('Type RObjectType.DOT not implemented') |
| `rdatajl/data_v2/envs.rda` | 148 | `84f12e5cc55a250a009b81999f023ccb2a927d9058c926b955e22c0ba9f44745` | rda `RDX2` · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/expr.rda` | 574 | `6c5bfa14bc68bf97ae009ebc7da86ea151ff9d6519fde9216d762f5dc640d29c` | rda `RDX2` · xdr · v2 · gzip · R 4.1.2 | parses; conversion fails: AssertionError() |
| `rdatajl/data_v2/intdates.rds` | 79 | `538c6c937d8d688a515f4f4074ab31e76f0e05bd2a4500caaaedcade1bef1654` | rds · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/intdates_ascii.rds` | 85 | `63b292bafc522b097f06f1888cc9cb6b53c205a725531ddecbf61d84e41b8e3f` | rds · ascii · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/list_of_vec.rda` | 292 | `e2c6229c385519fd8dee802724f8fcb47c3728c53af1659a43c1cef6946f3c73` | rda `RDX2` · xdr · v2 · gzip · R 4.0.4 | parse+convert OK |
| `rdatajl/data_v2/minimal.rda` | 130 | `e5164b3dff534de7711454edb75cff8bb5c2a96799e9b8324f12ca320566c2b7` | rda `RDX2` · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/minimal_ascii.rda` | 194 | `b6b9d0325878feaa78f5be73427b7880d6d354fae43c060dbcdb2f691040c459` | rda `RDA2` · ascii · v2 · none · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/names.rda` | 240 | `e1162dc6c199fa05df8f5640bb26476064a1a0b432455ee8d827329587c9f629` | rda `RDX2` · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/names_ascii.rda` | 429 | `423bee0781b78b66b46c3ad0b58b5b16a5097b08291096cdf4cf187cda5c717f` | rda `RDA2` · ascii · v2 · none · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/numdates.rds` | 84 | `213c2aabe304d9c382e385f0151d728becd1b197d959634b7d6d206d92e1f2de` | rds · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/numdates_ascii.rds` | 85 | `97a7fcb839f0ad27df2f06f6b0e3c7af0a76bb486529edb5c98ac14e6c7e5acb` | rds · ascii · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/ord.rda` | 145 | `e148db7edd7a46c771ebc099e152457fecebbe9bffb95a9903eaa4d1f9abe376` | rda `RDX2` · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/pairlists.rda` | 115 | `ace654d42cd6b1387bb491fa452ca15af39b53415e2a50d7d5228230b1dbb500` | rda `RDX2` · xdr · v2 · gzip · R 3.5.2 | parses; conversion fails: TypeError("'list' object is not a mapping") |
| `rdatajl/data_v2/types.rda` | 225 | `ba24d601b3d047e9ebb80666421117e523920a91addffb1dc4d2f1846d19bd86` | rda `RDX2` · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/types.rds` | 210 | `a24f23a823dfc712833b15b04bbdb0c151d0d6801ff801db1b3ac34a726f3825` | rds · xdr · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/types_ascii.rda` | 423 | `43abab033f5ddf6521feb8b03484a8c3cf83c9e5578462bf1de18508238faf8a` | rda `RDA2` · ascii · v2 · none · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/types_ascii.rds` | 198 | `ef14ddbeddd3e8b2a87e191907e5b20c5e963e2b454f67b6b727052fb43ef4b6` | rds · ascii · v2 · gzip · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v2/types_decomp.rds` | 436 | `3f0bb2000dc62fdbb3f98f8916c38c4439810b0c9d57a245d526b00b962eac10` | rds · xdr · v2 · none · R 3.5.2 | parse+convert OK |
| `rdatajl/data_v3/NAs.rda` | 249 | `a088fe68164a3fb254223118f8aadcf5be87ff0e6146c43027dd74f2a08ae6b5` | rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/NAs_ascii.rda` | 451 | `e0b2be780e41798df746f140ed37e47b5578e1bdce09d09588b875ba1588486b` | rda `RDA3` · ascii · v3 · none · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/altrep.rda` | 442 | `7de77b055ffbdc1cc47e5de2b5b8242957b7d2fde88ed1d25e04be581d8b23c1` | rda `RDX3` · xdr · v3 · gzip · R 3.6.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/altrep_ascii.rda` | 405 | `8b8d3700ff02bc6d1ddaf62fba9f6860fe7f524dfe4a26de14db85f75b0a04d7` | rda `RDA3` · ascii · v3 · gzip · R 3.6.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/altrep_names.rda` | 224 | `d9543d6223776bca9ae0490c45eaea57c2bea906f5a3db028e65f49bb354534a` | rda `RDX3` · xdr · v3 · gzip · R 4.0.0 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/budworm_glm.rda` | 6681 | `5b0cc7954961f5cc774fa693fe531c207acfd8d0494a1a31791ca1acb60bbd81` | rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8 | parse FAILS: ValueError('252 is not a valid RObjectType') |
| `rdatajl/data_v3/closures.rda` | 295 | `2ab544df68aaf351e45ff9ef4af9c0ac1d15c69ffa674b94806af27dcb5d6fef` | rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parses; conversion fails: AssertionError() |
| `rdatajl/data_v3/cmpfun.rda` | 298 | `c6925d347a4cd909c3e0c142bbfa6fb8ec1656b05b3396ec071320ef5daf6113` | rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parses; conversion fails: AssertionError() |
| `rdatajl/data_v3/compact_seq.rda` | 2313 | `f890c91623f45e20812cebe936a47ed7e942c2d26fae5a43b8fd765c693d2dc9` | rda `RDX3` · xdr · v3 · gzip · R 4.0.5 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/compressed_bzip2.rda` | 161 | `be22fc280bd655064c1f0bc9be158b26bb3df9f7b37d9538d361df0601d64bd5` | rda `RDX3` · xdr · v3 · bzip2 · R 3.6.1 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/compressed_false.rda` | 214 | `5df53189770a7cf46678aa71de4e9ac7e8f0dc4120390bb9e178395baa059bb9` | rda `RDX3` · xdr · v3 · none · R 3.6.1 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/compressed_gzip.rda` | 139 | `e5c840c28988a4ea25c65b0c83851d14ba45a62d500df02067e41214dd129b83` | rda `RDX3` · xdr · v3 · gzip · R 3.6.1 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/compressed_xz.rda` | 180 | `25cba4da89c2f2f32237df3dd74fe71f8903ebd8c118167ddd4fc14b5d33ab39` | rda `RDX3` · xdr · v3 · xz · R 3.6.1 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/datedfs.rds` | 250 | `8baa5205892cb0a8a6033225cec9c6d7e8c70ebbcee4f05dcf80562511926cd3` | rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/datesNA.rds` | 145 | `61d36f373bbaaec309604c429023a451cb80973c24de48b6fc76f500cd72294c` | rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/datetimes.rds` | 177 | `2b30cc9748be3a603ba8ba008b63c7c8d461c7f709eb25573221820cb0fe8de6` | rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/datetimes_tz.rds` | 151 | `cf113a0e9370fcf3c690d73a3926b29ebfe7ab8f803e3fd62842328673fe0a14` | rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/dfattributes.rda` | 406 | `78a0a243d7b0171b984e4fffe6b1a7850c11193e688f535837f315870c5c7b66` | rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/dotdotdot.rda` | 89 | `a7c9d4623460eff4ba8489fc6e773dce41595fd316f09eaadf531f015793e722` | rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8 | parses; conversion fails: NotImplementedError('Type RObjectType.DOT not implemented') |
| `rdatajl/data_v3/dup_levels.rda` | 272 | `b8c35049d19e5f0d616d81d1ee6905dd1a9032f750e6e5c3aeea022f4199deaf` | rda `RDX2` · xdr · v2 · gzip · R 3.4.4 | parses; conversion fails: ValueError('Categorical categories must be unique') |
| `rdatajl/data_v3/envs.rda` | 157 | `0ac1ecff1c438003e807356869d8ef5aa5a11a6314ab0cdb6edd8cc3eb464c08` | rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/expr.rda` | 584 | `f486dd65230045eb64d094e2e4e23fbd1fcd485e442684b10d4a66733b59f5fd` | rda `RDX3` · xdr · v3 · gzip · R 4.1.2 · enc UTF-8 | parses; conversion fails: AssertionError() |
| `rdatajl/data_v3/intdates.rds` | 85 | `e383ee06b7a0b605eb052ec8ab11e4bc65477c5756743e7f143a445b81379060` | rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/intdates_ascii.rds` | 95 | `e50bf933c1879ac8442c0b37f67a54f871147d44e71fb45892721ac5b6de49d7` | rds · ascii · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/list_of_vec.rda` | 304 | `6f8d004a8abe667c517d411d42b07dad1aa0045043e532194d1b137150764387` | rda `RDX3` · xdr · v3 · gzip · R 4.0.4 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/minimal.rda` | 138 | `de2937cbabf022256b584465bd2def5f0ce7eb12890b2479ed12d25058084b06` | rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/minimal_ascii.rda` | 202 | `8d1530de397b1776ac21b94626c84983c663f01baf1eb414ed1d59f402c09661` | rda `RDA3` · ascii · v3 · none · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/names.rda` | 246 | `63ba4d2004304c44c87b361cd19b88e9eef47054fc5b020cd509095510ca8d74` | rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/names_ascii.rda` | 437 | `b88513b6ae95a275d9386acbf6f0bf384180e9b23e027e3820dbff1335dc78d1` | rda `RDA3` · ascii · v3 · none · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/numdates.rds` | 91 | `370492c2a1161d29d23fcfc80667ba0a4e8cee7d61485b34f433ca22a2f66e16` | rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/numdates_ascii.rds` | 95 | `1e358e5fa11270626e41c812e9cc55196ae7eb91507f2c920db67953e941f5f7` | rds · ascii · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/ord.rda` | 152 | `2157978cf7fe781f95c477d0ae34c5525d1abbc0e42afc381b94a2241fb0975b` | rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/pairlists.rda` | 123 | `83fdbf374a9a50bc1b42235a55f77ebd47fe81726f0df0a2387fbeaf2c362649` | rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parses; conversion fails: TypeError("'list' object is not a mapping") |
| `rdatajl/data_v3/types.rda` | 233 | `19a9be545281417ff22760f13c669427dd09d1a05881ade02a3432e9504a087e` | rda `RDX3` · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/types.rds` | 217 | `764c89e02db79ff474487802a504f6ba31cb07bbca5afc16dbdf21fce6eb090b` | rds · xdr · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/types_ascii.rda` | 431 | `33c123753aeeb7db96f851269a9efe213f3e49feff62af2bfe256191c8e494a1` | rda `RDA3` · ascii · v3 · none · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/types_ascii.rds` | 204 | `6abe8f77ef6baf80235d7dd7ffd73cb1ae27df9e74cfbabdcc4e802015fbd91c` | rds · ascii · v3 · gzip · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `rdatajl/data_v3/types_decomp.rds` | 445 | `f8298aab7bf1f8288f90b8b4087f25b31ee3793fa7ee2b5ab2a4a95559c42889` | rds · xdr · v3 · none · R 3.5.2 · enc UTF-8 | parse+convert OK |
| `sce/mouseBrainSubsetSCE.rda` | 96123 | `a20843f62fa36f1f7514a65fb5bb3913967eb4528fab38c5b96a63f79f11600a` | rda `RDX2` · xdr · v2 · bzip2 · R 4.3.1 | parse+convert OK |
| `sce/scExample.rda` | 44473 | `547800b763b1a4034849acfb8513a4aeb278d9d46caaa2bd2739243efc66fd00` | rda `RDX3` · xdr · v3 · bzip2 · R 4.0.2 · enc UTF-8 | parse+convert OK |
| `sce/simple_sce.rds` | 86567 | `c5471998682748eb6a3148e923510992c88c53db6cb083cb68284ecc7862a3d3` | rds · xdr · v3 · gzip · R 4.4.0 · enc UTF-8 | parse+convert OK |
| `seurat/pbmc_small.rda` | 61056 | `b0532db6b5e09e9e2c6d61c880ad31eea96697bb92b4c44bcea3e972d54a1c74` | rda `RDX2` · xdr · v2 · bzip2 · R 4.0.3 | parse+convert OK |
| `seurat/scRep_example.rda` | 208112 | `4d532546676d783459257e36856ecba65ee48ceb8ebc1bed6705faba2ba072f3` | rda `RDX3` · xdr · v3 · xz · R 4.3.0 · enc UTF-8 | parses; conversion fails: AssertionError() |
| `seurat/v5/seu_overlapping_layers.rds` | 1260 | `c823ed1868a02c582dd2323082ca0fc684743fa9c8ec23f1981f9e94ee66ddd3` | rds · xdr · v3 · gzip · R 4.5.2 · enc UTF-8 | parse+convert OK |
| `seurat/v5/seu_split.rds` | 2016 | `59895bdd2d88f0a9994ab06282eefb5e3d32b0c6ff4c055f72545396e427a47a` | rds · xdr · v3 · gzip · R 4.5.2 · enc UTF-8 | parse+convert OK |
| `seurat/v5/seu_split_diff_features.rds` | 1093 | `c9de5a37fef3769287c0729df0b47d3774e92cfbaa358a03dceb4af25afdd275` | rds · xdr · v3 · gzip · R 4.5.2 · enc UTF-8 | parse+convert OK |
| `zurds/builtin.rds` | 49 | `b58e62a13d998dc1607922a787c3f90c75dcdc7bf649bee89916e6b42bcfc378` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/bytecode.rds` | 201 | `bba37dc431b575cfa0c54468f52c2e638707727cf97056d1088d392316f1254a` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parses; conversion fails: AssertionError() |
| `zurds/bytes.rds` | 56 | `7376c15607e59fd89ba6075318b643586bfca101562aca03036a356c596f2f28` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/character.rds` | 81 | `ffb171ab3d8bc94cc5217277d0db8f5f64bfe55dd4b63954a13a1ac106e1909e` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/closure.rds` | 88 | `4490f40b4fc31ef10c81c7e9316e9b9e2dd7c6cabdc0a2501452223c874d49fe` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parses; conversion fails: AssertionError() |
| `zurds/compact_seq.rds` | 99 | `c03c2fe0701023702d73f32d823e514806c63ccf3c35c60f07c722bc2bd00540` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/complex.rds` | 67 | `6da2e0a41074fd4477e6cf3f10568bb39b9694ebc8435343ec8136ffa99cfda2` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/data_frame.rds` | 138 | `842cefda814146005d84bec14ff90dc0091c0c92eca608b2f130ee79f4f69aa7` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/deferred_string.rds` | 134 | `404ddcef353c3c3f719339c2e2effb3401a1e65f1022158bab84010c7ca4b39e` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/double.rds` | 78 | `a3d1834c19f43cae4eb7e4be556c2f7273e992a381018cac50f25426cea01a86` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/env_cycle.rds` | 81 | `a630f40c243f3a136ceacc14aa56f1dee029522fd93b65abcf3e0a16669da78a` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/environment.rds` | 80 | `dfebe6dde5b6cf5af1d0e86aec5b21cb869724a12b8b7e5cf159f5d25352b456` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/external_pointer.rds` | 48 | `cd6516cf570451ff592dcb0be77bd0bc1c8cead69f2a72b65e9c73601dc10bba` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/factor.rds` | 101 | `7b6776e34c039d8138fff0cec2a3ba97c1dae8447f60377cd9ddcce953807441` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/formula.rds` | 142 | `43aad2577bbcbea96a1dea09d0077783429ddafbcda90ab2b0b2dc42e5b7135e` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/integer.rds` | 59 | `9739cf9be42407df09b309aace5e47965341e664c8df01fd1930359afa41e800` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/language.rds` | 74 | `dd65dc1a5cff448eb60ea91b1b253c84e4747f361aa9c44ce89a589680e03faf` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/list.rds` | 70 | `34d143d82665f2572da74c937a56bde0d1db3e1ebdb9d395aabf2066170197d0` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/logical.rds` | 52 | `afd2fb2cba54050778ddc0a506e6c18f8e2561845cd1f46d36c67e8a13d18d8c` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/matrix.rds` | 110 | `2158ead8e6eab78903403712519725930f243b36f06b9713ee7d25a1d43c4ca0` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/multi-ascii-v2-gzip.rda` | 252 | `675ac0caa4407962ddba66a639e33467f7d353b6812c0bda3e403ed1bc84c2e6` | rda `RDA2` · ascii · v2 · gzip · R 4.6.1 | parse+convert OK |
| `zurds/multi-ascii-v3-gzip.rda` | 262 | `2e056855c396b479d6aea952da1ceccbe4326ad79ca0fa14b171cd74dd34c52d` | rda `RDA3` · ascii · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/multi-binary-v2-gzip.rda` | 257 | `d357fb0a7312f1c690ecb3490900fec6e8dd6235d5edf42606b2b31303c45b15` | rda `RDB2` · binary · v2 · gzip · R 4.6.1 · little-endian | parse+convert OK |
| `zurds/multi-binary-v3-gzip.rda` | 272 | `eee59e5af71473200c5c2021a9579cbedc607b15a85e9706d5d2ef5cbd82aece` | rda `RDB3` · binary · v3 · gzip · R 4.6.1 · enc UTF-8 · little-endian | parse+convert OK |
| `zurds/multi-xdr-v2-gzip.rda` | 255 | `02f3857c80fdba6133ce04a146e9d2dbc4a7454433b16286dbf796f92762f34c` | rda `RDX2` · xdr · v2 · gzip · R 4.6.1 | parse+convert OK |
| `zurds/multi-xdr-v3-bzip2.rda` | 277 | `b6f0766fc0392fd1412f3ec299f3ee6cf12e7cf80f6f31ae5d1674a158c6fe77` | rda `RDX3` · xdr · v3 · bzip2 · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/multi-xdr-v3-gzip.rda` | 266 | `c454bf031ca6176b30b23dd108d319813700bcd5a2ac10ad267632e289766d21` | rda `RDX3` · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/multi-xdr-v3-none.rda` | 535 | `0bd5f98abf58b4e8662fa389f754ad0ce59f2fa003ffae22ba071ea44f9f9fb4` | rda `RDX3` · xdr · v3 · none · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/multi-xdr-v3-xz.rda` | 292 | `2e1a77b6d4242a1268815733a8eaad3a106df5ca2d5e8fd49e2ceb40f74b9938` | rda `RDX3` · xdr · v3 · xz · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/multi-xdr-v3-zstd.rda` | 262 | `0c97fa734e684a88f7f266a2617be511aac876685487c6b240a3e67ce2ad9c65` | rda `RDX3` · xdr · v3 · zstd · R 4.6.1 · enc UTF-8 | parse+convert OK (zstd decompressed first) |
| `zurds/named_list-ascii-v2-bzip2.rds` | 106 | `7d8ced18a2f5b0947379da2222de7d54095b338c74418379f80401103780ad85` | rds · ascii · v2 · bzip2 · R 4.6.1 | parse+convert OK |
| `zurds/named_list-ascii-v2-gzip.rds` | 98 | `730d24515d08b4e9368140e3574f83add8c4de0ea1a321dfa4c392d80dc28842` | rds · ascii · v2 · gzip · R 4.6.1 | parse+convert OK |
| `zurds/named_list-ascii-v2-none.rds` | 138 | `bdd6ef435593554e880ae30841d264ab2dac5e348f0ac21e20d72e8c6e196935` | rds · ascii · v2 · none · R 4.6.1 | parse+convert OK |
| `zurds/named_list-ascii-v2-xz.rds` | 144 | `fb4ce128c83885cf31ab6bf0c01a6d4afba66954d413afc243e0ad525a3e9205` | rds · ascii · v2 · xz · R 4.6.1 | parse+convert OK |
| `zurds/named_list-ascii-v2-zstd.rds` | 94 | `41fb4c158f9b7aa4f88c1cda1b896c62623a2aeba5cd23674c46eeabc6b06e02` | rds · ascii · v2 · zstd · R 4.6.1 | parse+convert OK (zstd decompressed first) |
| `zurds/named_list-ascii-v3-bzip2.rds` | 118 | `bf8522627edb5d3998552b5f57301b3446117fbfe50056ccda5ea74d4ef60f6f` | rds · ascii · v3 · bzip2 · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/named_list-ascii-v3-gzip.rds` | 109 | `933f1ab5fbccd3b2874a61f376a9ee9a058f13be6842d88d190c32b750b64803` | rds · ascii · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/named_list-ascii-v3-none.rds` | 146 | `0e11413d77cfb1fc3ca7cca72bed02571cba6d4a5b45121a93e9441552d23fb6` | rds · ascii · v3 · none · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/named_list-ascii-v3-xz.rds` | 152 | `e9fd643353afc3aa02048d344e20e7d98cfa9037e11501597f11812bced7ddb6` | rds · ascii · v3 · xz · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/named_list-ascii-v3-zstd.rds` | 102 | `f3951cc73fe82480dd584a6c9778a9e6627d84c750b64fc78a4852888efa2ea5` | rds · ascii · v3 · zstd · R 4.6.1 · enc UTF-8 | parse+convert OK (zstd decompressed first) |
| `zurds/named_list-binary-v2-bzip2.rds` | 108 | `5069f8455238a954b1e2d2364cf45b9a95ea12d2ae7da3dd2737a6f211ba610a` | rds · binary · v2 · bzip2 · R 4.6.1 · little-endian | parse+convert OK |
| `zurds/named_list-binary-v2-gzip.rds` | 97 | `22fdc9f25dae9409400ab8296a14ec780af52ff230f150c3a032f3d2c327692b` | rds · binary · v2 · gzip · R 4.6.1 · little-endian | parse+convert OK |
| `zurds/named_list-binary-v2-none.rds` | 146 | `9f3a13abd34eb86e8e3dc1309f00d86df82b40465ce10f24d4123dd3eaea06a4` | rds · binary · v2 · none · R 4.6.1 · little-endian | parse+convert OK |
| `zurds/named_list-binary-v2-xz.rds` | 136 | `657980cd01ed1dc6c9199e668b9d8138d62c40c5b6c8671f17880d80c55feed0` | rds · binary · v2 · xz · R 4.6.1 · little-endian | parse+convert OK |
| `zurds/named_list-binary-v2-zstd.rds` | 103 | `c2c0c3db388ed5365825b008d681edd72054f9dacb99b6c1506afbc687658e8d` | rds · binary · v2 · zstd · R 4.6.1 · little-endian | parse+convert OK (zstd decompressed first) |
| `zurds/named_list-binary-v3-bzip2.rds` | 122 | `1aaf75b4811af23dd89926164391010534887cfc42b30463477ba3c173e9e5b9` | rds · binary · v3 · bzip2 · R 4.6.1 · enc UTF-8 · little-endian | parse+convert OK |
| `zurds/named_list-binary-v3-gzip.rds` | 106 | `5097084d0e3caa12e0215ca305574caa7d9ecff60330ec76ca44be5e1793a092` | rds · binary · v3 · gzip · R 4.6.1 · enc UTF-8 · little-endian | parse+convert OK |
| `zurds/named_list-binary-v3-none.rds` | 155 | `7dfb967982b37484f6e26d1cca09ee99db3c2aa9c161e809d7e636a29316fd7d` | rds · binary · v3 · none · R 4.6.1 · enc UTF-8 · little-endian | parse+convert OK |
| `zurds/named_list-binary-v3-xz.rds` | 148 | `278985f65da4a9e79d03a760d637b80409dea61f8e467d2453ffe1c92710f6a8` | rds · binary · v3 · xz · R 4.6.1 · enc UTF-8 · little-endian | parse+convert OK |
| `zurds/named_list-binary-v3-zstd.rds` | 110 | `9bb90e193ab10336e6b9b5ee4d3fbc56db7b4b4f6d062af8b46371505ce06fce` | rds · binary · v3 · zstd · R 4.6.1 · enc UTF-8 · little-endian | parse+convert OK (zstd decompressed first) |
| `zurds/named_list-xdr-v2-bzip2.rds` | 111 | `0d35bc4800cac9879d039e09a13c6bebb722d3e2c9aae504adf1f8a2f30cadff` | rds · xdr · v2 · bzip2 · R 4.6.1 | parse+convert OK |
| `zurds/named_list-xdr-v2-gzip.rds` | 96 | `86ef08042fb33f46f3a106d0902d7c71fb44b4b5c8cd56f5f0873a3304c7ccaa` | rds · xdr · v2 · gzip · R 4.6.1 | parse+convert OK |
| `zurds/named_list-xdr-v2-none.rds` | 146 | `a10fc544d3180639ac6252133f4ba5d89bb991f9a0e1ba7cf2db3b324bda56db` | rds · xdr · v2 · none · R 4.6.1 | parse+convert OK |
| `zurds/named_list-xdr-v2-xz.rds` | 136 | `964763c101c84a7dbb3bdb754b120fb68683ea0f026811a8adc22e54c01bffab` | rds · xdr · v2 · xz · R 4.6.1 | parse+convert OK |
| `zurds/named_list-xdr-v2-zstd.rds` | 103 | `6ac2dc841d83b51da7a5e13dcfe7aa4c330b4d94fa6e74eb7c62ac5c4f7dd1bf` | rds · xdr · v2 · zstd · R 4.6.1 | parse+convert OK (zstd decompressed first) |
| `zurds/named_list-xdr-v3-bzip2.rds` | 123 | `a72cea4168980d2579e99a4a3c71d77041fa149e9d8d304f454cabbf72eda91f` | rds · xdr · v3 · bzip2 · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/named_list-xdr-v3-gzip.rds` | 107 | `074d7ae1c53cdc54e6a50dbfab4670028738bd4a713c65e5774a7391db3485e9` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/named_list-xdr-v3-none.rds` | 155 | `0fe7623a4e476e7f04515dc2136adb21583ce7e0d8f45500cdee0d549805565d` | rds · xdr · v3 · none · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/named_list-xdr-v3-xz.rds` | 148 | `5207053353614b50f17016a8b40b5cb1971cbe95003b67f3c3c4531e542eeaa6` | rds · xdr · v3 · xz · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/named_list-xdr-v3-zstd.rds` | 109 | `0beb712009d51e0eeee68408e0280f65b4178306d1bb64f09bc4e90d4333b65d` | rds · xdr · v3 · zstd · R 4.6.1 · enc UTF-8 | parse+convert OK (zstd decompressed first) |
| `zurds/named_list.rds` | 107 | `074d7ae1c53cdc54e6a50dbfab4670028738bd4a713c65e5774a7391db3485e9` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/namespace_closure.rds` | 443 | `db09c5f3420273cf125d7c8f6ab646a7e5723a7a86bc2e1bd87d8cb2a0c7f792` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parses; conversion fails: AssertionError() |
| `zurds/null.rds` | 44 | `738d47bdaeb4bef5f51573031df915d9b7dd2fb947066a6fd4c8e2da3dc9ab53` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/package_env.rds` | 67 | `580b33e50ea75d32a4839932138389ae80f4a80eae2003097074942e9f9792c6` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse FAILS: ValueError('248 is not a valid RObjectType') |
| `zurds/pairlist.rds` | 78 | `63eed793359862ce6f297341104b46b9080215f808bf23580546767f83379fd0` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/persistent.rds` | 63 | `367539dbd21c4e5f3abf2a43d1182d15b9f22c48dc55216b2148f7bc625b34e6` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse FAILS: ValueError('247 is not a valid RObjectType') |
| `zurds/promise.rds` | 137 | `80feb3ecca67217d35a96f9cf26bf11b495f375fcdcf86f7e4ff5d6699592691` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse FAILS: ValueError('252 is not a valid RObjectType') |
| `zurds/raw.rds` | 48 | `933736961f0da89e48496be93ed2b91421df9d8a8741ac4af5cf5957e18f8cb4` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse FAILS: NotImplementedError('Type RObjectType.RAW not implemented') |
| `zurds/s4.rds` | 125 | `ee8e1b922d520d3b1824684da0259528fac3d708ce865066c10ccbbe211ddf51` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/special.rds` | 49 | `eaead450f61615e3942165e3706f8e8df402bc24c42d8627b8894a1466f8cc43` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/symbol.rds` | 53 | `e8a8316732033eb8b7eb7e4b242487483753bd0ca57bb42b20e5164a01887a13` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse+convert OK |
| `zurds/zlib.rds` | 23783 | `f4b1f831c61ab1be7c3d7eee5b91e5e48cc817bd0fa8464a530e61ea0b283b17` | rds · xdr · v3 · gzip · R 4.6.1 · enc UTF-8 | parse FAILS: NotImplementedError('Type RObjectType.RAW not implemented') |
| `seurat/v5/seu_split_joined_data.csv` | 3883 | `6021637b05af7801d292e4c3c96c4a9fd75866c5124b845ff4d66b5637f32b9f` | CSV ground truth written by R `write.csv` | — |
| `seurat/v5/seu_split_diff_features_joined_counts.csv` | 657 | `2eea865e889ee53e45eb1f0ee24b1254fb52e15b6c2adb804b2c1ff88fb05b75` | CSV ground truth written by R `write.csv` | — |
