"""Indexes stored in Azure Blob Storage, tested against Azurite.

Azurite is Microsoft's Azure Storage emulator. Like the S3 tests against moto,
these exercise the real Azure client: shared-key signing, ranged reads, block
uploads, and the conditional writes (If-None-Match / If-Match) that the
writer lock relies on.

They need the `azurite-blob` command (`npm install -g azurite`, or set
TANTIVY_TEST_AZURITE to its path) and the `azure-storage-blob` package, which
creates the test containers. Without either, they are skipped.
"""

import os
import shutil
import socket
import subprocess
import tempfile
import time
import uuid

import pytest

from tantivy import Document, Index, SchemaBuilder

blob = pytest.importorskip("azure.storage.blob")

AZURITE = os.environ.get("TANTIVY_TEST_AZURITE") or shutil.which("azurite-blob")
# Azurite's well-known development account.
ACCOUNT = "devstoreaccount1"
ACCOUNT_KEY = (
    "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/"
    "KBHBeksoGMGw=="
)

pytestmark = pytest.mark.skipif(AZURITE is None, reason="azurite-blob not installed")


def _free_port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


@pytest.fixture(scope="module")
def azurite_url():
    port = _free_port()
    workdir = tempfile.mkdtemp(prefix="azurite-")
    proc = subprocess.Popen(
        [
            AZURITE,
            "--silent",
            "--inMemoryPersistence",
            # Newer Azure SDKs send API versions Azurite doesn't list yet.
            "--skipApiVersionCheck",
            # Azurite otherwise sends usage telemetry to Microsoft.
            "--disableTelemetry",
            "--blobHost",
            "127.0.0.1",
            "--blobPort",
            str(port),
        ],
        # Azurite writes a debug log and settings into its working directory.
        cwd=workdir,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    deadline = time.time() + 30
    while True:
        try:
            socket.create_connection(("127.0.0.1", port), timeout=1).close()
            break
        except OSError:
            if proc.poll() is not None or time.time() > deadline:
                proc.kill()
                pytest.fail("Azurite did not start")
            time.sleep(0.2)
    yield f"http://127.0.0.1:{port}"
    proc.terminate()
    proc.wait(timeout=10)
    shutil.rmtree(workdir, ignore_errors=True)


@pytest.fixture(autouse=True)
def _point_at_azurite(azurite_url, monkeypatch):
    # object_store reads the emulator's address from this variable.
    monkeypatch.setenv("AZURITE_BLOB_STORAGE_URL", azurite_url)


@pytest.fixture
def container(azurite_url):
    name = f"tantivy-{uuid.uuid4().hex[:12]}"
    service = blob.BlobServiceClient(
        account_url=f"{azurite_url}/{ACCOUNT}",
        credential={"account_name": ACCOUNT, "account_key": ACCOUNT_KEY},
    )
    return service.create_container(name)


@pytest.fixture
def uri(container):
    return f"az://{container.container_name}/indexes/{uuid.uuid4().hex[:8]}"


OPTIONS = {"use_emulator": "true"}


@pytest.fixture
def account_options(azurite_url):
    """Azurite addressed as an ordinary account endpoint.

    In emulator mode object_store always signs with the emulator's key and
    ignores a credential_provider, so the credential tests use this instead.
    It is also the code path real Azure accounts take.
    """
    return {"account_name": ACCOUNT, "endpoint": f"{azurite_url}/{ACCOUNT}"}


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


def test_create_search_and_reopen(uri, container):
    assert not Index.exists(uri, storage_options=OPTIONS)
    index = Index(_schema(), path=uri, storage_options=OPTIONS)
    _add(index, "old man and the sea", "of mice and men")
    index.reload()
    assert _titles(index, "sea") == ["old man and the sea"]

    assert Index.exists(uri, storage_options=OPTIONS)
    assert Index.is_compatible(uri, storage_options=OPTIONS)
    reopened = Index.open(uri, storage_options=OPTIONS)
    assert _titles(reopened) == ["of mice and men", "old man and the sea"]
    names = [b.name for b in container.list_blobs()]
    assert any(n.endswith("/meta.json") for n in names)


def test_large_segment_round_trips(uri):
    index = Index(_schema(), path=uri, storage_options=OPTIONS)
    titles = [f"doc{i}" for i in range(400)]
    _add(index, *titles, body_size=20_000)
    reopened = Index.open(uri, storage_options=OPTIONS)
    assert _titles(reopened, "filler") == sorted(titles)


def test_only_one_writer_at_a_time(uri):
    index = Index(_schema(), path=uri, storage_options=OPTIONS)
    writer = index.writer(15_000_000, 1)
    other = Index.open(uri, storage_options=OPTIONS)
    with pytest.raises(ValueError, match="(?i)lock"):
        other.writer(15_000_000, 1)
    writer.commit()
    writer.wait_merging_threads()
    del writer
    other.writer(15_000_000, 1)


def test_stale_lock_left_by_a_crash_is_taken_over(uri, container, monkeypatch):
    Index(_schema(), path=uri, storage_options=OPTIONS)
    prefix = uri.split(f"az://{container.container_name}/", 1)[1]
    # What a writer that crashed would leave behind.
    container.upload_blob(f"{prefix}/.tantivy-writer.lock", b"{}")

    monkeypatch.setenv("TANTIVY_OBJECT_STORE_LOCK_TIMEOUT_SECONDS", "2")
    index = Index.open(uri, storage_options=OPTIONS)
    with pytest.raises(ValueError, match="(?i)lock"):
        index.writer(15_000_000, 1)
    time.sleep(2.5)
    writer = index.writer(15_000_000, 1)
    writer.add_document(Document(title="after takeover", body="body"))
    writer.commit()
    writer.wait_merging_threads()


def test_held_lock_is_not_taken_over(uri, monkeypatch):
    monkeypatch.setenv("TANTIVY_OBJECT_STORE_LOCK_TIMEOUT_SECONDS", "2")
    index = Index(_schema(), path=uri, storage_options=OPTIONS)
    writer = index.writer(15_000_000, 1)
    time.sleep(3)
    with pytest.raises(ValueError, match="(?i)lock"):
        Index.open(uri, storage_options=OPTIONS).writer(15_000_000, 1)
    writer.commit()


def test_reader_sees_other_writers_commits_without_reload(uri, monkeypatch):
    monkeypatch.setenv("TANTIVY_OBJECT_STORE_POLL_SECONDS", "0.2")
    writer_side = Index(_schema(), path=uri, storage_options=OPTIONS)
    _add(writer_side, "one")
    reader_side = Index.open(uri, storage_options=OPTIONS)
    assert _titles(reader_side) == ["one"]

    _add(writer_side, "two")
    deadline = time.time() + 10
    while _titles(reader_side) != ["one", "two"]:
        assert time.time() < deadline, "reader never saw the new commit"
        time.sleep(0.1)


def test_credential_provider_account_key(uri, account_options):
    calls = []

    def provider():
        calls.append(1)
        return {"account_key": ACCOUNT_KEY}, None

    index = Index(
        _schema(),
        path=uri,
        storage_options=account_options,
        credential_provider=provider,
    )
    _add(index, "via provider")
    index.reload()
    assert _titles(index) == ["via provider"]
    assert len(calls) == 1


def test_credential_provider_wrong_key_is_rejected(uri, account_options):
    Index(_schema(), path=uri, storage_options=OPTIONS)
    wrong = "A" * 86 + "=="
    with pytest.raises(ValueError):
        Index.open(
            uri,
            storage_options=account_options,
            credential_provider=lambda: ({"account_key": wrong}, None),
        )
