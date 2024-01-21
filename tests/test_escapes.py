import pytest

from tantivy import Query


def test_escape_quote_parse_query(ram_index):
    index = ram_index
    with pytest.raises(ValueError) as ex:
        # This test to show that surrounding quotes are required to allow
        # a single to be escaped the quote, otherwise it will raise `ValueError`.
        _ = index.parse_query(r'sea\"', ["title", "body"])


def test_escape_quote_parse_query_with_quotes(ram_index):
    index = ram_index
    # We verify only that `parse_query` doesn't raise. We are not testing
    # whether tantivy's `parse_query` is correct.
    query = index.parse_query(r'"sea\""', ["title", "body"])


def test_escape_quote_parse_query_quoted(ram_index):
    index = ram_index
    # We verify only that `parse_query` doesn't raise. We are not testing
    # whether tantivy's `parse_query` is correct.
    query = index.parse_query(r'title:"sea \"whale"')


def test_escape_quote_term_query(ram_index):
    index = ram_index
    # We verify only that `parse_query` doesn't raise. We are not testing
    # whether tantivy's `parse_query` is correct.
    query = Query.term_query(index.schema, "title", "sea\" whale")
