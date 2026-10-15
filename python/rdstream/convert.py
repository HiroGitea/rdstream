"""Convert R objects to NumPy arrays, pandas objects and SciPy sparse matrices.

pandas and SciPy are imported only by the functions that need them.
"""

from __future__ import annotations

import warnings
from typing import TYPE_CHECKING, Any

import numpy as np

from ._objects import NA_INTEGER, RObject

if TYPE_CHECKING:
    import pandas as pd
    import scipy.sparse as sp

__all__ = ["as_array", "as_pandas", "as_sparse", "dimnames", "is_sparse", "row_names"]

SPARSE_CLASSES = {
    f"{kind}{layout}Matrix" for kind in ("dg", "lg", "ng", "ds", "ls", "ns") for layout in ("C", "R", "T")
}


def _loaded(obj: RObject) -> RObject:
    if not obj.loaded:
        raise ValueError(f"{obj!r} was not loaded; adjust threshold/keep when reading")
    return obj


def as_array(obj: RObject) -> np.ndarray:
    """A vector, matrix or array as NumPy, shaped by its ``dim`` attribute.

    Integers keep R's ``NA`` as ``NA_INTEGER``; logical vectors become ``bool``
    unless they contain ``NA`` (then ``float64`` with ``NaN``); character
    vectors become object arrays with ``None`` for ``NA``. Dense ``Matrix``
    objects (``dgeMatrix`` and friends) are supported too.
    """
    if obj.type == "S4" and "x" in obj.attributes and "Dim" in obj.attributes:
        if is_sparse(obj):
            raise TypeError(f"{obj!r} is sparse; use as_sparse()")
        dim = tuple(int(d) for d in _loaded(obj.attr("Dim")).value)
        return as_array(obj.attr("x")).reshape(dim, order="F")
    _loaded(obj)
    v = obj.value
    if obj.type == "lgl":
        na = v == NA_INTEGER
        v = np.where(na, np.nan, v.astype(np.float64)) if na.any() else v.astype(bool)
    elif obj.type == "chr":
        v = np.array(v, dtype=object)
    elif not isinstance(v, np.ndarray):
        raise TypeError(f"{obj!r} is not an atomic vector")
    dim = obj.dim
    return v.reshape(dim, order="F") if dim is not None and len(dim) > 1 else v


def is_sparse(obj: RObject) -> bool:
    """Whether ``obj`` is a sparse ``Matrix`` (including subclasses such as Seurat's ``Graph``)."""
    return obj.type == "S4" and "p" in obj.attributes and "Dim" in obj.attributes or bool(
        SPARSE_CLASSES.intersection(obj.classes)
    )


def as_sparse(obj: RObject, *, transpose: bool = False) -> "sp.csc_matrix | sp.csr_matrix | sp.coo_matrix":
    """A ``Matrix`` sparse matrix (``dgCMatrix``, ``dgRMatrix``, ``dgTMatrix`` and
    their logical/pattern/symmetric variants) as a SciPy sparse matrix sharing
    its buffers. With ``transpose=True`` the result is transposed, which for the
    common column-compressed case is a free ``csc`` → ``csr`` relabelling.

    Symmetric variants (``dsCMatrix``, ...) store one triangle; it is mirrored.
    """
    import scipy.sparse as sp

    nrow, ncol = (int(d) for d in _loaded(obj.attr("Dim")).value)
    x = obj.attr("x")
    if x is None:  # pattern matrix
        nnz = len(obj.attr("i") if obj.attr("i") is not None else obj.attr("j"))
        data = np.ones(nnz, dtype=bool)
    else:
        data = as_array(x)
    shape = (ncol, nrow) if transpose else (nrow, ncol)
    if "p" in obj.attributes and "i" in obj.attributes:  # column-compressed
        i, p = as_array(obj.attr("i")), as_array(obj.attr("p"))
        m = sp.csr_matrix((data, i, p), shape=shape) if transpose else sp.csc_matrix((data, i, p), shape=shape)
    elif "p" in obj.attributes:  # row-compressed
        j, p = as_array(obj.attr("j")), as_array(obj.attr("p"))
        m = sp.csc_matrix((data, j, p), shape=shape) if transpose else sp.csr_matrix((data, j, p), shape=shape)
    else:  # triplet
        i, j = as_array(obj.attr("i")), as_array(obj.attr("j"))
        m = sp.coo_matrix((data, (j, i) if transpose else (i, j)), shape=shape)
    uplo = obj.attr("uplo")
    if uplo is not None and obj.classes and obj.classes[0][1] == "s":
        tri = m.tocoo()
        off = tri.row != tri.col
        mirrored = sp.coo_matrix((tri.data[off], (tri.col[off], tri.row[off])), shape=shape, dtype=tri.dtype)
        m = (tri + mirrored).asformat(m.format)
    return m


def dimnames(obj: RObject) -> tuple[list[str | None] | None, list[str | None] | None]:
    """Row and column names of a base matrix (``dimnames``) or ``Matrix`` (``@Dimnames``)."""
    dn = obj.attr("dimnames") or obj.attr("Dimnames")
    if dn is None or dn.type != "list":
        return None, None
    names = [d.value if d.type == "chr" and d.loaded else None for d in dn.value]
    names += [None] * (2 - len(names))
    return names[0], names[1]


def row_names(obj: RObject) -> list[str]:
    """Row names of a ``data.frame`` or S4Vectors ``DataFrame`` as R reports
    them: automatic row names become ``"1"``, ``"2"``, ..."""
    if obj.type == "S4":
        rn = obj.attr("rownames")
        if rn is not None and rn.type == "chr":
            return list(_loaded(rn).value)
        return [str(k) for k in range(1, int(_loaded(obj.attr("nrows")).value[0]) + 1)]
    rn = obj.attr("row.names")
    if rn is None:
        n = len(obj.value[0]) if obj.value else 0
        return [str(k) for k in range(1, n + 1)]
    v = _loaded(rn).value
    if rn.type == "chr":
        return list(v)
    if len(v) == 2 and v[0] == NA_INTEGER:  # compact form c(NA, -n)
        return [str(k) for k in range(1, abs(int(v[1])) + 1)]
    return [str(int(k)) for k in v]


def _column(obj: RObject, n: int | None = None) -> Any:
    """One column / vector as a pandas-compatible array."""
    import pandas as pd

    _loaded(obj)
    v = obj.value
    if obj.inherits("factor"):
        levels = obj.attr("levels").value
        codes = np.where(v == NA_INTEGER, -1, v - 1)
        return pd.Categorical.from_codes(codes, categories=levels, ordered=obj.inherits("ordered"))
    if obj.inherits("Date") and obj.type in ("dbl", "int"):
        days = np.asarray(v, dtype=np.float64)
        out = np.full(days.shape, np.datetime64("NaT", "D"))
        ok = np.isfinite(days)
        out[ok] = days[ok].astype("int64").astype("datetime64[D]")
        return out
    if obj.inherits("POSIXct") and obj.type in ("dbl", "int"):
        secs = np.asarray(v, dtype=np.float64)
        tz = obj.attr("tzone")
        tz = tz.value[0] if tz is not None and tz.type == "chr" and tz.value and tz.value[0] else None
        ts = pd.to_datetime(secs, unit="s", utc=True)
        return ts.tz_convert(tz) if tz else ts.tz_localize(None)
    if obj.type == "int":
        na = v == NA_INTEGER
        return pd.arrays.IntegerArray(v, na) if na.any() else v
    if obj.type == "lgl":
        na = v == NA_INTEGER
        return pd.arrays.BooleanArray(v != 0, na) if na.any() else v != 0
    if obj.type == "chr":
        return np.array(v, dtype=object)
    if obj.type in ("dbl", "cplx", "raw"):
        return v
    raise TypeError(f"cannot convert {obj!r} to a column")


def _frame(columns: list[tuple[str, RObject]], index: list[str]) -> "pd.DataFrame":
    import pandas as pd

    data: dict[str, Any] = {}
    for name, col in columns:
        if col.inherits("data.frame") or col.inherits("DFrame"):
            nested = as_pandas(col)
            for c in nested.columns:
                data[f"{name}.{c}"] = nested[c].to_numpy()
            continue
        dim = col.dim
        if dim is not None and len(dim) == 2 and col.loaded:  # matrix column: one column per matrix column
            m = as_array(col)
            for k in range(m.shape[1]):
                data[f"{name}.{k + 1}"] = m[:, k]
            continue
        try:
            data[name] = _column(col)
        except (TypeError, ValueError) as e:
            warnings.warn(f"skipping column {name!r}: {e}", stacklevel=3)
    return pd.DataFrame(data, index=pd.Index(index, dtype=object))


def as_pandas(obj: RObject) -> "pd.DataFrame | pd.Series":
    """A ``data.frame`` or S4Vectors ``DataFrame`` as a pandas ``DataFrame``,
    a matrix as a ``DataFrame`` labelled by its dimnames, or a vector as a
    ``Series`` (factors become categoricals, ``Date``/``POSIXct`` datetimes,
    integers and logicals with ``NA`` nullable arrays).

    Nested data frames and matrix columns are flattened to ``"name.col"``;
    columns that cannot be converted are dropped with a warning.
    """
    import pandas as pd

    if obj.inherits("data.frame"):
        names = obj.names or []
        cols = [(n or f"V{k + 1}", c) for k, (n, c) in enumerate(zip(names, obj.value))]
        return _frame(cols, row_names(obj))
    if obj.type == "S4" and obj.attr("listData") is not None and obj.attr("nrows") is not None:
        cols = [(n or f"V{k + 1}", c) for k, (n, c) in enumerate(obj.attr("listData").items())]
        return _frame(cols, row_names(obj))
    dim = obj.dim
    if dim is not None and len(dim) == 2:
        rows, cols = dimnames(obj)
        return pd.DataFrame(as_array(obj), index=rows, columns=cols)
    return pd.Series(_column(obj), index=obj.names)
