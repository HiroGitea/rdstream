"""Read R's ``.rds`` and ``.RData`` files in one streaming pass.

>>> import rdstream
>>> doc = rdstream.read("seurat.rds", keep=["*@meta.data*"], threshold=10_000)
>>> meta = rdstream.as_pandas(doc.root.attr("meta.data"))

Vectors larger than ``threshold`` elements are skipped unless a ``keep`` glob
matches their path; ``skip`` globs win over both. Paths are the ones
``rdstream.describe`` prints: ``$name`` for a tagged pairlist entry (an object
in an ``.RData`` file), ``@name`` for an attribute or S4 slot, ``[k]`` for a
list element. ``*`` matches any run of characters.

Converting Seurat and SingleCellExperiment objects to AnnData lives in
:mod:`rdstream.anndata`.
"""

from __future__ import annotations

import json
import os
from typing import Any, Callable, Sequence, Union

from . import _rdstream
from ._objects import NA_INTEGER, Document, RObject, Skipped, Streamed
from ._rdstream import ParseError
from .convert import as_array, as_pandas, as_sparse

__all__ = [
    "NA_INTEGER",
    "Document",
    "ParseError",
    "RObject",
    "Skipped",
    "Streamed",
    "as_array",
    "as_pandas",
    "as_sparse",
    "describe",
    "extract",
    "read",
]

Source = Union[str, "os.PathLike[str]", bytes]
"""A path (``"-"`` reads standard input) or the file's contents. Compression
(gzip, bzip2, xz, zstd, also nested) is detected and removed."""


def _bytes(source: Any) -> Any:
    return bytes(source) if isinstance(source, (bytearray, memoryview)) else source


def _path(p: Any) -> str | None:
    return None if p is None else os.fspath(p)


def read(
    source: Source,
    *,
    threshold: int | None = 1_000_000,
    keep: Sequence[str] = (),
    skip: Sequence[str] = (),
    policy: Callable[[str, str, int], bool] | None = None,
    threads: int = 0,
    memory_budget: int | None = None,
    spool_dir: str | os.PathLike[str] | None = None,
) -> Document:
    """Parse an R serialization stream.

    ``threshold`` is the largest vector (in elements) loaded without a matching
    ``keep`` glob; ``None`` loads everything. Alternatively ``policy`` is called
    as ``policy(path, type, length)`` before each vector's data is read and
    returns whether to load it (``threshold``, ``keep`` and ``skip`` are then
    ignored). ``threads`` and ``memory_budget`` size the gzip decoders as on the
    command line (``0`` = size from the budget, ``1`` = single-threaded).
    ``spool_dir`` copies a stream (standard input) to a temporary file there
    first, so that it decodes in parallel like a file. The GIL is released
    while parsing, except while ``policy`` runs.
    """
    fields = _rdstream.read(
        _bytes(source),
        threshold=threshold,
        keep=list(keep),
        skip=list(skip),
        policy=policy,
        threads=threads,
        memory_budget=memory_budget,
        spool_dir=_path(spool_dir),
    )
    return Document(**fields)


def describe(
    source: Source,
    *,
    depth: int = 5,
    threshold: int = 1_000_000,
    threads: int = 0,
    memory_budget: int | None = None,
    spool_dir: str | os.PathLike[str] | None = None,
) -> str:
    """The object tree with the paths that ``keep``/``skip`` globs match, as text."""
    return _rdstream.describe(
        _bytes(source),
        depth=depth,
        threshold=threshold,
        threads=threads,
        memory_budget=memory_budget,
        spool_dir=_path(spool_dir),
    )


def extract(
    source: Source,
    paths: Sequence[str],
    out: str | os.PathLike[str],
    *,
    threshold: int = 0,
    threads: int = 0,
    memory_budget: int | None = None,
    spool_dir: str | os.PathLike[str] | None = None,
) -> dict[str, Any]:
    """Stream the vectors under ``paths`` to ``.npy``/``.json`` files in ``out``
    without loading them; returns the parsed ``manifest.json``."""
    if isinstance(paths, str):
        paths = [paths]
    manifest = _rdstream.extract(
        _bytes(source),
        list(paths),
        os.fspath(out),
        threshold=threshold,
        threads=threads,
        memory_budget=memory_budget,
        spool_dir=_path(spool_dir),
    )
    return json.loads(manifest)
