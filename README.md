[![Build Status](https://travis-ci.org/tantivy-search/tantivy-py.svg?branch=master)](https://travis-ci.org/tantivy-search/tantivy-py)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

tantivy-py
==========

Python bindings for tantivy.


# Installation

The bindings can be installed using setuptools:

    python3 setup.py install --user

Note that this requires setuptools-rust to be installed. Another thing to note
is that the bindings are using [PyO3](https://github.com/PyO3/pyo3), which
requires rust nightly and only supports python3.

# Usage

tantivy-py has a similar API to tantivy. To create a index first a schema
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
top_docs = tantivy.TopDocs(3)

(best_score, best_doc_address) = searcher.search(query, top_docs)[0]
best_doc = searcher.doc(best_doc_address) 
assert best_doc["title"] == ["The Old Man and the Sea"]
print(best_doc)
```
