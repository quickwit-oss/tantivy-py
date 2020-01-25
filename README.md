[![Build Status](https://travis-ci.org/tantivy-search/tantivy-py.svg?branch=master)](https://travis-ci.org/tantivy-search/tantivy-py)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

tantivy-py
==========

Python bindings for Tantivy.


# Installation

The bindings can be installed using from pypi using pip:

    pip install tantivy

If no binary wheel is present for your operating system the bindings will be
build from source, this means that Rust needs to be installed before building
can succeed.

Note that the bindings are using [PyO3](https://github.com/PyO3/pyo3), which
requires rust nightly and only supports python3.

# Development

Setting up a development enviroment can be done in a virtual environment using
`pipenv` or using local packages using the provided `Makefile`.

For the `pipenv` setup install the virtual environment and build the bindings using:

    pipenv install --dev
    pipenv run maturin develop

After the bindings are build, the tests can be run using:

    pipenv run python -m pytest

For the `Makefile` based setup run:

    make

Running the tests is done using:

    make test

# Usage

The Python bindings have a similar API to Tantivy. To create a index first a schema
needs to be built. After that documents can be added to the index and a reader
can be created to search the index.

```python
import tantivy

# Declaring our schema.
schema_builder = tantivy.SchemaBuilder()
schema_builder.add_text_field("title", stored=True)
schema_builder.add_text_field("body", stored=True)
schema = schema_builder.build()

# Creating our index (in memory, but filesystem is available too)
index = tantivy.Index(schema)


# Adding one document.
writer = index.writer()
writer.add_document(tantivy.Document(
    title=["The Old Man and the Sea"],
    body=["""He was an old man who fished alone in a skiff in the Gulf Stream and he had gone eighty-four days now without taking a fish."""],
))
# ... and committing
writer.commit()


# Reload the index to ensure it points to the last commit.
index.reload()
searcher = index.searcher()
query = index.parse_query("fish days", ["title", "body"])

(best_score, best_doc_address) = searcher.search(query, 3).hits[0]
best_doc = searcher.doc(best_doc_address)
assert best_doc["title"] == ["The Old Man and the Sea"]
print(best_doc)
```
