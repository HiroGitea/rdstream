import numpy as np
import pandas as pd
import pytest

import rdstream
from rdstream import Skipped, as_array, as_pandas, as_sparse
from rdstream.convert import dimnames, row_names

from rtree import NA, chr, data_frame, dbl, dgc, factor, ints, lgl, matrix, rlist, s4


def test_data_frame_columns():
    df = data_frame(
        {
            "g": factor([1, 2, NA, 1], ["u", "v"]),
            "n": ints([1, NA, 3, 4]),
            "k": ints([1, 2, 3, 4]),
            "x": dbl([0.5, np.nan, 2, 3]),
            "b": lgl([1, 0, NA, 1]),
            "s": chr("a", None, "c", "d"),
            "d": dbl([0, 1, np.nan, 19000], class_=chr("Date")),
        }
    )
    out = as_pandas(df)
    assert list(out.index) == ["1", "2", "3", "4"]
    assert out["g"].dtype == "category" and list(out["g"].cat.categories) == ["u", "v"]
    assert out["g"].isna().tolist() == [False, False, True, False]
    assert str(out["n"].dtype) == "Int32" and out["n"].isna().sum() == 1
    assert out["k"].dtype == np.int32
    assert out["b"].dtype == "boolean" and out["b"].tolist()[:2] == [True, False]
    assert out["s"].isna().tolist() == [False, True, False, False]
    assert str(out["d"].iloc[1].date()) == "1970-01-02" and pd.isna(out["d"].iloc[2])


def test_row_names_forms():
    assert row_names(data_frame({"a": ints([1, 2])}, row_names=["x", "y"])) == ["x", "y"]
    assert row_names(data_frame({"a": ints([1, 2, 3])})) == ["1", "2", "3"]
    frame = s4("DFrame", rownames=rdstream.RObject("NULL", None), nrows=ints([2]), listData=rlist({"a": ints([1, 2])}))
    assert row_names(frame) == ["1", "2"]
    assert list(as_pandas(frame)["a"]) == [1, 2]


def test_matrix_and_dense_matrix_class():
    m = matrix([[1, 2, 3], [4, 5, 6]], 2, 3, rows=["r1", "r2"], cols=["a", "b", "c"])
    np.testing.assert_array_equal(as_array(m), [[1, 2, 3], [4, 5, 6]])
    assert dimnames(m) == (["r1", "r2"], ["a", "b", "c"])
    assert as_pandas(m).loc["r2", "b"] == 5
    dge = s4("dgeMatrix", x=dbl([1, 4, 2, 5]), Dim=ints([2, 2]))
    np.testing.assert_array_equal(as_array(dge), [[1, 2], [4, 5]])


def test_sparse_layouts():
    dense = np.array([[0, 1, 0], [2, 0, 3]], dtype=float)
    m = dgc(dense, rows=["g1", "g2"], cols=["c1", "c2", "c3"])
    np.testing.assert_array_equal(as_sparse(m).toarray(), dense)
    t = as_sparse(m, transpose=True)
    assert t.format == "csr"
    np.testing.assert_array_equal(t.toarray(), dense.T)
    # The transposed matrix reuses the parsed buffers.
    assert np.shares_memory(t.data, m.attr("x").value)
    trip = s4("dgTMatrix", i=ints([1, 0]), j=ints([0, 2]), Dim=ints([2, 3]), x=dbl([5, 6]))
    np.testing.assert_array_equal(as_sparse(trip).toarray(), [[0, 0, 6], [5, 0, 0]])
    sym = s4("dsCMatrix", i=ints([0, 0, 1]), p=ints([0, 1, 3]), Dim=ints([2, 2]), x=dbl([1, 2, 3]), uplo=chr("U"))
    np.testing.assert_array_equal(as_sparse(sym).toarray(), [[1, 2], [2, 3]])


def test_unloaded_vectors_raise():
    skipped = rdstream.RObject("dbl", Skipped("dbl", 10))
    with pytest.raises(ValueError, match="not loaded"):
        as_array(skipped)
    assert len(skipped) == 10 and not skipped.loaded
