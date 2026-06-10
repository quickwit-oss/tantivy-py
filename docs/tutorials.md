# Tutorials

## Building an index and populating it

```python
import tempfile
import pathlib
import tantivy

# Declaring our schema.
schema_builder = tantivy.SchemaBuilder()
schema_builder.add_text_field("title", stored=True)
schema_builder.add_text_field("body", stored=True)
schema_builder.add_integer_field("doc_id",stored=True)
schema = schema_builder.build()

# Creating our index (in memory)
index = tantivy.Index(schema)
```

To have a persistent index, use the path
parameter to store the index on the disk, e.g:

```python
tmpdir = tempfile.TemporaryDirectory()
index_path = pathlib.Path(tmpdir.name) / "index"
index_path.mkdir()
persistent_index = tantivy.Index(schema, path=str(index_path))
```

By default, tantivy  offers the following tokenizers
which can be used in tantivy-py:
 -  `default`
`default` is the tokenizer that will be used if you do not
 assign a specific tokenizer to your text field.
 It will chop your text on punctuation and whitespaces,
 removes tokens that are longer than 40 chars, and lowercase your text.

-  `raw`
 Does not actual tokenizer your text. It keeps it entirely unprocessed.
 It can be useful to index uuids, or urls for instance.

-  `en_stem`

 In addition to what `default` does, the `en_stem` tokenizer also
 apply stemming to your tokens. Stemming consists in trimming words to
 remove their inflection. This tokenizer is slower than the default one,
 but is recommended to improve recall.

to use the above tokenizers, simply provide them as a parameter to `add_text_field`. e.g.
```python
schema_builder_tok = tantivy.SchemaBuilder()
schema_builder_tok.add_text_field("body",  stored=True,  tokenizer_name='en_stem')
```

## Adding one document.

```python
writer = index.writer()
writer.add_document(tantivy.Document(
	doc_id=1,
    title=["The Old Man and the Sea"],
    body=["""He was an old man who fished alone in a skiff in the Gulf Stream and he had gone eighty-four days now without taking a fish."""],
))
# ... and committing
writer.commit()
writer.wait_merging_threads()
```

Note that `wait_merging_threads()` must come at the end, because
the `writer` object will not be usable after this call.

Alternatively `writer` can be used as a context manager. The same block of code can then be written as

```python
with index.writer() as writer:
    writer.add_document(tantivy.Document(
        doc_id=1,
        title=["The Old Man and the Sea"],
        body=["""He was an old man who fished alone in a skiff in the Gulf Stream and he had gone eighty-four days now without taking a fish."""],
))
```

Both `commit()` and `wait_merging_threads()` is called when the with-block is exited.

## Building and Executing Queries with the Query Parser

With the Query Parser, you can easily build simple queries for your index.

First you need to get a searcher for the index

```python
# Reload the index to ensure it points to the last commit.
index.reload()
searcher = index.searcher()
```

Then you need to get a valid query object by parsing your query on the index.

```python
query = index.parse_query("fish days", ["title", "body"])
(best_score, best_doc_address) = searcher.search(query, 3).hits[0]
best_doc = searcher.doc(best_doc_address)
assert best_doc["title"] == ["The Old Man and the Sea"]
```

The `parse_query` method takes in a query string (visit [reference](reference.md#valid-query-formats) for more details on the syntax) and create a `Query` object that can be used to search the index.

In Tantivy, hit documents during search will return a `DocAddress` object that can be used to retrieve the document from the searcher, rather than returning the document directly.

## Building and Executing Queries with Query Objects

> *This is an advanced topic. Only consider this if you need very fine-grained control over your queries, or existing query parsers do not meet your needs.*

If you have a Lucene / ElasticSearch background, you might be more comfortable building nested queries programmatically. Also, some queries (e.g. ConstQuery, DisjunctionMaxQuery) are not supported by the query parser due to their complexity in expression.

Consider the following query in ElasticSearch:

```json
{
    "query": {
        "bool": {
            "must": [
                {
                    "dis_max": {
                        "queries": [
                            {
                                "match": {
                                    "title": {
                                        "query": "fish",
                                        "boost": 2
                                    }
                                }
                            },
                            {
                                "match": {
                                    "body": {
                                        "query": "eighty-four days",
                                        "boost": 1.5
                                    }
                                }
                            }
                        ],
                        "tie_breaker": 0.3
                    }
                }
            ]
        }
    }
}
```

It is impossible to express this query using the query parser. Instead, you can build the query programmatically mixing with the query parser:

```python
from tantivy import Query, Occur, Index

...

complex_query = Query.boolean_query(
    [
        (
            Occur.Must,
            Query.disjunction_max_query(
                [
                    Query.boost_query(
                        # by default, only the query parser will analyze
                        # your query string
                        index.parse_query("fish", ["title"]), 
                        2.0
                    ),
                    Query.boost_query(
                        index.parse_query("eighty-four days", ["body"]), 
                        1.5
                    ),
                ],
                0.3,
            ),
        )
    ]
)

```

<!--TODO: Update the reference link to the query parser docs when available.-->

## Combining Queries with the Boolean Helper Methods

`Query.boolean_query(...)` shown above is the most general way to combine
queries, but for the common AND, OR and AND-NOT combinations there are three
convenience methods that read more naturally:

- `q.and_must_match(*queries)` matches documents that match `q` and every
  given query (AND)
- `q.and_must_not_match(*queries)` matches documents that match `q` and none
  of the given queries (AND NOT)
- `q.or_should_match(*queries)` matches documents that match `q` or any of
  the given queries (OR)

Each method returns a new `Query`, leaving the originals untouched.

```python
from tantivy import Query

query_old = Query.term_query(schema, "title", "old")
query_man = Query.term_query(schema, "title", "man")
query_whale = Query.term_query(schema, "title", "whale")

# Match documents whose title contains both "old" AND "man"
combined = query_old.and_must_match(query_man)
(score, doc_address) = searcher.search(combined, 3).hits[0]
assert searcher.doc(doc_address)["title"] == ["The Old Man and the Sea"]

# Match documents whose title contains "old" but NOT "whale"
combined = query_old.and_must_not_match(query_whale)
assert len(searcher.search(combined, 3).hits) > 0

# Match documents whose title contains "old" OR "whale"
combined = query_old.or_should_match(query_whale)
assert len(searcher.search(combined, 3).hits) > 0
```

The methods accept any number of queries, so a list of queries can be applied
in a single call using argument unpacking:

```python
query_sea = Query.term_query(schema, "title", "sea")

# Equivalent to query_old.and_must_match(query_man, query_sea)
queries = [query_man, query_sea]
combined = query_old.and_must_match(*queries)
assert len(searcher.search(combined, 3).hits) > 0
```

The methods can also be chained:

```python
combined = (
    query_old
    .and_must_match(query_man)
    .and_must_not_match(query_whale)
    .or_should_match(query_sea)
)
assert len(searcher.search(combined, 3).hits) > 0
```

Note how the precedence works: each call in the chain applies to the whole
query built so far, not just to the preceding step. So the
`or_should_match(query_sea)` call above applies to the combination of the
first three queries, and the chain matches documents satisfying
"(old AND man AND NOT whale) OR sea". There is no AND-before-OR operator
precedence as in the boolean expressions of most programming languages;
grouping simply follows the order of the method calls, left to right. For any
other grouping, build the groups as separate queries first and then combine
them:

```python
# old AND (man OR sea)
man_or_sea = query_man.or_should_match(query_sea)
combined = query_old.and_must_match(man_or_sea)
assert len(searcher.search(combined, 3).hits) > 0
```

Although you can think of each call as wrapping the query built so far in a
new boolean query, the helpers avoid building one level of nesting per call
where they can: when the query being extended is already a boolean query, the
new clauses are appended to it directly, provided that doing so matches the
same documents. This is the case for `and_must_match` and
`and_must_not_match` chains generally, and for `or_should_match` when the
query built so far is a pure OR. So a long chain of AND clauses produces a
single flat boolean query rather than a deeply nested one. This is purely an
internal optimization — the matched documents are the same either way.

## Debugging Queries with explain()

When working with search queries, it's often useful to understand why a particular document matched a query and how its score was calculated. The `explain()` method provides detailed information about the scoring process.

```python
# Let's search with the complex_query built earlier and get the top result
result = searcher.search(complex_query, 10)
if result.hits:
    score, doc_address = result.hits[0]
    
    # Get an explanation for why this document matched
    explanation = complex_query.explain(searcher, doc_address)
    
    # The explanation provides a JSON representation of the scoring details
    explanation_json = explanation.to_json()
    print(explanation_json)
```

The `to_json()` method returns a pretty-printed JSON string that shows the final score value,
a breakdown of the score was calculated, details about which query clauses matched, and the contribution
of individual terms.

This is particularly useful when debugging why certain documents rank higher than others.

Example output might look like:
```json
{
  "value": 2.5,
  "description": "sum of:",
  "details": [
    {
      "value": 2.0,
      "description": "weight(title:fish) with boost 2.0"
    },
    {
      "value": 0.5,
      "description": "weight(body:days)"
    }
  ]
}
```

## Using the snippet generator

Let's revisit the query `"fish days"` in our [example](#building-and-executing-queries-with-the-query-parser):

```python
hit_text = best_doc["body"][0]
print(f"{hit_text=}")
assert hit_text == (
    "He was an old man who fished alone in a skiff in the "
    "Gulf Stream and he had gone eighty-four days now "
    "without taking a fish."
)

from tantivy import SnippetGenerator
snippet_generator = SnippetGenerator.create(
    searcher, query, schema, "body"
)
snippet = snippet_generator.snippet_from_doc(best_doc)
```

The snippet contains a **fragment** of the document text — typically a
window around the matched terms. You can retrieve it with `fragment()`:

```python
fragment = snippet.fragment()
```

The `highlighted()` method returns ranges whose offsets are **relative to
the fragment**, not the original document text. Use the fragment for
slicing:

```python
highlights = snippet.highlighted()
first_highlight = highlights[0]
assert first_highlight.start == 93
assert first_highlight.end == 97
assert fragment[first_highlight.start:first_highlight.end] == "days"
```

> **Note:** For short documents the fragment may cover the entire text, so
> slicing the original document would also work. For longer documents the
> fragment is a substring and the offsets will only be correct against
> `fragment()`.

The snippet object can also generate a marked-up HTML snippet:

```python
html_snippet = snippet.to_html()
assert html_snippet == (
    "He was an old man who fished alone in a skiff in the "
    "Gulf Stream and he had gone eighty-four <b>days</b> now "
    "without taking a <b>fish</b>"
)
```


## Create a Custom Tokenizer (Text Analyzer)

Tantivy provides several built-in tokenizers and filters that
can be chained together to create new tokenizers (or
'text analyzers') that better fit your needs.

Tantivy-py lets you access these components, assemble them,
and register the result with an index.

Let's walk through creating and registering a custom text analyzer
to see how everything fits together.

### Example

First, let's create a text analyzer. As explained further down,
a text analyzer is a pipeline consisting of one tokenizer and
any number of token filters.

```python
from tantivy import (
    TextAnalyzer,
    TextAnalyzerBuilder,
    Tokenizer,
    Filter,
    Index,
    SchemaBuilder
)

my_analyzer: TextAnalyzer = (
    TextAnalyzerBuilder(
        # Create a `Tokenizer` instance.
        # It instructs the builder about which type of tokenizer
        # to create internally and with which arguments.
        Tokenizer.regex(r"(?i)([a-z]+)")
    )
    .filter(
        # Create a `Filter` instance.
        # Like `Tokenizer`, this object provides instructions
        # to the builder.
        Filter.lowercase()
    )
    .filter(
        # Define custom words.
        Filter.custom_stopword(["www", "com"])
    )
    # Finally, build a TextAnalyzer
    # chaining all tokenizer > [filter, ...] steps together.
    .build()
)
```

We can check that our new analyzer is working as expected
by passing some text to its `.analyze()` method.

```python
# Will print: ['this', 'website', 'might', 'exist']
my_analyzer.analyze('www.this1website1might1exist.com')
```

The next step is to register our analyzer with an index. Let's
assume we already have one.

```python
index.register_tokenizer("custom_analyzer", my_analyzer)
```

To link an analyzer to a field in the index, pass the
analyzer name to the `tokenizer_name=` parameter of
the `SchemaBuilder`'s `add_text_field()` method.

Here is the schema that was used to construct our index:

```python
schema = (
    tantivy.SchemaBuilder()
    .add_text_field("content", tokenizer_name="custom_analyzer")
    .build()
)
index = Index(schema)
```

Summary:

1. Use `TextAnalyzerBuilder`, `Tokenizer`, and `Filter` to build a `TextAnalyzer`
2. The analyzer's `.analyze()` method lets you use your analyzer as a tokenizer from Python.
3. Refer to your analyzer's name when building the index schema.
4. Use the same name when registering your analyzer on the index.


### On terminology: Tokenizer vs. Text Analyzer

Tantivy-py mimics Tantivy's interface as closely as possible.
This includes minor terminological inconsistencies, one of
which is how Tantivy distinguishes between 'tokenizers' and
'text analyzers'.

Quite simply, a 'tokenizer' segments text into tokens.
A 'text analyzer' is a pipeline consisting of one tokenizer
and zero or more token filters. The `TextAnalyzer` is the
primary object of interest when talking about how to
change Tantivy's tokenization behavior.

Slightly confusingly, though, the `Index` and `SchemaBuilder`
interfaces use 'tokenizer' to mean 'text analyzer'.

This inconsistency can be observed in `SchemaBuilder.add_text_field`, e.g. --

```
SchemaBuilder.add_text_field(..., tokenizer_name=<analyzer name>)`
```

-- and in the name of the `Index.register_tokenizer(...)` method, which actually
serves to register a *text analyzer*.

## How to use aggregations

Aggregations summarize your data as metrics, statistics, or other analytics.
Tantivy-py supports a subset of the aggregations available in Tantivy.

### Cardinality Aggregation

The cardinality aggregation allows you to get the number of unique values
for a given field.

```python
import tantivy

# Create a schema with a numeric field
schema_builder = tantivy.SchemaBuilder()
schema_builder.add_integer_field("id", stored=True)
schema_builder.add_float_field("rating", stored=True, fast=True)
schema = schema_builder.build()

# Create an index in RAM
index = tantivy.Index(schema)

# Add some documents
writer = index.writer()
with writer:
    writer.add_document(tantivy.Document(id=1, rating=3.5))
    writer.add_document(tantivy.Document(id=2, rating=4.5))
    writer.add_document(tantivy.Document(id=3, rating=3.5))

# Reload the index to make the changes available for search
index.reload()

# Create a searcher
searcher = index.searcher()

# Create a query that matches all documents
query = tantivy.Query.all_query()

# Get the cardinality of the "rating" field
cardinality = searcher.cardinality(query, "rating")

assert cardinality == 2.0
```
