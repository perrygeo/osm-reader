from setuptools import setup, find_namespace_packages
from setuptools_rust import Binding, RustExtension


setup(
    name="osm_filter",
    version="0.1.0",
    packages=find_namespace_packages(include=["osm_filter.*"]),
    zip_safe=False,
    rust_extensions=[
        RustExtension(
            "osm_filter._pyo3_api", path="Cargo.toml", binding=Binding.PyO3, debug=False
        )
    ],
)
