# Reference

## Valid Query Formats

tantivy-py supports the [query language](https://docs.rs/tantivy/latest/tantivy/query/struct.QueryParser.html#method.parse_query) used in tantivy.
Below a few basic query formats are shown:

 - AND and OR conjunctions.
```python
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
query = index.parse_query('"eighty-four days"', ["doc_id"])
(best_score, best_doc_address) = searcher.search(query, 3).hits[0]
best_doc = searcher.doc(best_doc_address)
```
Note: for integer search, the integer field should be indexed.

For more possible query formats and possible query options, see [Tantivy Query Parser Docs.](https://docs.rs/tantivy/latest/tantivy/query/struct.QueryParser.html)

## Escape quotes inside a query string

The tantivy docs for the query parser say that special characters like quotes can be 
escaped inside query values. However, it will also be necessary to surround
the search query in additional quotes, as if a phrase query were being used.

The following will NOT work:

```python
# Raises ValueError
index.parse_query(r'sea\"', ["title", "body"])
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
