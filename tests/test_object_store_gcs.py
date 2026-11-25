"""Indexes stored in Google Cloud Storage, tested against fake-gcs-server.

object_store writes through the GCS XML API, which upstream fake-gcs-server
does not accept (fsouza/fake-gcs-server#1164), so these tests use the patched
fork object_store's own CI uses: the `tustvold/fake-gcs-server` image, or a
binary built from github.com/tustvold/fake-gcs-server at commit 46f2f72
(branch support-xml-api).

Point them at it with TANTIVY_TEST_FAKE_GCS (path to the binary, started per
test module) or TANTIVY_TEST_FAKE_GCS_URL (an already running server whose
-public-host matches the URL's host, as in CI). Without either they skip.

The emulator does not honour generation preconditions on uploads
(fsouza/fake-gcs-server#994) or report the new generation, so the writer
lock cannot work against it; those tests are skipped here, as they are in
object_store's own GCS tests. Real GCS supports both.
"""

import json
import os
import socket
import subprocess
import time
import urllib.parse
import urllib.request
import uuid

import pytest

from tantivy import Document, Index, SchemaBuilder

FAKE_GCS = os.environ.get("TANTIVY_TEST_FAKE_GCS")

pytestmark = pytest.mark.skipif(
    FAKE_GCS is None and not os.environ.get("TANTIVY_TEST_FAKE_GCS_URL"),
    reason="fake-gcs-server not configured",
)

needs_preconditions = pytest.mark.skip(
    reason="fake-gcs-server ignores generation preconditions on uploads "
    "(fsouza/fake-gcs-server#994), which the writer lock needs"
)


def _free_port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


@pytest.fixture(scope="module")
def gcs_url():
    # An already running server (e.g. a CI service container) takes priority.
    if url := os.environ.get("TANTIVY_TEST_FAKE_GCS_URL"):
        yield url.rstrip("/")
        return
    port = _free_port()
    # The emulator only treats requests as XML API calls (the ones
    # object_store makes) when their Host header matches -public-host.
    url = f"http://localhost:{port}"
    proc = subprocess.Popen(
        [
            FAKE_GCS,
            "-scheme", "http",
            "-host", "127.0.0.1",
            "-port", str(port),
            "-backend", "memory",
            "-external-url", url,
            "-public-host", f"localhost:{port}",
            "-log-level", "error",
        ],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    deadline = time.time() + 30
    while True:
        try:
            urllib.request.urlopen(f"{url}/storage/v1/b", timeout=1).close()
            break
        except OSError:
            if proc.poll() is not None or time.time() > deadline:
                proc.kill()
                pytest.fail("fake-gcs-server did not start")
            time.sleep(0.2)
    yield url
    proc.terminate()
    proc.wait(timeout=10)


def _request(method, url, body=None, content_type="application/json"):
    data = body if isinstance(body, bytes) or body is None else json.dumps(body).encode()
    req = urllib.request.Request(url, data=data, method=method)
    if data is not None:
        req.add_header("Content-Type", content_type)
    with urllib.request.urlopen(req, timeout=10) as resp:
        return json.loads(resp.read() or b"{}")


@pytest.fixture
def bucket(gcs_url):
    name = f"tantivy-{uuid.uuid4().hex[:12]}"
    _request("POST", f"{gcs_url}/storage/v1/b", {"name": name})
    return name


@pytest.fixture
def uri(bucket):
    return f"gs://{bucket}/indexes/{uuid.uuid4().hex[:8]}"


@pytest.fixture
def options(gcs_url):
    # A service account that tells object_store to skip OAuth: the emulator
    # accepts unauthenticated requests.
    key = {
        "private_key": "",
        "private_key_id": "",
        "client_email": "",
        "disable_oauth": True,
    }
    return {"base_url": gcs_url, "service_account_key": json.dumps(key)}


def _list(gcs_url, bucket):
    listing = _request("GET", f"{gcs_url}/storage/v1/b/{bucket}/o")
    return [item["name"] for item in listing.get("items", [])]


def _upload(gcs_url, bucket, name, data):
    query = urllib.parse.urlencode({"uploadType": "media", "name": name})
    _request(
        "POST",
        f"{gcs_url}/upload/storage/v1/b/{bucket}/o?{query}",
        data,
        "application/octet-stream",
    )


def _schema():
    return (
        SchemaBuilder()
        .add_text_field("title", stored=True)
        .add_text_field("body")
        .build()
    )


def _add(index, *titles, body_size=0):
    writer = index.writer(15_000_000, 1)
    for title in titles:
        body = f"body of {title} " + "filler " * body_size
        writer.add_document(Document(title=title, body=body))
    writer.commit()
    writer.wait_merging_threads()


def _titles(index, query="body"):
    searcher = index.searcher()
    result = searcher.search(index.parse_query(query, ["title", "body"]), 1000)
    return sorted(searcher.doc(addr)["title"][0] for _, addr in result.hits)


def test_create_search_and_reopen(uri, options, gcs_url, bucket):
    assert not Index.exists(uri, storage_options=options)
    index = Index(_schema(), path=uri, storage_options=options)
    _add(index, "old man and the sea", "of mice and men")
    index.reload()
    assert _titles(index, "sea") == ["old man and the sea"]

    assert Index.exists(uri, storage_options=options)
    assert Index.is_compatible(uri, storage_options=options)
    reopened = Index.open(uri, storage_options=options)
    assert _titles(reopened) == ["of mice and men", "old man and the sea"]
    assert any(n.endswith("/meta.json") for n in _list(gcs_url, bucket))


def test_large_segment_round_trips(uri, options):
    index = Index(_schema(), path=uri, storage_options=options)
    titles = [f"doc{i}" for i in range(400)]
    _add(index, *titles, body_size=20_000)
    reopened = Index.open(uri, storage_options=options)
    assert _titles(reopened, "filler") == sorted(titles)


@needs_preconditions
def test_only_one_writer_at_a_time(uri, options):
    index = Index(_schema(), path=uri, storage_options=options)
    writer = index.writer(15_000_000, 1)
    other = Index.open(uri, storage_options=options)
    with pytest.raises(ValueError, match="(?i)lock"):
        other.writer(15_000_000, 1)
    writer.commit()
    writer.wait_merging_threads()
    del writer
    other.writer(15_000_000, 1)


@needs_preconditions
def test_stale_lock_left_by_a_crash_is_taken_over(
    uri, options, gcs_url, bucket, monkeypatch
):
    Index(_schema(), path=uri, storage_options=options)
    prefix = uri.split(f"gs://{bucket}/", 1)[1]
    # What a writer that crashed would leave behind.
    _upload(gcs_url, bucket, f"{prefix}/.tantivy-writer.lock", b"{}")

    monkeypatch.setenv("TANTIVY_OBJECT_STORE_LOCK_TIMEOUT_SECONDS", "2")
    index = Index.open(uri, storage_options=options)
    with pytest.raises(ValueError, match="(?i)lock"):
        index.writer(15_000_000, 1)
    time.sleep(2.5)
    writer = index.writer(15_000_000, 1)
    writer.add_document(Document(title="after takeover", body="body"))
    writer.commit()
    writer.wait_merging_threads()


@needs_preconditions
def test_held_lock_is_not_taken_over(uri, options, monkeypatch):
    monkeypatch.setenv("TANTIVY_OBJECT_STORE_LOCK_TIMEOUT_SECONDS", "2")
    index = Index(_schema(), path=uri, storage_options=options)
    writer = index.writer(15_000_000, 1)
    time.sleep(3)
    with pytest.raises(ValueError, match="(?i)lock"):
        Index.open(uri, storage_options=options).writer(15_000_000, 1)
    writer.commit()


def test_reader_sees_other_writers_commits_without_reload(
    uri, options, monkeypatch
):
    monkeypatch.setenv("TANTIVY_OBJECT_STORE_POLL_SECONDS", "0.2")
    writer_side = Index(_schema(), path=uri, storage_options=options)
    _add(writer_side, "one")
    reader_side = Index.open(uri, storage_options=options)
    assert _titles(reader_side) == ["one"]

    _add(writer_side, "two")
    deadline = time.time() + 10
    while _titles(reader_side) != ["one", "two"]:
        assert time.time() < deadline, "reader never saw the new commit"
        time.sleep(0.1)


def test_credential_provider_bearer_token(uri, gcs_url):
    calls = []

    def provider():
        calls.append(1)
        return {"bearer_token": "token-from-python"}, int(time.time()) + 3600

    index = Index(
        _schema(),
        path=uri,
        storage_options={"base_url": gcs_url},
        credential_provider=provider,
    )
    _add(index, "via provider")
    index.reload()
    assert _titles(index) == ["via provider"]
    assert len(calls) == 1


def test_credential_provider_bad_result_is_reported(uri, gcs_url):
    with pytest.raises(ValueError, match="bearer_token"):
        Index.exists(
            uri,
            storage_options={"base_url": gcs_url},
            credential_provider=lambda: ({"token": "x"}, None),
        )
