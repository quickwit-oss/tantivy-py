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
