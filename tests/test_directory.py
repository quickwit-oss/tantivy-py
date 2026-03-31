"""Tests for the custom Python Directory support."""

import json
import threading

import tantivy
from tantivy import Document, Index, SchemaBuilder


class InMemoryDirectory:
    """A pure-Python in-memory Directory implementation for testing."""

    def __init__(self):
        self._files = {}  # path -> bytes
        self._writers = {}  # writer_id -> (path, BytesIO-like buffer)
        self._next_writer_id = 0
        self._lock = threading.Lock()

    def get_file_handle(self, path: str) -> bytes:
        with self._lock:
            if path not in self._files:
                raise FileNotFoundError(f"File not found: {path}")
            return self._files[path]

    def open_write(self, path: str) -> int:
        with self._lock:
            if any(p == path for p, _ in self._writers.values()):
                raise FileExistsError(f"File already opened for write: {path}")
            writer_id = self._next_writer_id
            self._next_writer_id += 1
            self._writers[writer_id] = (path, bytearray())
            return writer_id

    def write(self, writer_id: int, data: bytes) -> None:
        with self._lock:
            _, buf = self._writers[writer_id]
            buf.extend(data)

    def flush(self, writer_id: int) -> None:
        pass

    def terminate(self, writer_id: int) -> None:
        with self._lock:
            path, buf = self._writers.pop(writer_id)
            self._files[path] = bytes(buf)

    def atomic_read(self, path: str) -> bytes:
        with self._lock:
            if path not in self._files:
                raise FileNotFoundError(f"File not found: {path}")
            return self._files[path]

    def atomic_write(self, path: str, data: bytes) -> None:
        with self._lock:
            self._files[path] = bytes(data)

    def exists(self, path: str) -> bool:
        with self._lock:
            return path in self._files

    def delete(self, path: str) -> None:
        with self._lock:
            self._files.pop(path, None)

    def sync_directory(self) -> None:
        pass


class TestPyDirectory:
    def test_create_index_with_directory(self):
        schema = (
            SchemaBuilder()
            .add_text_field("title", stored=True)
            .add_text_field("body")
            .build()
        )

        directory = InMemoryDirectory()
        index = Index(schema, directory=directory)

        writer = index.writer(15_000_000, 1)

        doc = Document()
        doc.add_text("title", "The Old Man and the Sea")
        doc.add_text(
            "body",
            "He was an old man who fished alone in a skiff in "
            "the Gulf Stream and he had gone eighty-four days "
            "now without taking a fish.",
        )
        writer.add_document(doc)

        writer.add_json(
            json.dumps(
                {
                    "title": "Of Mice and Men",
                    "body": (
                        "A few miles south of Soledad, the Salinas River drops "
                        "in close to the hillside bank and runs deep and green."
                    ),
                }
            )
        )

        writer.commit()
        writer.wait_merging_threads()

        index.reload()

        query = index.parse_query("sea", ["title", "body"])
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1

        _, doc_address = result.hits[0]
        searched_doc = index.searcher().doc(doc_address)
        assert searched_doc["title"] == ["The Old Man and the Sea"]

    def test_directory_has_files_after_commit(self):
        schema = (
            SchemaBuilder()
            .add_text_field("title", stored=True)
            .build()
        )

        directory = InMemoryDirectory()
        index = Index(schema, directory=directory)

        writer = index.writer(15_000_000, 1)
        doc = Document()
        doc.add_text("title", "Test document")
        writer.add_document(doc)
        writer.commit()
        writer.wait_merging_threads()

        # The directory should have some files after commit
        assert len(directory._files) > 0
        # meta.json should exist
        assert "meta.json" in directory._files

    def test_multiple_commits(self):
        schema = (
            SchemaBuilder()
            .add_text_field("title", stored=True)
            .build()
        )

        directory = InMemoryDirectory()
        index = Index(schema, directory=directory)

        # First commit
        writer = index.writer(15_000_000, 1)
        doc = Document()
        doc.add_text("title", "First document")
        writer.add_document(doc)
        writer.commit()
        writer.wait_merging_threads()
        index.reload()

        query = index.parse_query("first", ["title"])
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1

        # Second commit
        writer = index.writer(15_000_000, 1)
        doc = Document()
        doc.add_text("title", "Second document")
        writer.add_document(doc)
        writer.commit()
        writer.wait_merging_threads()
        index.reload()

        query = index.parse_query("second", ["title"])
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 1

        # Both documents should be searchable
        query = index.parse_query("document", ["title"])
        result = index.searcher().search(query, 10)
        assert len(result.hits) == 2
