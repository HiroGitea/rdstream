"""Seurat (v3 and v5) and SingleCellExperiment conversion on synthetic objects
with the slot layout of SeuratObject 5 and SingleCellExperiment 1.2x."""

import gzip
import warnings

import numpy as np
import pytest

from rdstream import RObject
from rdstream.anndata import read, read_sce, read_seurat, to_anndata

from rtree import chr, data_frame, dbl, dgc, factor, ints, lgl, logmap, matrix, null, rlist, s4
from rwrite import rdata, rds

GENES = ["g1", "g2", "g3"]
CELLS = ["c1", "c2", "c3", "c4"]
COUNTS = np.array([[1, 0, 2, 0], [0, 0, 3, 1], [5, 1, 0, 0]], dtype=float)  # genes x cells
DATA = np.log1p(COUNTS)
EMB = np.array([[0.1, 1.0], [0.2, 2.0], [0.3, 3.0], [0.4, 4.0]])  # cells x dims


def seurat(assay: RObject) -> RObject:
    meta = data_frame(
        {"nCount_RNA": dbl(COUNTS.sum(0)), "cluster": factor([1, 2, 1, 2], ["a", "b"])}, row_names=CELLS
    )
    pca = s4(
        "DimReduc",
        cell__embeddings=matrix(EMB, 4, 2, rows=CELLS, cols=["PC_1", "PC_2"]),
        feature__loadings=matrix(np.ones((3, 2)), 3, 2, rows=GENES, cols=["PC_1", "PC_2"]),
        assay__used=chr("RNA"),
        key=chr("PC_"),
    )
    graph = dgc(np.eye(4), CELLS, CELLS, cls="Graph")
    return s4(
        "Seurat",
        assays=rlist({"RNA": assay}),
        meta__data=meta,
        active__assay=chr("RNA"),
        active__ident=factor([1, 2, 1, 2], ["a", "b"]),
        graphs=rlist({"RNA_snn": graph}),
        reductions=rlist({"pca": pca}),
        project__name=chr("demo"),
        commands=rlist({"big": dbl(np.arange(1000))}),
    )


def assay_v3() -> RObject:
    scale = np.array([[1.0, -1, 0.5, 0], [0, 2, -2, 1]])  # variable features g1, g3
    return s4(
        "Assay",
        counts=dgc(COUNTS, GENES, CELLS),
        data=dgc(DATA, GENES, CELLS),
        scale__data=matrix(scale, 2, 4, rows=["g1", "g3"], cols=CELLS),
        key=chr("rna_"),
        assay__orig=null(),
        var__features=chr("g1", "g3"),
        meta__features=data_frame({"vst.mean": dbl([1.0, 2.0, 3.0])}, row_names=GENES),
        misc=null(),
    )


def assay_v5() -> RObject:
    names = ["counts.1", "counts.2", "data"]
    cells_mask = [[1, 0, 1], [1, 0, 1], [0, 1, 1], [0, 1, 1]]
    return s4(
        "Assay5",
        layers=rlist(
            {"counts.1": dgc(COUNTS[:, :2]), "counts.2": dgc(COUNTS[:, 2:]), "data": dgc(DATA)}
        ),
        cells=logmap(CELLS, names, cells_mask),
        features=logmap(GENES, names, np.ones((3, 3))),
        default=ints([1]),
        assay__orig=chr(),
        meta__data=data_frame({"vf_vst_counts_variable": lgl([1, 0, 1])}),
        misc=rlist(),
        key=chr("rna_"),
    )


def dframe(nrows, columns=None, rownames=None):
    return s4(
        "DFrame",
        rownames=chr(*rownames) if rownames else null(),
        nrows=ints([nrows]),
        listData=rlist(columns or {}),
        elementType=chr("ANY"),
        elementMetadata=null(),
        metadata=rlist(),
    )


def sce() -> RObject:
    reduced = dframe(4, {"PCA": matrix(EMB, 4, 2)})
    row_ranges = s4(
        "CompressedGRangesList",
        elementMetadata=dframe(3, {"symbol": chr("A", "B", "C")}),
        partitioning=s4("PartitioningByEnd", end=ints([0, 0, 0]), NAMES=chr(*GENES)),
    )
    return s4(
        "SingleCellExperiment",
        int_elementMetadata=dframe(3),
        int_colData=dframe(4, {"reducedDims": reduced, "altExps": dframe(4)}),
        int_metadata=rlist(),
        rowRanges=row_ranges,
        colData=dframe(4, {"label": factor([1, 1, 2, 2], ["x", "y"])}, rownames=CELLS),
        assays=s4(
            "SimpleAssays",
            data=s4("SimpleList", listData=rlist({"counts": matrix(COUNTS, 3, 4, kind=ints), "logcounts": dgc(DATA)})),
        ),
        NAMES=null(),
        elementMetadata=dframe(3),
        metadata=rlist(),
    )


def dense(m):
    return m.toarray() if hasattr(m, "toarray") else np.asarray(m)


def check_seurat(adata, obsp=False):
    assert list(adata.obs_names) == CELLS and list(adata.var_names) == GENES
    np.testing.assert_allclose(dense(adata.X), DATA.T)
    np.testing.assert_allclose(dense(adata.layers["counts"]), COUNTS.T)
    assert adata.obs["cluster"].tolist() == ["a", "b", "a", "b"]
    np.testing.assert_allclose(adata.obsm["X_pca"], EMB)
    assert ("RNA_snn" in adata.obsp) == obsp


def test_seurat_v3():
    adata = to_anndata(seurat(assay_v3()))
    check_seurat(adata)
    assert adata.X.format == "csr"
    assert adata.var["highly_variable"].tolist() == [True, False, True]
    assert adata.var["vst.mean"].tolist() == [1.0, 2.0, 3.0]
    assert adata.uns["seurat"] == {"assay": "RNA", "X": "data", "project": "demo"}
    scaled = to_anndata(seurat(assay_v3()), X="scale.data", layers=())
    np.testing.assert_allclose(scaled.X[:, 0], [1.0, -1, 0.5, 0])
    assert np.isnan(scaled.X[:, 1]).all()  # g2 is not a variable feature


def test_seurat_v5_joins_split_layers():
    adata = to_anndata(seurat(assay_v5()), obsp=True)
    check_seurat(adata, obsp=True)
    assert adata.var["vf_vst_counts_variable"].tolist() == [True, False, True]


def test_seurat_counts_only_falls_back_to_counts():
    a = assay_v3()
    a.attributes["data"] = dgc(np.zeros((0, 0)))
    adata = to_anndata(seurat(a), layers=())
    assert adata.uns["seurat"]["X"] == "counts"
    np.testing.assert_allclose(dense(adata.X), COUNTS.T)


def test_sce():
    adata = to_anndata(sce())
    assert list(adata.obs_names) == CELLS and list(adata.var_names) == GENES
    np.testing.assert_allclose(dense(adata.X), DATA.T)
    assert adata.layers["counts"].dtype == np.int32
    np.testing.assert_array_equal(adata.layers["counts"], COUNTS.T)
    assert adata.obs["label"].tolist() == ["x", "x", "y", "y"]
    assert adata.var["symbol"].tolist() == ["A", "B", "C"]
    np.testing.assert_allclose(adata.obsm["X_pca"], EMB)
    assert [k for k in to_anndata(sce(), X="counts", layers=()).layers if k is not None] == []


@pytest.mark.parametrize("version", ["v3", "v5"])
def test_read_seurat_end_to_end(tmp_path, version):
    obj = seurat(assay_v3() if version == "v3" else assay_v5())
    path = tmp_path / "obj.rds.gz"
    path.write_bytes(gzip.compress(rds(obj)))
    # A tiny threshold forces every large vector through the read policy.
    adata = read_seurat(path, threshold=2, obsp=True)
    check_seurat(adata, obsp=True)
    adata.write_h5ad(tmp_path / "out.h5ad")
    with warnings.catch_warnings():
        warnings.simplefilter("error")
        read(gzip.compress(rds(obj)), threshold=2, X="counts", layers=())


def test_read_policy_skips_unneeded_parts():
    from rdstream import read as read_document
    from rdstream.anndata import _policy

    doc = read_document(rds(seurat(assay_v3())), policy=_policy(2, {"data", "counts"}, False))
    s = doc.root
    assert not s.attr("commands")["big"].loaded
    assert not s.attr("graphs")["RNA_snn"].attr("x").loaded
    assert not s.attr("reductions")["pca"].attr("feature.loadings").loaded
    assert not s.attr("assays")["RNA"].attr("scale.data").loaded
    assert s.attr("assays")["RNA"].attr("counts").attr("x").loaded


def test_read_sce_from_rdata():
    data = rdata(other=dbl([1.0]), sce=sce())
    adata = read_sce(data, threshold=2)
    np.testing.assert_allclose(dense(adata.X), DATA.T)
    assert adata.obs["label"].tolist() == ["x", "x", "y", "y"]
    with pytest.raises(TypeError, match="no Seurat"):
        read_seurat(data)
    assert read(data, name="sce", threshold=2).n_obs == 4
