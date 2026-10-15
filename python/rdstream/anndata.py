"""Seurat and SingleCellExperiment objects as AnnData, without an R runtime.

>>> from rdstream.anndata import read
>>> adata = read("pbmc.rds")                     # Seurat or SingleCellExperiment
>>> adata = read("GSE.Robj.gz", name="RefMerge", assay="RNA", X="counts", layers=())

R stores expression matrices as features × cells; AnnData wants cells ×
features. Sparse ``dgCMatrix`` matrices are transposed for free (the CSC
buffers are reused as CSR), so the matrices are never copied after parsing.

What is read, and where it goes:

=====================  ==========================================  ==============================
AnnData                Seurat (v3/v4 ``Assay`` and v5 ``Assay5``)  SingleCellExperiment
=====================  ==========================================  ==============================
``X``, ``layers``      the assay's ``counts``/``data``/            ``assays(sce)``
                       ``scale.data`` (v5: layers, split layers
                       such as ``counts.1``, ``counts.2`` joined)
``obs``                ``meta.data`` and ``active.ident``          ``colData(sce)``
``var``                ``meta.features`` (v5: feature
                       ``meta.data``) and ``highly_variable``      ``rowData(sce)``
``obsm["X_<name>"]``   reductions' ``cell.embeddings``             ``reducedDims(sce)``
``obsp[<name>]``       ``graphs`` (with ``obsp=True``)             —
=====================  ==========================================  ==============================

Only what is needed is loaded: the read policy keeps assay matrices, cell and
feature metadata and embeddings, and skips everything else larger than
``threshold`` elements (command history, images, other slots). Seurat v3
``scale.data`` and dense v5 layers are only loaded when ``scale.data`` is
requested, because list element names are only known after their contents.
"""

from __future__ import annotations

import warnings
from typing import TYPE_CHECKING, Any, Callable, Sequence

import numpy as np

from ._objects import RObject
from .convert import as_array, as_pandas, as_sparse, dimnames, is_sparse, row_names

if TYPE_CHECKING:
    import anndata as ad
    import pandas as pd

__all__ = ["read", "read_sce", "read_seurat", "sce_to_anndata", "seurat_to_anndata", "to_anndata"]

SEURAT = ("Seurat",)
SCE = ("SingleCellExperiment", "SummarizedExperiment", "RangedSummarizedExperiment")


def _matrix(obj: RObject) -> Any:
    """An R features × cells matrix as cells × features (CSR when sparse)."""
    if is_sparse(obj):
        return as_sparse(obj, transpose=True).tocsr()
    if obj.type == "S4" and "x" not in obj.attributes:
        raise TypeError(f"unsupported matrix class {obj.classes} (e.g. DelayedArray); realize it in R first")
    return np.ascontiguousarray(as_array(obj).T)


def _shape(obj: RObject) -> tuple[int, int] | None:
    d = obj.attr("Dim")
    if d is not None and d.loaded:
        return int(d.value[0]), int(d.value[1])
    dim = obj.dim
    return (dim[0], dim[1]) if dim is not None and len(dim) == 2 else None


def _chr(obj: RObject | None) -> Any:
    return obj.value[0] if obj is not None and obj.type == "chr" and obj.loaded and obj.value else None


def _index(names: Sequence[str | None] | None, n: int) -> list[str]:
    if names is None or len(names) != n:
        return [str(k) for k in range(n)]
    return [str(k) if s is None else s for k, s in enumerate(names)]


def _frame(obj: RObject | None, index: list[str]) -> "pd.DataFrame":
    import pandas as pd

    if obj is None:
        return pd.DataFrame(index=pd.Index(index, dtype=object))
    df = as_pandas(obj)
    if len(df) != len(index):
        warnings.warn(f"metadata has {len(df)} rows for {len(index)} observations/features; dropping it", stacklevel=3)
        return pd.DataFrame(index=pd.Index(index, dtype=object))
    df.index = pd.Index(index, dtype=object)
    return df


def _pick(doc_objects: dict[str, RObject], name: str | None, classes: Sequence[str]) -> RObject:
    if name is not None:
        return doc_objects[name]
    for obj in doc_objects.values():
        if obj.inherits(*classes):
            return obj
    found = {k or "<root>": v.classes for k, v in doc_objects.items()}
    raise TypeError(f"no {' / '.join(classes)} object found (objects: {found})")


# --------------------------------------------------------------------------- Seurat


def _seurat_layers(assay: RObject, needed: set[str]) -> tuple[dict[str, Any], list[str], list[str]]:
    """Needed layers as name -> (cells × features matrix, feature indices, cell
    indices), plus all feature and cell names of the assay."""
    out: dict[str, Any] = {}
    if assay.attr("layers") is not None:  # Assay5: layers may cover subsets, recorded in LogMaps
        fmap, cmap = assay.attr("features"), assay.attr("cells")
        features, fcols = (list(n or []) for n in dimnames(fmap))
        cells, ccols = (list(n or []) for n in dimnames(cmap))
        fmask, cmask = as_array(fmap), as_array(cmap)
        for lname, m in assay.attr("layers").items():
            if lname is None or lname.split(".")[0] not in needed and lname not in needed:
                continue
            fsel = np.flatnonzero(fmask[:, fcols.index(lname)]) if lname in fcols else np.arange(len(features))
            csel = np.flatnonzero(cmask[:, ccols.index(lname)]) if lname in ccols else np.arange(len(cells))
            out[lname] = (_matrix(m), fsel, csel)
        return out, features, cells
    # Assay (v3/v4): counts, data and scale.data slots.
    ref = assay.attr("counts")
    if _shape(ref) in (None, (0, 0)):
        ref = assay.attr("data")
    shape = _shape(ref)
    rows, cols = dimnames(ref)
    features, cells = _index(rows, shape[0]), _index(cols, shape[1])
    pos = {f: k for k, f in enumerate(features)}
    for lname in ("counts", "data", "scale.data"):
        m = assay.attr(lname)
        s = _shape(m) if m is not None else None
        if lname not in needed or s is None or 0 in s:
            continue
        r, _ = dimnames(m)
        if s == shape:
            fsel = np.arange(shape[0])
        elif r is not None and all(f in pos for f in r):  # scale.data: usually the variable features
            fsel = np.array([pos[f] for f in r])
        else:
            warnings.warn(f"skipping {lname!r}: shape {s} does not match {shape}", stacklevel=3)
            continue
        out[lname] = (_matrix(m), fsel, np.arange(shape[1]))
    return out, features, cells


def _assemble(parts: list[tuple[Any, np.ndarray, np.ndarray]], n_obs: int, n_vars: int) -> Any:
    """Place (matrix, feature indices, cell indices) blocks into one n_obs × n_vars matrix."""
    import scipy.sparse as sp

    if len(parts) == 1:
        m, fsel, csel = parts[0]
        if len(fsel) == n_vars and len(csel) == n_obs and (fsel == np.arange(n_vars)).all() and (
            csel == np.arange(n_obs)
        ).all():
            return m
    if all(sp.issparse(m) for m, _, _ in parts):
        rows, cols, vals = [], [], []
        for m, fsel, csel in parts:
            c = m.tocoo()
            rows.append(csel[c.row])
            cols.append(fsel[c.col])
            vals.append(c.data)
        dtype = np.result_type(*[v.dtype for v in vals])
        return sp.csr_matrix(
            (np.concatenate(vals).astype(dtype, copy=False), (np.concatenate(rows), np.concatenate(cols))),
            shape=(n_obs, n_vars),
        )
    out = np.full((n_obs, n_vars), np.nan)
    for m, fsel, csel in parts:
        out[np.ix_(csel, fsel)] = m.toarray() if sp.issparse(m) else m
    return out


def _layer(layers: dict[str, Any], name: str, n_obs: int, n_vars: int) -> Any:
    """Layer ``name``, joined from its split parts (``name.<group>``) if needed."""
    names = [name] if name in layers else [k for k in layers if k.startswith(name + ".")]
    if not names:
        raise KeyError(f"layer {name!r} not found (available: {sorted(layers)})")
    cells = np.concatenate([layers[k][2] for k in names])
    if len(np.unique(cells)) != len(cells):
        raise ValueError(f"layers {names} share cells, so they cannot be joined; pick one, e.g. X={names[0]!r}")
    return _assemble([layers[k] for k in names], n_obs, n_vars)


def seurat_to_anndata(
    obj: RObject,
    *,
    assay: str | None = None,
    X: str = "data",
    layers: Sequence[str] = ("counts",),
    obsm: bool = True,
    obsp: bool = False,
) -> "ad.AnnData":
    """Convert a parsed Seurat object; see the module documentation. ``X``
    falls back to ``counts`` when the assay has no ``data`` yet."""
    import anndata as ad

    layers = [layers] if isinstance(layers, str) else list(layers)
    assays = obj.attr("assays")
    assay = assay or _chr(obj.attr("active.assay")) or assays.names[0]
    a = assays.get(assay)
    if a is None:
        raise KeyError(f"assay {assay!r} not found (available: {assays.names})")
    mats, features, cells = _seurat_layers(a, {X, "counts", *layers})
    meta = obj.attr("meta.data")
    if not cells:
        cells = row_names(meta)
    n_obs, n_vars = len(cells), len(features)
    available = {k.split(".")[0] for k in mats} | set(mats)
    if X == "data" and "data" not in available and "counts" in available:
        X = "counts"
    adata = ad.AnnData(
        X=_layer(mats, X, n_obs, n_vars),
        obs=_frame(meta, cells),
        var=_frame(a.attr("meta.features") or a.attr("meta.data"), features),
    )
    for lname in layers:
        if lname != X and lname in available:
            adata.layers[lname] = _layer(mats, lname, n_obs, n_vars)
    ident = obj.attr("active.ident")
    if ident is not None and ident.inherits("factor") and ident.loaded and len(ident) == n_obs:
        if "active.ident" not in adata.obs:
            adata.obs["active.ident"] = as_pandas(ident).to_numpy()
    vf = a.attr("var.features")
    if vf is not None and vf.type == "chr" and vf.loaded and vf.value:
        adata.var["highly_variable"] = adata.var_names.isin(vf.value)
    if obsm and obj.attr("reductions") is not None:
        for rname, red in obj.attr("reductions").items():
            emb = red.attr("cell.embeddings")
            if rname is None or emb is None or not emb.loaded or (_shape(emb) or (0,))[0] != n_obs:
                continue
            m = as_array(emb)
            emb_cells, _ = dimnames(emb)
            if emb_cells is not None and list(emb_cells) != cells:
                pos = {c: k for k, c in enumerate(emb_cells)}
                if not all(c in pos for c in cells):
                    warnings.warn(f"skipping reduction {rname!r}: its cells differ from the assay's", stacklevel=2)
                    continue
                m = m[[pos[c] for c in cells]]
            adata.obsm[f"X_{rname.lower()}"] = np.ascontiguousarray(m)
    if obsp and obj.attr("graphs") is not None:
        for gname, g in obj.attr("graphs").items():
            x = g.attr("x")
            if gname is not None and is_sparse(g) and x is not None and x.loaded and _shape(g) == (n_obs, n_obs):
                adata.obsp[gname] = as_sparse(g).tocsr()
    adata.uns["seurat"] = {"assay": assay, "X": X, "project": _chr(obj.attr("project.name")) or ""}
    return adata


# --------------------------------------------------------------------------- SingleCellExperiment


def _row_names(obj: RObject) -> list[str | None] | None:
    names = obj.attr("NAMES")
    if names is not None and names.type == "chr":
        return names.value
    rr = obj.attr("rowRanges")
    if rr is not None:
        for holder in (rr.attr("partitioning"), rr.attr("ranges"), rr):
            n = holder.attr("NAMES") if holder is not None else None
            if n is not None and n.type == "chr":
                return n.value
    return None


def sce_to_anndata(
    obj: RObject, *, X: str | None = None, layers: Sequence[str] | None = None, obsm: bool = True
) -> "ad.AnnData":
    """Convert a parsed SingleCellExperiment (or SummarizedExperiment); see the
    module documentation. ``X`` defaults to ``logcounts`` if present, else the
    first assay; ``layers`` defaults to all other assays."""
    import anndata as ad

    assays = dict(obj.attr("assays").attr("data").attr("listData").items())
    if not assays:
        raise ValueError("the object has no assays")
    X = X or ("logcounts" if "logcounts" in assays else next(iter(assays)))
    if X not in assays:
        raise KeyError(f"assay {X!r} not found (available: {list(assays)})")
    layers = [k for k in assays if k != X] if layers is None else [layers] if isinstance(layers, str) else layers
    x = assays[X]
    n_vars, n_obs = _shape(x)
    rows, cols = dimnames(x)
    col_data = obj.attr("colData")
    rn = col_data.attr("rownames") if col_data is not None else None
    cells = _index(rn.value if rn is not None and rn.type == "chr" and rn.loaded else cols, n_obs)
    features = _index(_row_names(obj) or rows, n_vars)
    rr = obj.attr("rowRanges")
    row_data = rr.attr("elementMetadata") if rr is not None else obj.attr("elementMetadata")
    adata = ad.AnnData(X=_matrix(x), obs=_frame(col_data, cells), var=_frame(row_data, features))
    for lname in layers:
        if lname in assays and lname != X:
            try:
                adata.layers[lname] = _matrix(assays[lname])
            except (TypeError, ValueError) as e:
                warnings.warn(f"skipping assay {lname!r}: {e}", stacklevel=2)
    int_col = obj.attr("int_colData")
    rd = int_col.attr("listData").get("reducedDims") if obsm and int_col is not None else None
    if rd is not None and rd.attr("listData") is not None:
        for rname, emb in rd.attr("listData").items():
            if rname is not None and emb.loaded and _shape(emb) is not None and _shape(emb)[0] == n_obs:
                adata.obsm[f"X_{rname.lower()}"] = np.ascontiguousarray(as_array(emb))
    return adata


# --------------------------------------------------------------------------- reading


def to_anndata(obj: RObject, **kwargs: Any) -> "ad.AnnData":
    """Convert a parsed Seurat or SingleCellExperiment object."""
    if obj.inherits(*SEURAT):
        return seurat_to_anndata(obj, **kwargs)
    if obj.inherits(*SCE):
        return sce_to_anndata(obj, **kwargs)
    raise TypeError(f"cannot convert an object of class {obj.classes} to AnnData")


def _policy(threshold: int, wanted: set[str], obsp: bool) -> Callable[[str, str, int], bool]:
    """Load assay matrices, metadata and embeddings; skip the rest above ``threshold``."""
    sce_keys = ("@colData", "@int_colData", "@rowRanges", "@elementMetadata", "@NAMES", "@assays@data")

    def keep(path: str, type: str, length: int) -> bool:
        if length <= threshold:
            return True
        if "@reductions" in path:
            return "@cell.embeddings" in path
        if "@graphs" in path:
            return obsp
        if "@assays[" in path:  # a Seurat assay: "...@assays[k]@<slot>..."
            slot = path.split("@assays[", 1)[1].split("@")[1:2]
            slot = slot[0] if slot else ""
            if slot in ("counts", "data", "scale.data"):
                return slot in wanted
            if slot.startswith("layers[") and path.endswith("]") and type == "dbl":
                return "scale.data" in wanted  # dense v5 layer; names come after the contents
            return True
        return "@meta.data" in path or any(k in path for k in sce_keys)

    return keep


def _read(source: Any, classes: Sequence[str], name: str | None, threshold: int, threads: int,
          memory_budget: int | None, spool_dir: Any, kwargs: dict[str, Any]) -> "ad.AnnData":
    from . import read as read_document

    layers = kwargs.get("layers")
    layers = ("counts",) if layers is None else [layers] if isinstance(layers, str) else layers
    wanted = {kwargs.get("X") or "data", "counts", *layers}
    doc = read_document(
        source,
        policy=_policy(threshold, wanted, bool(kwargs.get("obsp"))),
        threads=threads,
        memory_budget=memory_budget,
        spool_dir=spool_dir,
    )
    return to_anndata(_pick(doc.objects, name, classes), **kwargs)


def read(
    source: Any,
    *,
    name: str | None = None,
    threshold: int = 100_000,
    threads: int = 0,
    memory_budget: int | None = None,
    spool_dir: Any = None,
    **kwargs: Any,
) -> "ad.AnnData":
    """Read a Seurat or SingleCellExperiment object from an ``.rds``/``.RData``
    file (or its bytes) as AnnData. ``name`` selects an object in an ``.RData``
    file (default: the first convertible one); ``threshold`` is the largest
    vector loaded outside the parts listed in the module documentation. Other
    keyword arguments go to :func:`seurat_to_anndata` or :func:`sce_to_anndata`."""
    return _read(source, SEURAT + SCE, name, threshold, threads, memory_budget, spool_dir, kwargs)


def read_seurat(source: Any, *, name: str | None = None, threshold: int = 100_000, threads: int = 0,
                memory_budget: int | None = None, spool_dir: Any = None, **kwargs: Any) -> "ad.AnnData":
    """:func:`read` for a Seurat object."""
    return _read(source, SEURAT, name, threshold, threads, memory_budget, spool_dir, kwargs)


def read_sce(source: Any, *, name: str | None = None, threshold: int = 100_000, threads: int = 0,
             memory_budget: int | None = None, spool_dir: Any = None, **kwargs: Any) -> "ad.AnnData":
    """:func:`read` for a SingleCellExperiment or SummarizedExperiment."""
    return _read(source, SCE, name, threshold, threads, memory_budget, spool_dir, kwargs)
