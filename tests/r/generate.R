#!/usr/bin/env Rscript
# Write R-serialized fixtures for rdstream's tests, plus what R says they hold.
#
#   Rscript tests/r/generate.R <out-dir>
#
# Writes every object below in each serialization variant (saveRDS / save,
# versions 2 and 3, XDR / ASCII / hex ASCII / native binary, every compression
# including zstd where R supports it),
# and expected.json describing the objects as R sees them. With SeuratObject or
# SingleCellExperiment installed it also writes single-cell objects and
# expected_singlecell.json. tests/r_generated.rs and python/tests/test_r_generated.py
# read RDSTREAM_R_FIXTURES=<out-dir>.

suppressPackageStartupMessages(library(jsonlite))

args <- commandArgs(trailingOnly = TRUE)
out <- if (length(args) > 0) args[[1]] else "target/r-fixtures"
dir.create(out, recursive = TRUE, showWarnings = FALSE)

setClass("Thing", representation(x = "numeric", label = "character", extra = "ANY"))

latin1 <- c("caf\xe9", "na\xefve")
Encoding(latin1) <- "latin1"

env <- new.env()
assign("v", 1:3, envir = env)
assign("self", env, envir = env)

set.seed(1)
objects <- list(
  int = c(1L, NA, -3L, .Machine$integer.max),
  dbl = c(1.5, NA, NaN, Inf, -Inf, -0, 1e-300, .Machine$double.xmax, pi, 5e-324),
  lgl = c(TRUE, FALSE, NA),
  cplx = complex(real = c(1, NA), imaginary = c(-2.5, 0)),
  raw = as.raw(c(0, 1, 127, 128, 255)),
  chr = c("a", NA, "", "with space", "tab\tnewline\n", "quote\"'\\?", "ünïcödé",
          "日本語", strrep("x", 300)),
  latin1 = latin1,
  null = NULL,
  named = c(a = 1L, b = 2L),
  factor = factor(c("u", "v", NA, "u"), levels = c("v", "u")),
  ordered = factor(c("lo", "hi"), levels = c("lo", "hi"), ordered = TRUE),
  df = data.frame(n = 1:3, s = c("x", "y", NA), f = factor(c("a", "b", "a")), stringsAsFactors = FALSE),
  df_rownames = data.frame(v = c(0.5, 1.5), row.names = c("r1", "r2")),
  matrix = matrix(1:6, 2, 3, dimnames = list(c("a", "b"), NULL)),
  nested = list(1L, list(x = "deep", y = list(TRUE)), NULL),
  compact_seq = 1:100000,
  deferred = as.character(1:5),
  s4 = new("Thing", x = c(1, 2), label = "thing", extra = NULL),
  env = env,
  closure = function(x, y = 2) x + y,
  big_dbl = rnorm(2e6),
  big_chr = sprintf("cell_%06d", 1:200000),
  date = as.Date(c("2024-01-02", NA)),
  long_list = as.list(1:5000)
)

# ---------------------------------------------------------------- expected values

num <- function(v) sprintf("%.17g", v) # "NA", "NaN", "Inf" and "-Inf" come out as such
values <- function(x) {
  switch(typeof(x),
    integer = I(ifelse(is.na(x), "NA", sprintf("%d", x))),
    double = I(num(x)),
    logical = I(ifelse(is.na(x), "NA", ifelse(x, "TRUE", "FALSE"))),
    complex = lapply(seq_along(x), function(k) c(num(Re(x[k])), num(Im(x[k])))),
    raw = I(as.integer(x)),
    character = as.list(ifelse(is.na(x), NA, enc2utf8(x))),
    NULL
  )
}

describe <- function(x) {
  if (typeof(x) == "symbol" && identical(as.character(x), "\001NULL\001")) x <- NULL
  d <- list(type = typeof(x))
  if (is.environment(x) || is.function(x) || is.symbol(x) || is.language(x)) return(d)
  d$length <- length(x)
  v <- values(x)
  if (!is.null(v)) {
    if (length(x) <= 1000) d$values <- v else d$head <- if (is.list(v)) v[1:5] else I(v[1:5])
  }
  if (is.list(x) && !isS4(x)) {
    shown <- if (length(x) <= 100) seq_along(x) else 1:5
    d$elements <- unname(lapply(unclass(x)[shown], describe))
  }
  a <- attributes(x)
  if (length(a) > 0) d$attributes <- lapply(a, describe)
  d
}

# ---------------------------------------------------------------- serialization variants

# Objects are described only after saving: describing materializes ALTREP
# vectors, which R then serializes in full.
files <- list()
record <- function(file, format, encoding, version, compression) {
  files[[length(files) + 1]] <<- list(
    file = file, format = format, encoding = encoding, version = version, compression = compression
  )
}

for (version in c(2, 3)) {
  for (compress in c("none", "gzip", "bzip2", "xz")) {
    for (ascii in c(FALSE, TRUE)) {
      if (ascii && !(compress %in% c("none", "gzip"))) next
      enc <- if (ascii) "ascii" else "xdr"
      file <- sprintf("all.v%d.%s.%s.rds", version, enc, compress)
      saveRDS(objects, file.path(out, file), version = version, ascii = ascii,
              compress = if (compress == "none") FALSE else compress)
      record(file, "rds", enc, version, compress)
    }
  }
  file <- sprintf("all.v%d.binary.none.rds", version)
  con <- file(file.path(out, file), "wb")
  serialize(objects, con, xdr = FALSE, version = version)
  close(con)
  record(file, "rds", "binary", version, "none")

  for (ascii in list(FALSE, TRUE)) {
    enc <- if (isTRUE(ascii)) "ascii" else "xdr"
    file <- sprintf("all.v%d.%s.RData", version, enc)
    local({
      list2env(objects, environment())
      save(list = names(objects), file = file.path(out, file), version = version, ascii = ascii)
    })
    # save() compresses only binary output by default.
    record(file, "rdata", enc, version, if (enc == "xdr") "gzip" else "none")
  }

  # serialize() supports hexadecimal ASCII on R versions that reject save(ascii = NA).
  file <- sprintf("all.v%d.asciihex.RData", version)
  con <- file(file.path(out, file), "wb")
  writeBin(charToRaw(sprintf("RDA%d\n", version)), con)
  serialize(as.pairlist(objects), con, ascii = NA, version = version)
  close(con)
  stopifnot(identical(sort(load(file.path(out, file), envir = new.env())), sort(names(objects))))
  record(file, "rdata", "ascii", version, "none")
}

# zstd: R >= 4.5 built with zstd writes it through zstdfile() connections.
has_zstd <- exists("zstdfile") &&
  !inherits(try(memCompress(as.raw(1L), "zstd"), silent = TRUE), "try-error")
if (has_zstd) {
  for (version in c(2, 3)) {
    file <- sprintf("all.v%d.xdr.zstd.rds", version)
    con <- zstdfile(file.path(out, file), "wb")
    saveRDS(objects, con, version = version)
    close(con)
    record(file, "rds", "xdr", version, "zstd")
  }
  file <- "all.v3.xdr.zstd.RData"
  con <- zstdfile(file.path(out, file), "wb")
  local({
    list2env(objects, environment())
    save(list = names(objects), file = con, version = 3)
  })
  close(con)
  record(file, "rdata", "xdr", 3, "zstd")
} else {
  message("this R cannot write zstd; skipping zstd fixtures")
}

# GEO-style recompression: gzip over R's gzipped save().
file <- "all.v3.xdr.RData.gz"
local({
  list2env(objects, environment())
  save(list = names(objects), file = file.path(out, "tmp.RData"), version = 3)
})
raw <- readBin(file.path(out, "tmp.RData"), "raw", file.size(file.path(out, "tmp.RData")))
con <- gzfile(file.path(out, file), "wb")
writeBin(raw, con)
close(con)
unlink(file.path(out, "tmp.RData"))
record(file, "rdata", "xdr", 3, "gzip+gzip")

expected <- list(objects = lapply(objects, describe), files = files)
write_json(expected, file.path(out, "expected.json"), auto_unbox = TRUE, null = "null", na = "null", digits = NA,
           pretty = TRUE)

# ---------------------------------------------------------------- single-cell objects

singlecell <- list()

matrix_facts <- function(m) {
  m <- as.matrix(m)
  list(dim = I(dim(m)), rownames = I(head(rownames(m), 5)), colnames = I(head(colnames(m), 5)),
       col_sums = I(num(head(colSums(m), 5))), total = num(sum(m)))
}

if (requireNamespace("SeuratObject", quietly = TRUE)) {
  suppressPackageStartupMessages(library(SeuratObject))
  data("pbmc_small", package = "SeuratObject")
  pbmc <- suppressWarnings(UpdateSeuratObject(pbmc_small))
  saveRDS(pbmc, file.path(out, "seurat_v3.rds"))
  facts <- function(obj, assay = "RNA") {
    list(
      cells = I(colnames(obj)), features = I(rownames(obj)),
      counts = matrix_facts(LayerData(obj, assay = assay, layer = "counts")),
      data = matrix_facts(LayerData(obj, assay = assay, layer = "data")),
      meta_columns = I(colnames(obj[[]])),
      groups = I(as.character(obj$groups)),
      reductions = I(Reductions(obj)),
      pca_first_row = I(num(Embeddings(obj, "pca")[1, ])),
      variable_features = I(VariableFeatures(obj))
    )
  }
  singlecell$seurat_v3 <- facts(pbmc)
  tryCatch({
    pbmc5 <- pbmc
    pbmc5[["RNA"]] <- as(pbmc5[["RNA"]], "Assay5")
    saveRDS(pbmc5, file.path(out, "seurat_v5.rds"))
    singlecell$seurat_v5 <- facts(pbmc5)
    pbmc5[["RNA"]] <- split(pbmc5[["RNA"]], f = pbmc5$groups)
    saveRDS(pbmc5, file.path(out, "seurat_v5_split.rds"))
    singlecell$seurat_v5_split <- singlecell$seurat_v5
    singlecell$seurat_v5_split$layers <- I(Layers(pbmc5[["RNA"]]))
  }, error = function(e) message("skipping Seurat v5 fixtures: ", conditionMessage(e)))
}

if (requireNamespace("SingleCellExperiment", quietly = TRUE)) {
  suppressPackageStartupMessages(library(SingleCellExperiment))
  counts <- matrix(rpois(200 * 50, 2), 200, 50, dimnames = list(sprintf("gene%03d", 1:200), sprintf("cell%02d", 1:50)))
  sce <- SingleCellExperiment(
    assays = list(counts = counts, logcounts = log1p(counts)),
    colData = S4Vectors::DataFrame(label = factor(rep(c("a", "b"), 25)), depth = colSums(counts)),
    rowData = S4Vectors::DataFrame(symbol = sprintf("SYM%d", 1:200))
  )
  reducedDim(sce, "PCA") <- matrix(rnorm(50 * 3), 50, 3)
  counts(sce) <- Matrix::Matrix(counts(sce), sparse = TRUE)
  saveRDS(sce, file.path(out, "sce.rds"))
  singlecell$sce <- list(
    cells = I(colnames(sce)), features = I(rownames(sce)),
    counts = matrix_facts(counts(sce)), logcounts = matrix_facts(logcounts(sce)),
    labels = I(as.character(sce$label)), symbols = I(head(rowData(sce)$symbol, 5)),
    pca_first_row = I(num(reducedDim(sce, "PCA")[1, ]))
  )
}

write_json(singlecell, file.path(out, "expected_singlecell.json"), auto_unbox = TRUE, digits = NA, pretty = TRUE)
cat("wrote", length(files), "serialization variants and", length(singlecell), "single-cell objects to", out, "\n")
