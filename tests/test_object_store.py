"""Indexes stored in an object store, using the in-memory `memory://` backend.

These tests need an extension built with object store support (the `cloud`
feature, or `object-store` for the in-memory backend only). Without it, only
the test checking the error message runs.
"""

import uuid

import pytest

from tantivy import Document, Index, SchemaBuilder


def _has_object_store_support() -> bool:
    try:
        Index.exists("memory:///probe")
    except ValueError as e:
        if "no object store support" in str(e):
            return False
        raise
    return True


HAS_OBJECT_STORE = _has_object_store_support()
needs_object_store = pytest.mark.skipif(
    not HAS_OBJECT_STORE, reason="built without object store support"
)


def _schema():
    return (
        SchemaBuilder()
        .add_text_field("title", stored=True)
        .add_text_field("body")
        .build()
    )


@pytest.fixture
def uri():
    # Every memory:// URI shares one store per process, so isolate by prefix.
    return f"memory:///tests/{uuid.uuid4()}"


def _add(index, *titles):
    writer = index.writer(15_000_000, 1)
    for title in titles:
        writer.add_document(Document(title=title, body=f"body of {title}"))
    writer.commit()
    writer.wait_merging_threads()


def _search_titles(index, query):
    index.reload()
    searcher = index.searcher()
    result = searcher.search(index.parse_query(query, ["title", "body"]), 10)
    return sorted(searcher.doc(addr)["title"][0] for _, addr in result.hits)


@needs_object_store
def test_create_write_and_reopen(uri):
    assert not Index.exists(uri)

    index = Index(_schema(), path=uri)
    _add(index, "old man and the sea", "of mice and men")
    assert _search_titles(index, "sea") == ["old man and the sea"]

    assert Index.exists(uri)
    assert Index.is_compatible(uri)

    reopened = Index.open(uri)
    assert _search_titles(reopened, "and") == [
        "of mice and men",
        "old man and the sea",
    ]


@needs_object_store
def test_reuse_appends_to_existing_index(uri):
    _add(Index(_schema(), path=uri), "first")
    index = Index(_schema(), path=uri, reuse=True)
    _add(index, "second")
    assert _search_titles(index, "body") == ["first", "second"]


@needs_object_store
def test_second_reader_sees_new_commits_after_reload(uri):
    writer_side = Index(_schema(), path=uri)
    _add(writer_side, "one")
    reader_side = Index.open(uri)
    assert _search_titles(reader_side, "body") == ["one"]

    _add(writer_side, "two")
    assert _search_titles(reader_side, "body") == ["one", "two"]


@needs_object_store
def test_only_one_writer_at_a_time(uri):
    index = Index(_schema(), path=uri)
    writer = index.writer(15_000_000, 1)
    with pytest.raises(ValueError, match="(?i)lock"):
        Index.open(uri).writer(15_000_000, 1)
    writer.commit()
    writer.wait_merging_threads()
    del writer
    # Once the first writer is gone the lock is released.
    Index.open(uri).writer(15_000_000, 1)


@needs_object_store
def test_open_missing_index_raises(uri):
    with pytest.raises(ValueError):
        Index.open(uri)


@needs_object_store
def test_unknown_storage_option_raises(uri):
    with pytest.raises(ValueError, match="Unknown storage option 'region'"):
        Index(_schema(), path=uri, storage_options={"region": "us-east-1"})


def test_storage_options_rejected_for_filesystem_path(tmp_path):
    with pytest.raises(ValueError, match="storage_options"):
        Index(_schema(), path=str(tmp_path), storage_options={"a": "b"})
    with pytest.raises(ValueError, match="storage_options"):
        Index(_schema(), storage_options={"a": "b"})


@pytest.mark.skipif(HAS_OBJECT_STORE, reason="built with object store support")
def test_uri_without_object_store_support_explains_why():
    with pytest.raises(ValueError, match="no object store support"):
        Index(_schema(), path="s3://bucket/index")
