from tantivy.tantivy import Document, Index, SchemaBuilder
from tests.conftest import build_schema_numeric_fields
import pytest


@pytest.mark.parametrize("weight_by_field", [
    "weight_f64",
    "weight_i64",
    "weight_u64"
])
def test_document_scoring(weight_by_field: str):
    schema = (
        SchemaBuilder()
        .add_integer_field("id", stored=True, indexed=True, fast=True)
        .add_float_field("weight_f64", stored=True, indexed=True, fast=True)
        .add_integer_field("weight_i64", stored=True, indexed=True, fast=True)
        .add_unsigned_field("weight_u64", stored=True, indexed=True, fast=True)
        .add_text_field("body", stored=True, fast=True)
        .build()
    )
    index = Index(schema)
    writer = index.writer(15_000_000, 1)

    with writer:
        doc = Document()
        doc.add_integer("id", 1)
        doc.add_float("weight_f64", 0.1)
        doc.add_integer("weight_i64", 1)
        doc.add_unsigned("weight_u64", 1)
        doc.add_text("body", "apple banana orange mango")
        _ = writer.add_document(doc)

        doc = Document()
        doc.add_integer("id", 2)
        doc.add_float("weight_f64", 0.9)
        doc.add_integer("weight_i64", 10)
        doc.add_unsigned("weight_u64", 10)
        doc.add_text("body", "pear lemon tomato banana")
        _ = writer.add_document(doc)

    index.reload()

    searcher = index.searcher()

    query_text = "body:banana"
    query = index.parse_query(query_text)
    results = searcher.search(query, limit=1)
    assert len(results.hits) == 1
    print(results)
    _, doc_address = results.hits[0]
    d = index.searcher().doc(doc_address)
    assert d["id"] == [1]


    query_text = "body:banana"
    query = index.parse_query(query_text)
    results = searcher.search(query, limit=1, weight_by_field=weight_by_field)
    assert len(results.hits) == 1
    print(results)
    _, doc_address = results.hits[0]
    d = index.searcher().doc(doc_address)
    assert d["id"] == [2]


def test_not_fastfield():
    schema = (
        SchemaBuilder()
        .add_integer_field("id", stored=True, indexed=True, fast=True)
        .add_float_field("weight_f64", stored=True, indexed=True, fast=False)
        .add_text_field("body", stored=True, fast=True)
        .build()
    )
    index = Index(schema)
    index.reload()

    searcher = index.searcher()

    query_text = "body:banana"
    query = index.parse_query(query_text)
    with pytest.raises(ValueError, match="not a fast field"):
        _ = searcher.search(query, limit=1, weight_by_field="weight_f64")
