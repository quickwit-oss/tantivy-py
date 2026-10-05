# Query Language

## Setup

We'll use a test index for the examples that follow.

```python
import os
from tantivy import SchemaBuilder, Index, Document
schema = (
    SchemaBuilder()
        .add_integer_field("doc_id", indexed=True, stored=True)
        .add_text_field("title", stored=True)
        .add_text_field("body")
        .build()
)
index = Index(schema=schema, path=None)
writer = index.writer(heap_size=15_000_000, num_threads=1)
doc = Document()
doc.add_integer("doc_id", 1)
doc.add_text("title", "The Old Man and the Sea")
doc.add_text(
    "body",
    (
        "He was an old man who fished alone in a skiff in"
        "the Gulf Stream and he had gone eighty-four days "
        "now without taking a fish."
    ),
)
writer.add_document(doc)

doc = Document()
doc.add_integer("doc_id", 2)
doc.add_text("title", "The Old Man and the Sea II")
doc.add_text("body", "He was an old man who sailed alone.")

writer.add_document(doc)
writer.commit()
index.reload()
```

## Valid Query Formats

tantivy-py supports the [query language](https://docs.rs/tantivy/latest/tantivy/query/struct.QueryParser.html#method.parse_query) used in tantivy.
Below a few basic query formats are shown:

 - AND and OR conjunctions.
```python
searcher = index.searcher()
query = index.parse_query('(Old AND Man) OR Stream', ["title", "body"])
(best_score, best_doc_address) = searcher.search(query, 3).hits[0]
best_doc = searcher.doc(best_doc_address)
```

 - +(includes) and -(excludes) operators.
```python
query = index.parse_query('+Old +Man chef -fished', ["title", "body"])
(best_score, best_doc_address) = searcher.search(query, 3).hits[0]
best_doc = searcher.doc(best_doc_address)
```
Note: in a query like above, a word with no +/- acts like an OR.

 - phrase search.
```python
query = index.parse_query('"eighty-four days"', ["title", "body"])
(best_score, best_doc_address) = searcher.search(query, 3).hits[0]
best_doc = searcher.doc(best_doc_address)
```

- integer search
```python
query = index.parse_query('1', ["doc_id"])
(best_score, best_doc_address) = searcher.search(query, 3).hits[0]
best_doc = searcher.doc(best_doc_address)
```
Note: for integer search, the integer field should be indexed.

For more possible query formats and possible query options, see [Tantivy Query Parser Docs.](https://docs.rs/tantivy/latest/tantivy/query/struct.QueryParser.html)

## Field boosts

Use `field_boosts` to make increase the importance of certain matched
fields when scoring relevance. The keys are field names. The values are
multipliers for the score.

In this example below, the query matches "ii" in the title of document 2,
and "skiff" in the body of document 1. Without boosts, document 2 is first:

```python
query = index.parse_query("ii skiff", ["title", "body"])
hits = searcher.search(query, 3).hits
assert [searcher.doc(addr)["doc_id"][0] for _, addr in hits] == [2, 1]
```

A boost on `body` makes document 1 first:

```python
query = index.parse_query(
    "ii skiff", ["title", "body"], field_boosts={"body": 3.0}
)
hits = searcher.search(query, 3).hits
assert [searcher.doc(addr)["doc_id"][0] for _, addr in hits] == [1, 2]
```

You can also boost one term in the query string with `^`:

```python
query = index.parse_query("ii body:skiff^3", ["title", "body"])
hits = searcher.search(query, 3).hits
assert [searcher.doc(addr)["doc_id"][0] for _, addr in hits] == [1, 2]
```

## Fuzzy fields

Use `fuzzy_fields` to match terms that are near to the query terms.
The keys are field names. Each value is a tuple
`(prefix, distance, transpose_cost_one)`:

 - `prefix`: if `True`, a term also matches when the query term is near to the start of that term.
 - `distance`: the maximum [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) between the query term and a matching term.
 - `transpose_cost_one`: if `True`, a swap of two adjacent characters counts as one edit, not two.

The misspelled term "skif" does not match a document:

```python
query = index.parse_query("skif", ["title", "body"])
assert searcher.search(query, 3).hits == []
```

With a fuzzy `body` field, "skif" matches "skiff" in document 1:

```python
query = index.parse_query(
    "skif", ["title", "body"], fuzzy_fields={"body": (False, 1, True)}
)
hits = searcher.search(query, 3).hits
assert [searcher.doc(addr)["doc_id"][0] for _, addr in hits] == [1]
```

The `parse_query_lenient` method accepts the same `field_boosts`
and `fuzzy_fields` arguments.

## Escape quotes inside a query string

The tantivy docs for the query parser say that special characters like quotes can be 
escaped inside query values. However, it will also be necessary to surround
the search query in additional quotes, as if a phrase query were being used.

The following will NOT work:

```python
try:
    index.parse_query(r'sea\"', ["title", "body"])
except ValueError as e:
    assert str(e) == r'Syntax Error: sea\"'
```

However, the following will succeed:

```python
# Works!
index.parse_query(r'"sea\""', ["title", "body"])
```

Note that whether the included (and escaped) quote actually gets used
to match documents depends on the tokenizer used for the field. For example,
the default tokenizer will not match the document "sea\"s" with the query
"sea\"", because this tokenizer discards punctuation. 
