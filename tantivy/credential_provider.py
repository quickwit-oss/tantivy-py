"""Credential providers for indexes stored in S3, Google Cloud Storage or Azure.

Pass one as ``credential_provider`` to ``Index``, ``Index.open``,
``Index.exists`` or ``Index.is_compatible``::

    from tantivy import CredentialProviderAWS, Index

    index = Index.open(
        "s3://bucket/index",
        credential_provider=CredentialProviderAWS(profile_name="prod"),
    )

Any callable taking no arguments and returning ``(credentials, expiry)`` also
works, where ``credentials`` is a ``dict[str, str]`` and ``expiry`` is when
they stop working, as seconds since the Unix epoch, or ``None`` if they never
expire. tantivy calls it again shortly before ``expiry``. The keys are:

- S3: ``aws_access_key_id``, ``aws_secret_access_key``, and optionally
  ``aws_session_token``.
- GCS: ``bearer_token``.
- Azure: one of ``bearer_token``, ``account_key`` or ``sas_token``.

Adapted from Polars (https://github.com/pola-rs/polars), file
``py-polars/src/polars/io/cloud/credential_provider/_providers.py``.
Copyright (c) 2025 Ritchie Vink, MIT License; see ``licenses/polars-LICENSE``
in the tantivy-py source distribution. Differences from Polars: no Polars
logging or unstable-API warnings, cached credentials are refreshed five
minutes before they expire rather than at expiry, and ``CredentialProviderAWS``
reports its profile's region through ``_storage_options()``.
"""

from __future__ import annotations

import abc
import importlib.util
import threading
import time
from collections.abc import Callable
from datetime import datetime, timezone
from typing import Any, TypedDict, Union

__all__ = [
    "CredentialProvider",
    "CredentialProviderAWS",
    "CredentialProviderAzure",
    "CredentialProviderFunction",
    "CredentialProviderFunctionReturn",
    "CredentialProviderGCP",
]

CredentialProviderFunctionReturn = tuple[dict[str, str], Union[int, None]]
"""The ``(credentials, expiry)`` pair a credential provider returns."""

CredentialProviderFunction = Union[
    Callable[[], CredentialProviderFunctionReturn], "CredentialProvider"
]
"""Anything accepted as ``credential_provider``."""

# Fetch new credentials this long before the cached ones expire. Longer than
# the margin the Rust side uses (60 s), so credentials handed over are always
# valid for at least that long.
_REFRESH_MARGIN_SECONDS = 5 * 60


class AWSAssumeRoleKWArgs(TypedDict, total=False):
    """Parameters for `STS.Client.assume_role()`.

    See https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/sts/client/assume_role.html
    """

    RoleArn: str
    RoleSessionName: str
    PolicyArns: list[dict[str, str]]
    Policy: str
    DurationSeconds: int
    Tags: list[dict[str, str]]
    TransitiveTagKeys: list[str]
    ExternalId: str
    SerialNumber: str
    TokenCode: str
    SourceIdentity: str
    ProvidedContexts: list[dict[str, str]]


class CredentialProvider(abc.ABC):
    """Base class for credential providers.

    Subclasses implement ``retrieve_credentials_impl``. Results are cached and
    reused until five minutes before they expire.
    """

    def __init__(self) -> None:
        self._cached: CredentialProviderFunctionReturn | None = None
        self._lock = threading.Lock()

    def __call__(self) -> CredentialProviderFunctionReturn:
        with self._lock:
            cached = self._cached
            if cached is None or (
                cached[1] is not None
                and cached[1] - _REFRESH_MARGIN_SECONDS <= int(time.time())
            ):
                cached = self.retrieve_credentials_impl()
                self._cached = cached
        credentials, expiry = cached
        return {**credentials}, expiry

    @abc.abstractmethod
    def retrieve_credentials_impl(self) -> CredentialProviderFunctionReturn:
        """Fetch new credentials, bypassing the cache."""

    def clear_cached_credentials(self) -> None:
        """Forget cached credentials, so the next call fetches new ones."""
        with self._lock:
            self._cached = None

    def __getstate__(self) -> dict[str, Any]:
        # Credentials are secrets: never pickle them. Locks can't be pickled.
        state = self.__dict__.copy()
        state["_cached"] = None
        del state["_lock"]
        return state

    def __setstate__(self, state: dict[str, Any]) -> None:
        self.__dict__.update(state)
        self._lock = threading.Lock()

    def __repr__(self) -> str:
        return f"{type(self).__name__}()"


def _require(module: str, package: str, provider: str) -> None:
    if importlib.util.find_spec(module) is None:
        raise ImportError(f"{package} must be installed to use {provider}")


class CredentialProviderAWS(CredentialProvider):
    """AWS credentials from boto3: profiles, SSO, assume-role, instance roles.

    Uses boto3's credential chain, so everything the AWS CLI supports works,
    including ``AWS_PROFILE`` and SSO sessions. Requires ``boto3``.

    Args:
        profile_name: Profile to use from ``~/.aws/config``.
        region_name: Region to use; defaults to the profile's region.
        assume_role: If given, assume this role with the session's
            credentials. The keys are passed to ``STS.Client.assume_role()``,
            e.g. ``{"RoleArn": "...", "RoleSessionName": "tantivy"}``.
    """

    def __init__(
        self,
        *,
        profile_name: str | None = None,
        region_name: str | None = None,
        assume_role: AWSAssumeRoleKWArgs | None = None,
    ) -> None:
        _require("boto3", "boto3", "CredentialProviderAWS")
        self.profile_name = profile_name
        self.region_name = region_name
        self.assume_role = assume_role
        super().__init__()

    def retrieve_credentials_impl(self) -> CredentialProviderFunctionReturn:
        session = self._session()
        if self.assume_role is not None:
            return self._assume_role(session)

        creds = session.get_credentials()
        if creds is None:
            raise RuntimeError(
                "boto3 found no AWS credentials; configure a profile, "
                "environment variables or an instance role"
            )
        # Read the keys before the expiry: for refreshable credentials,
        # reading them is what triggers the refresh that sets the expiry.
        frozen = creds.get_frozen_credentials()
        credentials = {
            "aws_access_key_id": frozen.access_key,
            "aws_secret_access_key": frozen.secret_key,
        }
        if frozen.token is not None:
            credentials["aws_session_token"] = frozen.token
        expiry = getattr(creds, "_expiry_time", None)
        return credentials, (
            int(expiry.timestamp()) if isinstance(expiry, datetime) else None
        )

    def _assume_role(self, session: Any) -> CredentialProviderFunctionReturn:
        assert self.assume_role is not None
        response = session.client("sts").assume_role(**self.assume_role)
        creds = response["Credentials"]
        expiry = creds["Expiration"]
        if expiry.tzinfo is None:
            raise ValueError("STS returned an expiration time with no time zone")
        return {
            "aws_access_key_id": creds["AccessKeyId"],
            "aws_secret_access_key": creds["SecretAccessKey"],
            "aws_session_token": creds["SessionToken"],
        }, int(expiry.timestamp())

    def _storage_options(self) -> dict[str, str]:
        """Settings tantivy uses unless storage_options gives them.

        Called from Rust when the store is built: passes on the profile's
        region, which object_store cannot read for a named profile.
        """
        region = self.region_name or self._session().region_name
        return {"region": region} if region else {}

    def _session(self) -> Any:
        import boto3

        # boto3 reads AWS_PROFILE itself when profile_name is None.
        return boto3.Session(
            profile_name=self.profile_name, region_name=self.region_name
        )

    def __repr__(self) -> str:
        return f"CredentialProviderAWS(profile_name={self.profile_name!r})"


class CredentialProviderGCP(CredentialProvider):
    """Google Cloud credentials from ``google.auth.default()``.

    Covers application default credentials, ``gcloud auth``, service account
    files (``GOOGLE_APPLICATION_CREDENTIALS``) and workload identity.
    Requires ``google-auth``. The arguments are passed to
    ``google.auth.default()``.
    """

    def __init__(
        self,
        *,
        scopes: Any | None = None,
        request: Any | None = None,
        quota_project_id: Any | None = None,
        default_scopes: Any | None = None,
    ) -> None:
        _require("google.auth", "google-auth", "CredentialProviderGCP")
        self.scopes = (
            scopes
            if scopes is not None
            else ["https://www.googleapis.com/auth/cloud-platform"]
        )
        self.request = request
        self.quota_project_id = quota_project_id
        self.default_scopes = default_scopes
        super().__init__()

    def retrieve_credentials_impl(self) -> CredentialProviderFunctionReturn:
        import google.auth
        import google.auth.transport.requests

        creds, _project_id = google.auth.default(
            scopes=self.scopes,
            request=self.request,
            quota_project_id=self.quota_project_id,
            default_scopes=self.default_scopes,
        )
        creds.refresh(google.auth.transport.requests.Request())
        expiry = creds.expiry
        if expiry is not None and expiry.tzinfo is None:
            # google-auth reports naive UTC times.
            expiry = expiry.replace(tzinfo=timezone.utc)
        return {"bearer_token": creds.token}, (
            int(expiry.timestamp()) if expiry is not None else None
        )


class CredentialProviderAzure(CredentialProvider):
    """Azure credentials from ``azure.identity``.

    By default uses ``DefaultAzureCredential()``, which covers environment
    variables, managed identity and ``az login``. Requires ``azure-identity``
    unless ``credential`` is given.

    Args:
        scopes: Scopes to request a token for.
        tenant_id: Azure tenant ID.
        credential: An Azure credential object to use instead, such as
            ``azure.identity.ClientSecretCredential(...)``. It must have a
            ``get_token()`` method.
    """

    def __init__(
        self,
        *,
        scopes: list[str] | None = None,
        tenant_id: str | None = None,
        credential: Any | None = None,
    ) -> None:
        if credential is None:
            _require("azure.identity", "azure-identity", "CredentialProviderAzure")
        elif not hasattr(credential, "get_token"):
            raise ValueError(
                f"credential {credential!r} does not have a get_token() method"
            )
        self.scopes = (
            scopes if scopes is not None else ["https://storage.azure.com/.default"]
        )
        self.tenant_id = tenant_id
        self.credential = credential
        super().__init__()

    def retrieve_credentials_impl(self) -> CredentialProviderFunctionReturn:
        credential = self.credential
        if credential is None:
            import azure.identity

            credential = azure.identity.DefaultAzureCredential()
        token = credential.get_token(*self.scopes, tenant_id=self.tenant_id)
        return {"bearer_token": token.token}, token.expires_on
