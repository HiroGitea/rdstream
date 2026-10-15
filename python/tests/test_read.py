import gzip

import numpy as np
import pytest

import rdstream

from rtree import NA, chr, data_frame, dbl, factor, ints, rlist
from rwrite import rdata, rds


def test_round_trip_and_selection():
    obj = rlist({"big": dbl(np.arange(1000)), "small": ints([1, NA]), "s": chr("a", None)})
    data = gzip.compress(rds(obj))
    doc = rdstream.read(data)
    assert (doc.format, doc.encoding, doc.layers[0]["compression"]) == ("rds", "xdr", "gzip")
    np.testing.assert_array_equal(doc.root["big"].value, np.arange(1000))
    assert doc.root["small"].value.dtype == np.int32 and doc.root["small"].value[1] == rdstream.NA_INTEGER
    assert doc.root["s"].value == ["a", None]

    doc = rdstream.read(data, threshold=10)
    assert doc.root["big"].value == rdstream.Skipped("dbl", 1000)
    assert rdstream.read(data, threshold=10, keep=["[0]"]).root["big"].loaded
    assert not rdstream.read(data, threshold=None, skip=["[0]"]).root["big"].loaded


def test_policy_callback_sees_paths_and_errors_propagate(tmp_path):
    data = rdata(df=data_frame({"g": factor([1, 2], ["u", "v"])}), x=dbl([1.5]))
    seen = []
    doc = rdstream.read(data, policy=lambda path, t, n: seen.append((path, t, n)) or True)
    assert ("$df[0]", "int", 2) in seen and ("$x", "dbl", 1) in seen
    assert list(doc.objects) == ["df", "x"]
    assert rdstream.as_pandas(doc["df"])["g"].tolist() == ["u", "v"]

    def boom(path, t, n):
        raise RuntimeError(f"policy failed at {path}")

    with pytest.raises(RuntimeError, match=r"policy failed at \$df\[0\]"):
        rdstream.read(data, policy=boom)

    path = tmp_path / "x.RData"
    path.write_bytes(data)
    assert "$df: list[1] <data.frame>" in rdstream.describe(path)


def test_extract_streams_files(tmp_path):
    data = rds(rlist({"a": dbl(np.arange(50_000)), "b": chr("x", "y")}))
    manifest = rdstream.extract(data, "[0]", tmp_path / "out")
    entry = manifest["entries"][0]
    assert entry["name"] == "a" and entry["len"] == 50_000
    np.testing.assert_array_equal(np.load(tmp_path / "out" / entry["file"]), np.arange(50_000))


def test_errors():
    with pytest.raises(rdstream.ParseError):
        rdstream.read(b"not an R file")
    with pytest.raises(rdstream.ParseError, match="unexpectedly"):
        rdstream.read(rds(dbl([1, 2, 3]))[:-4])
    with pytest.raises(OSError):
        rdstream.read("/nonexistent/file.rds")


def test_spool_dir_decodes_streams_like_files(tmp_path):
    data = gzip.compress(rds(rlist({"x": dbl(np.arange(10))})))
    doc = rdstream.read(data, spool_dir=tmp_path, threads=2)
    np.testing.assert_array_equal(doc.root["x"].value, np.arange(10))
    assert doc.layers[0]["decoder"] == "ParallelFile"
    assert list(tmp_path.iterdir()) == []  # the spool file is gone
    assert rdstream.read(data).layers[0]["decoder"] == "Serial"
