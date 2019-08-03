from setuptools import setup
from os import path

try:
    from setuptools_rust import Binding, RustExtension
except ImportError:
    print("Please install setuptools-rust package")
    raise SystemExit(1)

this_directory = path.abspath(path.dirname(__file__))
with open(path.join(this_directory, 'README.md'), encoding='utf-8') as f:
    long_description = f.read()


setup(
    name="tantivy",
    version="0.10.1",
    rust_extensions=[RustExtension("tantivy.tantivy", binding=Binding.PyO3)],
    packages=["tantivy"],
    description=("Python bindings for the Tantivy full-text search engine "
                 "library"),
    long_description=long_description,
    long_description_content_type="text/markdown",
    license="MIT",

    zip_safe=False,
)
