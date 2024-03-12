import gc
import pytest
import psutil

from tantivy import Document, Index, SchemaBuilder


def kapiche_schema():
    return (
        SchemaBuilder()
        .add_text_field("title", stored=True)
        .add_text_field("body", tokenizer_name='kapiche_tokenizer')
        .add_unsigned_field("document_id__", stored=True, indexed=True, fast=True)
        .add_unsigned_field("frame_id__", stored=True, indexed=True, fast=True)
        .add_unsigned_field("sentence_id__", stored=True, indexed=True, fast=True)
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

    def test_stat_searcher_none(self, ram_kapiche_index):
        index = ram_kapiche_index
        query = index.parse_query("sea", ["body"])
        result = index.stat_searcher().search(query)
        assert sorted(result.unique_docs_frames) == [(1, 1), (1, 2), (2, 4), (3, 5)]

    def test_stat_searcher_filter(self, ram_kapiche_index):
        index = ram_kapiche_index
        query = index.parse_query("sea", ["body"])

        result = index.stat_searcher().search(query, "frame_id__", {1, 3, 5, 6, 7})
        assert sorted(result.unique_docs_frames) == [(1, 1), (3, 5)]
        assert list(result.unique_docs) == [1, 3]
        assert list(result.unique_frames) == [1, 5]
        assert list(result.unique_sentences) == [1, 2, 7]
        print(f"{result.hits}")
        print(f"{result.unique_docs_frames}")
        print(f"{result.unique_sentences}")

    def test_stat_searcher_filter_unzipped(self, ram_kapiche_index):
        index = ram_kapiche_index
        query = index.parse_query("sea", ["body"])

        result = index.stat_searcher().search(query, "frame_id__", {1, 3, 5, 6, 7})
        d, f = result.unique_docs_frames_unzipped
        assert sorted(d) == [1, 3]
        assert sorted(f) == [1, 5]
        assert list(result.unique_docs) == [1, 3]
        assert list(result.unique_frames) == [1, 5]

        d, f, s = result.unique_docs_frames_sentences_unzipped
        assert sorted(s) == [1, 2, 7]


def test_stat_searcher_memory():
    # Create index
    schema = (
        SchemaBuilder()
        .add_text_field("title", stored=True)
        .add_text_field("body", tokenizer_name='kapiche_tokenizer')
        .add_unsigned_field("document_id__", stored=True, indexed=True, fast=True)
        .add_unsigned_field("frame_id__", stored=True, indexed=True, fast=True)
        .add_unsigned_field("sentence_id__", stored=True, indexed=True, fast=True)
        .build()
    )

    index = Index(schema, None)
    writer = index.writer()

    sherlock = (
        open("tests/sherlock.txt", "r", encoding="utf-8-sig").read().split("\n\n")
    )
    for i, paragraph in enumerate(sherlock):
        doc = Document()
        doc.add_text("title", f"Paragraph {i}")
        doc.add_text("body", paragraph)
        doc.add_unsigned("document_id__", i)
        doc.add_unsigned("frame_id__", i)
        doc.add_unsigned("sentence_id__", i)
        writer.add_document(doc)

    writer.commit()
    index.reload()
    gc.collect()

    # Run search
    query = index.parse_query("Holmes", ["body"])

    p = psutil.Process()
    print()
    print(f'Scored     {"iter":>4}: {"":>16} {"":>16} {"delt.Mem (bytes)":>16}')
    n = 200
    m0 = p.memory_info().rss
    total_mem_growth = 0
    for i in range(n):
        result = index.stat_searcher().search(query)
        items = sorted(result.unique_docs_frames)
        del items
        del result
        gc.collect()
        if i % (n // 10) == 0:
            m1 = p.memory_info().rss
            print(f'Score=Fals {i:0>4}: {m0:>16d} {m1:>16d} {(m1 - m0):>16d}')
            total_mem_growth += (m1 - m0)
            m0 = p.memory_info().rss

    assert total_mem_growth < 500_000

    result = index.stat_searcher().search(query)
    items = sorted(result.unique_docs_frames)
    assert len(items) == 441
    assert items[:4] == [(0, 0), (2, 2), (8, 8), (11, 11)]
