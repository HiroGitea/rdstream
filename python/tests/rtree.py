"""Build R object trees the way rdstream parses them, for conversion tests."""

from __future__ import annotations

import numpy as np

from rdstream import RObject

NA = np.iinfo(np.int32).min


def chr(*values):
    return RObject("chr", list(values))


def ints(values, **attrs):
    return RObject("int", np.asarray(values, dtype=np.int32), _attrs(attrs))


def dbl(values, **attrs):
    return RObject("dbl", np.asarray(values, dtype=np.float64), _attrs(attrs))


def lgl(values, **attrs):
    return RObject("lgl", np.asarray(values, dtype=np.int32), _attrs(attrs))


def null():
    return RObject("NULL", None)


def _attrs(attrs):
    out = {}
    for k, v in attrs.items():
        k = {"class_": "class", "row_names": "row.names"}.get(k, k)
        out[k] = v if isinstance(v, RObject) else chr(*v) if isinstance(v, (list, tuple)) and v and isinstance(v[0], str) else v
    return out


def rlist(names=None, **attrs):
    """A named list from ``names`` = {name: RObject}."""
    names = names or {}
    a = _attrs(attrs)
    if names:
        a = {"names": chr(*names), **a}
    return RObject("list", list(names.values()), a, is_object="class" in a)


def factor(codes, levels):
    return ints(codes, levels=chr(*levels), class_=chr("factor"))


def data_frame(columns: dict, row_names=None):
    n = len(next(iter(columns.values())).value) if columns else 0
    rn = chr(*row_names) if row_names is not None else ints([NA, -n])
    return rlist(columns, class_=chr("data.frame"), row_names=rn)


def s4(cls, **slots):
    attrs = {k.replace("__", "."): v for k, v in slots.items()}
    attrs["class"] = RObject("chr", [cls], {"package": chr("pkg")})
    return RObject("S4", None, attrs, is_object=True)


def matrix(values, nrow, ncol, rows=None, cols=None, kind=dbl):
    a = np.asarray(values).reshape((nrow, ncol)).flatten(order="F")
    dn = rlist({}) if rows is None and cols is None else None
    attrs = {"dim": ints([nrow, ncol])}
    if rows is not None or cols is not None:
        attrs["dimnames"] = RObject("list", [chr(*rows) if rows else null(), chr(*cols) if cols else null()])
    return kind(a, **attrs)


def dgc(dense, rows=None, cols=None, cls="dgCMatrix"):
    """A column-compressed sparse matrix from a dense 2-D array."""
    dense = np.asarray(dense, dtype=np.float64)
    i, p, x = [], [0], []
    for c in range(dense.shape[1]):
        nz = np.flatnonzero(dense[:, c])
        i.extend(nz)
        x.extend(dense[nz, c])
        p.append(len(i))
    dimnames = RObject("list", [chr(*rows) if rows else null(), chr(*cols) if cols else null()])
    return s4(cls, i=ints(i), p=ints(p), Dim=ints(dense.shape), Dimnames=dimnames, x=dbl(x), factors=rlist())


def logmap(rows, cols, mask):
    m = np.asarray(mask, dtype=np.int32)
    attrs = {
        "dim": ints(m.shape),
        "dimnames": RObject("list", [chr(*rows), chr(*cols)]),
        "class": RObject("chr", ["LogMap"], {"package": chr("SeuratObject")}),
    }
    return RObject("lgl", m.flatten(order="F"), attrs, is_object=True)
