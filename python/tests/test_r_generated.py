"""Single-cell objects written by tests/r/generate.R in CI, checked against
what R reported about them. Skipped unless RDSTREAM_R_FIXTURES is set."""

import json
import os
from pathlib import Path

import numpy as np
import pytest

from rdstream.anndata import read

DIR = Path(os.environ.get("RDSTREAM_R_FIXTURES", "/nonexistent"))
pytestmark = pytest.mark.skipif(not (DIR / "expected_singlecell.json").exists(), reason="RDSTREAM_R_FIXTURES not set")


def expected():
    return json.loads((DIR / "expected_singlecell.json").read_text())


def dense(m):
    return m.toarray() if hasattr(m, "toarray") else np.asarray(m)


def check_matrix(facts, m, obs_names, var_names):
    """``facts`` describe an R features × cells matrix; ``m`` is cells × features."""
    m = dense(m)
    assert list(m.shape) == facts["dim"][::-1]
    assert list(var_names[:5]) == facts["rownames"] and list(obs_names[:5]) == facts["colnames"]
    np.testing.assert_allclose(m.sum(axis=1)[:5], [float(x) for x in facts["col_sums"]], rtol=1e-12)
    np.testing.assert_allclose(m.sum(), float(facts["total"]), rtol=1e-12)


@pytest.mark.parametrize("name", ["seurat_v3", "seurat_v5", "seurat_v5_split"])
def test_seurat(name):
    facts = expected().get(name)
    if facts is None:
        pytest.skip(f"{name} was not generated")
    adata = read(DIR / f"{name}.rds")
    assert list(adata.obs_names) == facts["cells"] and list(adata.var_names) == facts["features"]
    check_matrix(facts["data"], adata.X, adata.obs_names, adata.var_names)
    check_matrix(facts["counts"], adata.layers["counts"], adata.obs_names, adata.var_names)
    assert set(facts["meta_columns"]) <= set(adata.obs.columns)
    assert adata.obs["groups"].astype(str).tolist() == facts["groups"]
    for r in facts["reductions"]:
        assert f"X_{r.lower()}" in adata.obsm
    np.testing.assert_allclose(adata.obsm["X_pca"][0], [float(x) for x in facts["pca_first_row"]], rtol=1e-12)
    if name == "seurat_v3":
        assert set(adata.var_names[adata.var["highly_variable"]]) == set(facts["variable_features"])


def test_sce():
    facts = expected().get("sce")
    if facts is None:
        pytest.skip("sce was not generated")
    adata = read(DIR / "sce.rds")
    assert list(adata.obs_names) == facts["cells"] and list(adata.var_names) == facts["features"]
    check_matrix(facts["logcounts"], adata.X, adata.obs_names, adata.var_names)
    check_matrix(facts["counts"], adata.layers["counts"], adata.obs_names, adata.var_names)
    assert adata.obs["label"].astype(str).tolist() == facts["labels"]
    assert adata.var["symbol"].tolist()[:5] == facts["symbols"]
    np.testing.assert_allclose(adata.obsm["X_pca"][0], [float(x) for x in facts["pca_first_row"]], rtol=1e-12)
