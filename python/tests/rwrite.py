"""Serialize RObject trees in R's XDR format (no reference sharing), so tests
can round-trip synthetic objects through the parser."""

from __future__ import annotations

import struct

import numpy as np

from rdstream import RObject

CODES = {"NULL": 254, "sym": 1, "pairlist": 2, "lgl": 10, "int": 13, "dbl": 14, "cplx": 15, "chr": 16, "list": 19,
         "raw": 24, "S4": 25}


def _i(v: int) -> bytes:
    return struct.pack(">i", v)


def _chars(s: str | None) -> bytes:
    if s is None:
        return _i(9) + _i(-1)
    b = s.encode()
    return _i(9 | (1 << 3) << 12) + _i(len(b)) + b


def _item(o: RObject) -> bytes:
    if o.type == "NULL":
        return _i(254)
    code = CODES[o.type]
    attrs = o.attributes
    out = bytearray(_i(code | int(o.is_object) << 8 | int(bool(attrs)) << 9))
    v = o.value
    if o.type in ("int", "lgl"):
        out += _i(len(v)) + np.asarray(v, dtype=">i4").tobytes()
    elif o.type == "dbl":
        out += _i(len(v)) + np.asarray(v, dtype=">f8").tobytes()
    elif o.type == "raw":
        out += _i(len(v)) + np.asarray(v, dtype=np.uint8).tobytes()
    elif o.type == "chr":
        out += _i(len(v)) + b"".join(_chars(s) for s in v)
    elif o.type == "list":
        out += _i(len(v)) + b"".join(_item(x) for x in v)
    elif o.type != "S4":
        raise TypeError(o.type)
    if attrs:
        out += _pairlist(attrs.items())
    return bytes(out)


def _pairlist(items) -> bytes:
    out = bytearray()
    for k, v in items:
        out += _i(2 | 1 << 10) + _i(1) + _chars(k) + _item(v)
    return bytes(out + _i(254))


def rds(obj: RObject) -> bytes:
    return b"X\n" + _i(3) + _i(0x040301) + _i(0x030500) + _i(5) + b"UTF-8" + _item(obj)


def rdata(**objects: RObject) -> bytes:
    return b"RDX3\nX\n" + _i(3) + _i(0x040301) + _i(0x030500) + _i(5) + b"UTF-8" + _pairlist(objects.items())
