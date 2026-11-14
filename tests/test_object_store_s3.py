"""Indexes stored in S3, tested against moto's S3 server.

moto implements the S3 API, including the conditional writes (If-None-Match
and If-Match) that the writer lock relies on, so these tests exercise the
real S3 client code path: request signing, ranged reads, multipart uploads
and lock handover. They are skipped when moto is not installed or the
extension was built without S3 support.
"""

import time
import uuid

import pytest

from tantivy import Document, Index, SchemaBuilder

moto_server = pytest.importorskip("moto.server")
boto3 = pytest.importorskip("boto3")

REGION = "us-east-1"
KEY_ID = "testing"
SECRET = "testing"


def _has_s3_support() -> bool:
    try:
        Index.exists("s3://probe/probe", storage_options={"endpoint": "http://127.0.0.1:1"})
    except ValueError as e:
        return "not supported by this build" not in str(e) and (
            "no object store support" not in str(e)
        )
    except OSError:
        return True
    return True


pytestmark = pytest.mark.skipif(
    not _has_s3_support(), reason="built without S3 support"
)


@pytest.fixture(scope="module")
def endpoint():
    server = moto_server.ThreadedMotoServer(ip_address="127.0.0.1", port=0)
    server.start()
    host, port = server.get_host_and_port()
    yield f"http://{host}:{port}"
    server.stop()


@pytest.fixture
def s3(endpoint):
    return boto3.client(
        "s3",
        endpoint_url=endpoint,
        region_name=REGION,
        aws_access_key_id=KEY_ID,
        aws_secret_access_key=SECRET,
    )


@pytest.fixture
def bucket(s3):
    name = f"tantivy-{uuid.uuid4().hex[:12]}"
    s3.create_bucket(Bucket=name)
    return name


@pytest.fixture
def uri(bucket):
    return f"s3://{bucket}/indexes/{uuid.uuid4().hex[:8]}"


@pytest.fixture
def options(endpoint):
    return {
        "endpoint": endpoint,
        "region": REGION,
        "aws_access_key_id": KEY_ID,
        "aws_secret_access_key": SECRET,
    }


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


def test_create_search_and_reopen(uri, options, s3, bucket):
    assert not Index.exists(uri, storage_options=options)
    index = Index(_schema(), path=uri, storage_options=options)
    _add(index, "old man and the sea", "of mice and men")
    index.reload()
    assert _titles(index, "sea") == ["old man and the sea"]

    assert Index.exists(uri, storage_options=options)
    assert Index.is_compatible(uri, storage_options=options)
    reopened = Index.open(uri, storage_options=options)
    assert _titles(reopened) == ["of mice and men", "old man and the sea"]

    keys = [o["Key"] for o in s3.list_objects_v2(Bucket=bucket)["Contents"]]
    assert any(k.endswith("/meta.json") for k in keys)


def test_large_segment_round_trips(uri, options):
    # Bodies large enough that segment files go through multipart upload and
    # the split, uncached read path.
    index = Index(_schema(), path=uri, storage_options=options)
    titles = [f"doc{i}" for i in range(400)]
    _add(index, *titles, body_size=20_000)
    reopened = Index.open(uri, storage_options=options)
    assert _titles(reopened, "filler") == sorted(titles)


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


def test_stale_lock_left_by_a_crash_is_taken_over(
    uri, options, s3, bucket, monkeypatch
):
    Index(_schema(), path=uri, storage_options=options)
    prefix = uri.split(f"s3://{bucket}/", 1)[1]
    # What a writer that crashed would leave behind.
    s3.put_object(Bucket=bucket, Key=f"{prefix}/.tantivy-writer.lock", Body=b"{}")

    monkeypatch.setenv("TANTIVY_OBJECT_STORE_LOCK_TIMEOUT_SECONDS", "2")
    index = Index.open(uri, storage_options=options)
    with pytest.raises(ValueError, match="(?i)lock"):
        index.writer(15_000_000, 1)
    time.sleep(2.5)
    writer = index.writer(15_000_000, 1)
    writer.add_document(Document(title="after takeover", body="body"))
    writer.commit()
    writer.wait_merging_threads()


def test_held_lock_is_not_taken_over(uri, options, monkeypatch):
    monkeypatch.setenv("TANTIVY_OBJECT_STORE_LOCK_TIMEOUT_SECONDS", "2")
    index = Index(_schema(), path=uri, storage_options=options)
    writer = index.writer(15_000_000, 1)
    # Well past the timeout: the holder's heartbeat keeps the lock fresh.
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
    # A separately opened index stands in for another process.
    reader_side = Index.open(uri, storage_options=options)
    assert _titles(reader_side) == ["one"]

    _add(writer_side, "two")
    deadline = time.time() + 10
    while _titles(reader_side) != ["one", "two"]:
        assert time.time() < deadline, "reader never saw the new commit"
        time.sleep(0.1)


def test_credential_provider_callable(uri, endpoint):
    calls = []

    def provider():
        calls.append(1)
        return {
            "aws_access_key_id": KEY_ID,
            "aws_secret_access_key": SECRET,
        }, int(time.time()) + 3600

    options = {"endpoint": endpoint, "region": REGION}
    index = Index(
        _schema(), path=uri, storage_options=options, credential_provider=provider
    )
    _add(index, "via provider")
    index.reload()
    assert _titles(index) == ["via provider"]
    # Cached: called once per credential lifetime, not once per request.
    assert len(calls) == 1


def test_credential_provider_errors_are_reported(uri, endpoint):
    def provider():
        raise RuntimeError("no credentials for you")

    with pytest.raises(ValueError, match="RuntimeError: no credentials for you"):
        Index.exists(
            uri,
            storage_options={"endpoint": endpoint, "region": REGION},
            credential_provider=provider,
        )


def test_credential_provider_aws_uses_boto3_chain(uri, endpoint, monkeypatch):
    from tantivy import CredentialProviderAWS

    monkeypatch.setenv("AWS_ACCESS_KEY_ID", KEY_ID)
    monkeypatch.setenv("AWS_SECRET_ACCESS_KEY", SECRET)
    monkeypatch.setenv("AWS_DEFAULT_REGION", REGION)
    provider = CredentialProviderAWS()
    credentials, expiry = provider()
    assert credentials["aws_access_key_id"] == KEY_ID
    assert expiry is None

    index = Index(
        _schema(),
        path=uri,
        storage_options={"endpoint": endpoint},
        credential_provider=provider,
    )
    _add(index, "boto3")
    index.reload()
    assert _titles(index) == ["boto3"]


def test_wrong_credentials_fail(uri, options, endpoint):
    Index(_schema(), path=uri, storage_options=options)
    # moto accepts any key pair, so check the provider's value is what gets
    # used by making it unparseable instead.
    with pytest.raises((OSError, ValueError), match="aws_secret_access_key"):
        Index.open(
            uri,
            storage_options={"endpoint": endpoint, "region": REGION},
            credential_provider=lambda: ({"aws_access_key_id": "x"}, None),
        )


def test_credential_provider_rejected_for_filesystem_path(tmp_path):
    with pytest.raises(ValueError, match="credential_provider"):
        Index(_schema(), path=str(tmp_path), credential_provider=lambda: ({}, None))
