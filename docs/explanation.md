# Explanation

## Merge policy

When adding documents to a tantivy index, the indexed data will be recorded in multiple 
sections, called _segments_. There is more information about the [Life of a Segment](https://github.com/quickwit-oss/tantivy/wiki/Life-of-a-Segment)
on the [tantivy wiki at Github](https://github.com/quickwit-oss/tantivy/wiki).

Currently, tantivy-py does not offer a way to customize the merge policy, but fortunately
the default merge policy is the [`LogMergePolicy`](https://docs.rs/tantivy/latest/tantivy/merge_policy/struct.LogMergePolicy.html) 
which is a good choice for most use cases. It is aliased as the [default merge policy here](https://docs.rs/tantivy/latest/tantivy/merge_policy/type.DefaultMergePolicy.html).

Segment merging is performed in background threads. After adding documents to an index,
it is important to allow time for those threads to complete merges. This is done by calling
`writer.wait_merging_threads()` as the final step after adding data. This method will
consume the writer and the identifier will no longer be usable.

Here is a short description of the steps in pseudocode:

```
schema = Schema(...)
index = Index(schema)
writer = index.writer()
for ... in data:
    document = Document(...)
    writer.add_document(...)
writer.commit()
writer.wait_merging_threads()
```
