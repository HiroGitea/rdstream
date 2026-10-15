"""Python view of R objects as rdstream parses them."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Iterator

import numpy as np

# R's integer and logical NA.
NA_INTEGER = np.iinfo(np.int32).min


@dataclass(frozen=True, slots=True)
class Skipped:
    """A vector the read policy did not load; only its type and length are known."""

    type: str
    length: int


@dataclass(frozen=True, slots=True)
class Streamed:
    """A vector written to a sink while parsing (see ``rdstream.extract``)."""

    type: str
    length: int
    target: str


class RObject:
    """One node of an R object tree.

    ``type`` is the R type as ``rdstream describe`` prints it (``"int"``,
    ``"dbl"``, ``"lgl"``, ``"chr"``, ``"cplx"``, ``"raw"``, ``"list"``,
    ``"S4"``, ``"NULL"``, ``"pairlist"``, ``"env"``, ...). ``value`` is:

    - a 1-D NumPy array for ``int`` and ``lgl`` (int32, NA is ``NA_INTEGER``),
      ``dbl`` (float64, NA is a NaN), ``cplx`` (complex128) and ``raw`` (uint8);
    - a list of ``str`` or ``None`` (``NA``) for ``chr``;
    - a list of ``RObject`` for ``list`` and ``expr``;
    - a list of ``(tag, RObject)`` pairs for pairlists, calls and closures;
    - :class:`Skipped` or :class:`Streamed` for vectors that were not loaded;
    - ``None`` for ``NULL`` and S4 objects (whose slots are the attributes),
      the symbol name for ``sym``, an index into ``Document.environments``
      for ``env``, and a ``(class, state)`` pair for unknown ALTREP classes.

    ``attributes`` maps attribute (or S4 slot) names to ``RObject`` in stream order.
    """

    __slots__ = ("type", "value", "attributes", "is_object")

    def __init__(self, type: str, value: Any, attributes: dict[str, RObject] | None = None, is_object: bool = False):
        self.type = type
        self.value = value
        self.attributes = attributes if attributes is not None else {}
        self.is_object = is_object

    def __repr__(self) -> str:
        cls = self.classes
        tag = f" <{','.join(cls)}>" if cls else ""
        n = self.length
        size = f"[{n}]" if n is not None and self.type not in ("NULL", "S4") else ""
        if isinstance(self.value, (Skipped, Streamed)):
            size += f" ({type(self.value).__name__.lower()})"
        return f"RObject({self.type}{size}{tag})"

    @property
    def length(self) -> int | None:
        """Length of a vector, list or pairlist (``None`` for other types)."""
        v = self.value
        if isinstance(v, (Skipped, Streamed)):
            return v.length
        if self.type == "NULL":
            return 0
        if isinstance(v, (np.ndarray, list)):
            return len(v)
        return None

    def __bool__(self) -> bool:
        return True

    def __len__(self) -> int:
        n = self.length
        if n is None:
            raise TypeError(f"R {self.type} has no length")
        return n

    @property
    def loaded(self) -> bool:
        """False for vectors that were skipped or streamed instead of loaded."""
        return not isinstance(self.value, (Skipped, Streamed))

    def attr(self, name: str, default: Any = None) -> Any:
        """Attribute (or S4 slot) ``name``; R's pseudo-NULL slots come back as ``NULL`` objects."""
        return self.attributes.get(name, default)

    @property
    def classes(self) -> list[str]:
        """The ``class`` attribute."""
        c = self.attributes.get("class")
        return [s for s in c.value if s is not None] if c is not None and isinstance(c.value, list) else []

    def inherits(self, *classes: str) -> bool:
        return any(c in self.classes for c in classes)

    @property
    def names(self) -> list[str | None] | None:
        """The ``names`` attribute, or tags of a pairlist."""
        if self.type in ("pairlist", "call", "closure", "promise", "dots"):
            return [tag for tag, _ in self.value]
        n = self.attributes.get("names")
        return n.value if n is not None and isinstance(n.value, list) else None

    @property
    def dim(self) -> tuple[int, ...] | None:
        d = self.attributes.get("dim")
        return tuple(int(x) for x in d.value) if d is not None and isinstance(d.value, np.ndarray) else None

    def items(self) -> Iterator[tuple[str | None, RObject]]:
        """``(name, element)`` pairs of a list or pairlist."""
        if self.type in ("pairlist", "call", "closure", "promise", "dots"):
            yield from self.value
        elif isinstance(self.value, list) and self.type in ("list", "expr"):
            names = self.names or [None] * len(self.value)
            yield from zip(names, self.value)
        else:
            raise TypeError(f"R {self.type} has no elements")

    def get(self, name: str, default: Any = None) -> Any:
        """Element ``name`` of a list or pairlist."""
        for k, v in self.items():
            if k == name:
                return v
        return default

    def __getitem__(self, key: int | str) -> Any:
        """``obj[i]`` is the i-th list element (or vector value); ``obj["name"]``
        the named list element."""
        if isinstance(key, str):
            v = self.get(key, _MISSING)
            if v is _MISSING:
                raise KeyError(key)
            return v
        if self.type in ("pairlist", "call", "closure", "promise", "dots"):
            return self.value[key][1]
        if not self.loaded:
            raise ValueError(f"{self!r} was not loaded")
        return self.value[key]

    def __iter__(self) -> Iterator[Any]:
        for i in range(len(self)):
            yield self[i]

    def __contains__(self, name: object) -> bool:
        return name in (self.names or [])


_MISSING = object()


@dataclass(slots=True)
class Document:
    """A parsed ``.rds`` or ``.RData`` stream."""

    format: str
    """``"rds"`` (``saveRDS()``) or ``"rdata"`` (``save()``)."""
    encoding: str
    """``"xdr"``, ``"ascii"`` or ``"binary"``."""
    version: int
    writer_version: tuple[int, int, int]
    native_encoding: str | None
    root: RObject
    environments: list[dict[str, Any]] = field(default_factory=list)
    layers: list[dict[str, Any]] = field(default_factory=list)
    """Compression layers that were peeled, outermost first."""
    bytes_read: int = 0

    @property
    def objects(self) -> dict[str, RObject]:
        """Named objects of an ``.RData`` file, or ``{"": root}`` for ``.rds``."""
        if self.format == "rdata" and self.root.type == "pairlist":
            return {tag or "": v for tag, v in self.root.value}
        return {"": self.root}

    def __getitem__(self, name: str) -> RObject:
        return self.objects[name]
