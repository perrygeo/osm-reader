from ._pyo3_api import rust_func


class OsmPbf:
    def __init__(self, pbf_path):
        self.pbf_path = pbf_path

    def count(self):
        return rust_func(self.pbf_path)
