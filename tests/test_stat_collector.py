import tantivy
import pytest

from tantivy import Document, Index, SchemaBuilder


def kapiche_schema():
    return (
        SchemaBuilder()
        .add_text_field("title", stored=True)
        .add_text_field("body", tokenizer_name='kapiche_tokenizer')
        .add_unsigned_field("document_id__", stored=True, indexed=True, fast='single')
        .add_unsigned_field("frame_id__", stored=True, indexed=True, fast='single')
        .add_unsigned_field("sentence_id__", stored=True, indexed=True, fast='single')
        .build()
    )


def create_kapiche_index(dir=None):
    # assume all tests will use the same documents for now
    # other methods may set up function-local indexes
    index = Index(kapiche_schema(), dir)
    writer = index.writer()

    quotes = [
        ('''The sea''', 1, 1, 1),
        ('''The old man and the sea.''', 1, 1, 2),
        ('''The old duck and the sea.''', 1, 2, 3),
        ('''The old duck and the lake.''', 2, 3, 4),
        ('''The old man and the lake.''', 2, 3, 5),
        ('''The young man and the sea.''', 2, 4, 6),
        ('''The young duck and the sea.''', 3, 5, 7),
        ('''The young duck and the lake.''', 4, 6, 8),
        ('''The young man and the lake.''', 5, 7, 8),
    ]
    for i, tup in enumerate(quotes, start=1):
        doc = Document()
        # create a document instance
        # add field-value pairs
        doc.add_text("title", "A quote")
        doc.add_text("body", tup[0])
        doc.add_unsigned("document_id__", tup[1])
        doc.add_unsigned("frame_id__", tup[2])
        doc.add_unsigned("sentence_id__", tup[3])
        writer.add_document(doc)

    writer.commit()
    index.reload()
    return index


@pytest.fixture(scope="class")
def ram_kapiche_index():
    return create_kapiche_index()


class TestClass(object):
    def test_simple_search_in_kapiche_ram(self, ram_kapiche_index):
        index = ram_kapiche_index
        query = index.parse_query("sea", ["body"])

        result = index.searcher().search(query, 10)
        assert len(result.hits) == 5
        _, doc_address = result.hits[0]

    def test_stat_searcher(self, ram_kapiche_index):
        index = ram_kapiche_index
        query = index.parse_query("sea", ["body"])

        result = index.stat_searcher().search(query, set())
        assert sorted(result.unique_docs_frames) == [(1, 1), (1, 2), (2, 4), (3, 5)]

    def test_stat_searcher_filter(self, ram_kapiche_index):
        index = ram_kapiche_index
        query = index.parse_query("sea", ["body"])

        result = index.stat_searcher().search(query, {1, 3, 5, 6, 7})
        assert sorted(result.unique_docs_frames) == [(1, 1), (3, 5)]
        assert result.unique_docs == {1, 3}
        assert result.unique_frames == {1, 5}
        print(f"{result.hits}")
        print(f"{result.unique_docs_frames}")
