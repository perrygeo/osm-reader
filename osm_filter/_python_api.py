from ._pyo3_api import count


class OsmPbf:
    def __init__(self, pbf_path):
        self.pbf_path = pbf_path

    def count(self):
        return count(self.pbf_path)
