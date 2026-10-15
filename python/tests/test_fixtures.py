"""Seurat and SingleCellExperiment objects written by R, from tests/fixtures
(provenance and the R code behind each file: tests/fixtures/MANIFEST.md)."""

from pathlib import Path

import numpy as np
import pytest

import rdstream
from rdstream.anndata import read, read_sce, read_seurat

FIXTURES = Path(__file__).resolve().parents[2] / "tests" / "fixtures"


def dense(m):
    return m.toarray() if hasattr(m, "toarray") else np.asarray(m)


def csv(name):
    rows = (FIXTURES / "seurat" / "v5" / name).read_text().splitlines()
    cols = [c.strip('"') for c in rows[0].split(",")[1:]]
    genes = [r.split(",")[0].strip('"') for r in rows[1:]]
    values = np.array([[float(x) for x in r.split(",")[1:]] for r in rows[1:]])
    return genes, cols, values


def test_pbmc_small():
    adata = read_seurat(FIXTURES / "seurat" / "pbmc_small.rda", obsp=True)
    assert adata.shape == (80, 230)
    assert adata.obs_names[0] == "ATGCCAGAACGACT"
    counts = adata.layers["counts"]
    assert counts.nnz == 4456
    assert counts[0, list(adata.var_names).index("LTB")] == 3
    # Seurat's nCount_RNA is the column sum of counts.
    np.testing.assert_allclose(np.asarray(counts.sum(axis=1)).ravel(), adata.obs["nCount_RNA"])
    assert adata.obsm["X_pca"].shape == (80, 19)
    assert adata.obsm["X_tsne"].shape == (80, 2)
    assert adata.obsp["RNA_snn"].shape == (80, 80)
    assert adata.var["highly_variable"].sum() > 0
    assert adata.uns["seurat"]["X"] == "data"
    assert adata.obs["active.ident"].value_counts().to_dict() == {"0": 36, "1": 25, "2": 19}


def test_seurat_v5_split_layers_are_joined():
    adata = read(FIXTURES / "seurat" / "v5" / "seu_split.rds")
    genes, cells, data = csv("seu_split_joined_data.csv")
    assert list(adata.var_names) == genes and list(adata.obs_names) == cells
    np.testing.assert_allclose(dense(adata.X), data.T, rtol=1e-12)
    assert adata.obs["batch"].tolist()[:4] == ["B", "A", "B", "C"]


def test_seurat_v5_split_layers_with_different_features():
    adata = read(FIXTURES / "seurat" / "v5" / "seu_split_diff_features.rds")
    genes, cells, counts = csv("seu_split_diff_features_joined_counts.csv")
    assert list(adata.var_names) == genes and list(adata.obs_names) == cells
    np.testing.assert_array_equal(dense(adata.X), counts.T)


def test_seurat_v5_overlapping_layers_must_be_chosen():
    path = FIXTURES / "seurat" / "v5" / "seu_overlapping_layers.rds"
    with pytest.raises(ValueError, match="share cells"):
        read(path)
    raw = read(path, X="counts.raw", layers=())
    assert raw.shape == (12, 20) and raw.X.nnz == 216
    filtered = read(path, X="counts.filtered", layers=())
    assert filtered.X.nnz == 81
    assert dense(filtered.X)[6:].sum() == 0  # covers cells c1..c6 and genes g1..g15 only


def test_seurat_with_byte_compiled_closures():
    adata = read(FIXTURES / "seurat" / "scRep_example.rda")
    assert adata.shape == (500, 2000)
    assert "X_umap" in adata.obsm


def test_sce_with_reduced_dims():
    adata = read_sce(FIXTURES / "sce" / "mouseBrainSubsetSCE.rda")
    assert adata.shape == (30, 2314)
    np.testing.assert_allclose(dense(adata.X), np.log2(dense(adata.layers["counts"]) + 1))
    assert sorted(adata.obsm) == ["X_pca_counts", "X_pca_logcounts", "X_tsne_counts", "X_tsne_logcounts"]
    assert "tissue" in adata.obs


def test_sce_row_data():
    adata = read_sce(FIXTURES / "sce" / "scExample.rda", name="sce")
    assert adata.shape == (390, 200)
    assert list(adata.var.columns) == ["feature_ID", "feature_name"]


def test_sce_rds_and_h5ad_round_trip(tmp_path):
    adata = read(FIXTURES / "sce" / "simple_sce.rds")
    assert adata.shape == (100, 100)
    adata.write_h5ad(tmp_path / "x.h5ad")


def test_ascii_and_windows_files():
    doc = rdstream.read(FIXTURES / "rdata" / "legacy-1.0.0" / "test_ascii_win_v3.rds")
    assert doc.encoding == "ascii"
    assert doc.root[4].value == ["aä"]
