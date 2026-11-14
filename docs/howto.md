# How-to Guides

## Installation

tantivy-py can be installed using from [pypi](pypi.org) using pip:

    pip install tantivy

If no binary wheel is present for your operating system the bindings will be
build from source, this means that Rust needs to be installed before building
can succeed.

Note that the bindings are using [PyO3](https://github.com/PyO3/pyo3), which
only supports python3.

## Store an index in S3, Google Cloud Storage or Azure

Pass an object store URI wherever a path is accepted: `Index(schema, path=...)`,
`Index.open`, `Index.exists` and `Index.is_compatible`.

| Store | URI |
|-------|-----|
| Amazon S3 and S3-compatible stores (MinIO, R2, ...) | `s3://bucket/prefix` |
| Google Cloud Storage | `gs://bucket/prefix` |
| Azure Blob Storage | `az://container/prefix` or `abfss://container@account.dfs.core.windows.net/prefix` |
| In memory, shared within the process (for tests) | `memory:///prefix` |

Everything else works as it does for an index on disk:

```python
import tantivy

schema = tantivy.SchemaBuilder().add_text_field("title", stored=True).build()
index = tantivy.Index(schema, path="memory:///howto/books")

writer = index.writer()
writer.add_document(tantivy.Document(title="The Old Man and the Sea"))
writer.commit()
writer.wait_merging_threads()

index.reload()
searcher = index.searcher()
assert searcher.search(index.parse_query("sea", ["title"]), 1).count == 1
assert tantivy.Index.exists("memory:///howto/books")
```

### Credentials and settings

By default credentials and settings come from the provider's usual
environment variables (`AWS_ACCESS_KEY_ID`, `AWS_REGION`,
`GOOGLE_APPLICATION_CREDENTIALS`, `AZURE_STORAGE_ACCOUNT_NAME`, ...). For S3,
the `[default]` profile in `~/.aws/config` and `~/.aws/credentials` is also
read, and the bucket's region is looked up if none is set.

`storage_options` overrides them. The keys are those of the
[`object_store`](https://docs.rs/object_store) crate's
`AmazonS3ConfigKey`, `GoogleConfigKey` and `AzureConfigKey`, and an unknown key
raises `ValueError` rather than being ignored:

```py
index = tantivy.Index.open(
    "s3://my-bucket/indexes/books",
    storage_options={
        "endpoint": "http://localhost:9000",  # e.g. a local MinIO
        "region": "us-east-1",
        "aws_access_key_id": "...",
        "aws_secret_access_key": "...",
    },
)
```

For credentials that expire, such as AWS SSO, assumed roles or Google and
Azure tokens, pass a `credential_provider`. The built-in ones use each cloud's
own Python library, so whatever works with the cloud's CLI works here:

```py
from tantivy import CredentialProviderAWS, CredentialProviderAzure, CredentialProviderGCP

# boto3: profiles, SSO, environment, instance roles. Needs `pip install boto3`.
provider = CredentialProviderAWS(profile_name="prod")
# Assume a role with the session's credentials.
provider = CredentialProviderAWS(
    assume_role={"RoleArn": "arn:aws:iam::123456789012:role/search", "RoleSessionName": "tantivy"}
)
# google.auth.default(). Needs `pip install google-auth`.
provider = CredentialProviderGCP()
# azure.identity.DefaultAzureCredential(). Needs `pip install azure-identity`.
provider = CredentialProviderAzure()

index = tantivy.Index.open("s3://my-bucket/indexes/books", credential_provider=provider)
```

Any function returning `(credentials, expiry)` also works. `expiry` is a Unix
timestamp, or `None` for credentials that never expire, and the function is
called again shortly before it:

```py
def my_credentials():
    token = fetch_token_somehow()
    return {"bearer_token": token.value}, token.expires_at

index = tantivy.Index.open("gs://my-bucket/books", credential_provider=my_credentials)
```

The keys are `aws_access_key_id`, `aws_secret_access_key` and optionally
`aws_session_token` for S3; `bearer_token` for Google Cloud Storage; and one of
`bearer_token`, `account_key` or `sas_token` for Azure.

### How it behaves

- **One writer at a time.** The writer lock is an object created with a
  conditional write, so a second `index.writer()` anywhere raises
  `ValueError`, as it does on disk. The holder refreshes the lock in the
  background. If a writer crashes, its lock is taken over once it has gone
  unrefreshed for 2 minutes. The store must support conditional writes; S3,
  Google Cloud Storage, Azure and MinIO all do.
- **Readers see new commits.** With the default reload policy, an index
  notices commits from other processes within about a second, by checking
  `meta.json`. Call `index.reload()` to pick up a commit straight away.
- **Reads are cached.** Index files never change once written, so data read
  from the store is kept in an in-memory cache shared by every index in the
  process. Searches that touch the same parts of the index skip the network.
  The first search after opening an index is slower than on local disk.
- **Requests cost money.** Each uncached read is a GET request and each poll a
  HEAD request, both billed by most cloud providers.

These environment variables tune the behaviour:

| Variable | Default | Meaning |
|----------|---------|---------|
| `TANTIVY_OBJECT_STORE_CACHE_BYTES` | `268435456` (256 MiB) | Size of the read cache; `0` disables it. |
| `TANTIVY_OBJECT_STORE_POLL_SECONDS` | `1` | How often readers check for new commits; `0` disables it. |
| `TANTIVY_OBJECT_STORE_LOCK_TIMEOUT_SECONDS` | `120` | How long an unrefreshed lock lasts before it may be taken over. |
| `TANTIVY_HTTP_CONNECT_TIMEOUT_SECONDS` | `300` | Connection timeout for requests to the store. |

Object store support is built into the published wheels. When building from
source, it is controlled by the `cloud` Cargo feature (or `aws`, `gcp` and
`azure` for a single provider), which `pip install` and `maturin` enable
through `pyproject.toml`.

## Set up a development environment to work on tantivy-py itself

Since tantivy-py includes Rust code, you need a working Rust toolchain.
Install it via [rustup](https://rustup.rs/):

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Setting up a development environment can be done in a virtual environment using
[`nox`](https://nox.thea.codes) or using local packages using the provided `Makefile`.

For the `nox` setup install the virtual environment and build the bindings using:

    python3 -m pip install nox
    nox

For the `Makefile` based setup run:

    make

Running the tests is done using:

    make test

The `nox` test session will pass pytest arguments through. For example,
to run only the tests including "simple_search" in the test name, and only
on Python 3.11:

    nox -s test-3.11 -- -k simple_search

## Doctests

[Doctests](https://docs.python.org/3/library/doctest.html) are automatically
enabled for all docstrings in the `tantivy` module. Here is a very basic
introduction.  Consider the following hypothetical Rust `struct`:

```rust
/// Tantivy's Document is the object that can be indexed and then searched for.
///
/// Documents are fundamentally a collection of unordered tuples
/// (field_name, value). In this list, one field may appear more than once.
///
/// Example:
///     >>> doc = tantivy.Document()
///     >>> doc.add_text("title", "The Old Man and the Sea")
///     >>> doc.add_text("body", ("He was an old man who fished alone in a "
///     ...                       "skiff in the Gulf Stream and he had gone "
///     ...                       "eighty-four days now without taking a fish."))
///     >>> doc
///     Document(body=[He was an ],title=[The Old Ma])
///
#[pyclass(module = "tantivy")]
#[derive(Clone, Default, PartialEq)]
pub(crate) struct Document {
    pub(crate) field_values: BTreeMap<String, Vec<Value>>,
}
```

When the tests are executed, pytest will automatically search all the docstrings
for `>>>` and `...` and execute the code in the docstring. The output of the
code is compared to the text that follows the code. If the output matches, the
test passes. If the output does not match, the test fails.

In the above example, a Tantivy document object is created, and then the
representation of the document is printed. This representation, and indeed any
output that manual typing would produce, is compared to the text that follows
and this is how doctests work.

Doctests are a great way to ensure that the documentation is accurate and up to
date, and doctests are therefore encouraged be present on every public 
interface that users will interact with. However, doctest are not suitable 
for coverage testing and other more advanced testing methods so you must
judge when to use them.

## Working on tantivy-py documentation

Please be aware that this documentation is structured using the [Diátaxis](https://diataxis.fr/) framework. In very simple terms, this framework will suggest the correct location for different kinds of documentation. Please make sure you gain a basic understanding of the goals of the framework before making large pull requests with new documentation.

This documentation uses the [MkDocs](https://mkdocs.readthedocs.io/en/stable/) framework. This package is specified as an optional dependency in the `pyproject.toml` file. To install all optional dev dependencies into your virtual env, run the following command:

    pip install .[dev]

The [MkDocs](https://mkdocs.readthedocs.io/en/stable/) documentation itself is comprehensive. MkDocs provides some additional context and help around [writing with markdown](https://mkdocs.readthedocs.io/en/stable/user-guide/writing-your-docs/#writing-with-markdown).

If all you want to do is make a few edits right away, the documentation content is in the `/docs` directory and consists of [Markdown](https://www.markdownguide.org/) files, which can be edited with any text editor.

The most efficient way to work is to run a MkDocs livereload server in the background. This will launch a local web server on your dev machine, serve the docs (by default at `http://localhost:8000`), and automatically reload the page after you save any changes to the documentation files.
