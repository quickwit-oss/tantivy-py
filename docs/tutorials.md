# Tutorials

## Building an index and populating it

```python
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
index = tantivy.Index(schema, path=os.getcwd() + '/index')
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
schema_builder.add_text_field("body",  stored=True,  tokenizer_name='en_stem')
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
```

## Building and Executing Queries

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
print(best_doc)
```

