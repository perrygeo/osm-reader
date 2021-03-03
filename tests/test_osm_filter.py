from osm_filter import OsmPbf

# -------------------------------------- #
# Test the public Python API
# -------------------------------------- #


def test_osmpfb():
    op = OsmPbf("test")
    assert op.count() == 15


# -------------------------------------- #
# Test the private Rust-based APIs
# -------------------------------------- #
from osm_filter import _pyo3_api as rust


def test_rust():
    assert rust.rust_func("test") == 15