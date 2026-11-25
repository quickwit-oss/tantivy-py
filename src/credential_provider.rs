//! Object store credentials supplied by a Python callable.
//!
//! Adapted from Polars (https://github.com/pola-rs/polars), file
//! `crates/polars-io/src/cloud/credential_provider.rs`. Copyright (c) 2025
//! Ritchie Vink, MIT License; see `licenses/polars-LICENSE`.
//!
//! The callable takes no arguments and returns `(credentials, expiry)`:
//! `credentials` is a dict whose keys depend on the provider (below), and
//! `expiry` is when they stop working, in seconds since the Unix epoch, or
//! `None` if they never expire. The result is cached until shortly before
//! `expiry`, so the callable runs once per credential lifetime, not per
//! request.
//!
//! | Store | Keys |
//! |-------|------|
//! | S3    | `aws_access_key_id`, `aws_secret_access_key`, `aws_session_token` (optional) |
//! | GCS   | `bearer_token` |
//! | Azure | one of `bearer_token`, `account_key`, `sas_token` |
//!
//! Differences from Polars:
//! - The callable runs on a blocking thread (`spawn_blocking`) rather than
//!   inside the async task, so waiting for the GIL never stalls a runtime
//!   worker that other requests are using.
//! - Credentials are refreshed `REFRESH_MARGIN` before they expire, not at
//!   the moment they expire, so a request is never signed with a credential
//!   that lapses while it is in flight.

use std::{
    fmt,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use object_store::CredentialProvider;
use pyo3::{
    exceptions::PyValueError, prelude::*, pybacked::PyBackedStr, types::PyDict,
};

/// Refresh credentials this many seconds before they expire.
const REFRESH_MARGIN_SECS: u64 = 60;

type Parse<C> = fn(&Bound<'_, PyDict>) -> PyResult<C>;

/// A Python callable wrapped as an `object_store` credential provider.
pub(crate) struct PyCredentialProvider<C> {
    func: Arc<Py<PyAny>>,
    parse: Parse<C>,
    /// The last credential and when it expires (seconds since the epoch).
    cache: tokio::sync::Mutex<Option<(Arc<C>, u64)>>,
}

impl<C> fmt::Debug for PyCredentialProvider<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PyCredentialProvider")
    }
}

impl<C> PyCredentialProvider<C> {
    pub(crate) fn new(func: Arc<Py<PyAny>>, parse: Parse<C>) -> Self {
        PyCredentialProvider {
            func,
            parse,
            cache: tokio::sync::Mutex::new(None),
        }
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn error(
    err: impl Into<Box<dyn std::error::Error + Send + Sync>>,
) -> object_store::Error {
    object_store::Error::Generic {
        store: "credential_provider",
        source: err.into(),
    }
}

/// Call the provider and parse its result.
fn call<C>(func: &Py<PyAny>, parse: Parse<C>) -> PyResult<(C, u64)> {
    Python::attach(|py| {
        let result = func.call0(py)?;
        let (credentials, expiry) = result
            .bind(py)
            .extract::<(Bound<'_, PyDict>, Option<u64>)>()
            .map_err(|_| {
                PyValueError::new_err(
                    "credential_provider must return a tuple of \
                     (dict[str, str], expiry: int | None)",
                )
            })?;
        Ok((parse(&credentials)?, expiry.unwrap_or(u64::MAX)))
    })
}

#[async_trait::async_trait]
impl<C: Send + Sync + 'static> CredentialProvider for PyCredentialProvider<C> {
    type Credential = C;

    async fn get_credential(&self) -> object_store::Result<Arc<C>> {
        let mut cache = self.cache.lock().await;
        if let Some((credential, expiry)) = &*cache {
            if now().saturating_add(REFRESH_MARGIN_SECS) < *expiry {
                return Ok(credential.clone());
            }
        }
        let func = self.func.clone();
        let parse = self.parse;
        let (credential, expiry) =
            tokio::task::spawn_blocking(move || call(&func, parse))
                .await
                .map_err(error)?
                // "RuntimeError: message" reads better in the eventual
                // Python exception than PyErr's debug form.
                .map_err(|e| {
                    error(format!("credential_provider failed: {e}"))
                })?;
        if expiry <= now() {
            return Err(error(format!(
                "credential_provider returned credentials that expired at \
                 {expiry}, which is in the past (now {})",
                now()
            )));
        }
        let credential = Arc::new(credential);
        *cache = Some((credential.clone(), expiry));
        Ok(credential)
    }
}

/// The `(key, value)` pairs of a credentials dict.
fn entries(dict: &Bound<'_, PyDict>) -> PyResult<Vec<(PyBackedStr, String)>> {
    dict.iter()
        .map(|(k, v)| Ok((k.extract::<PyBackedStr>()?, v.extract::<String>()?)))
        .collect()
}

fn unknown_key(store: &str, key: &str, valid: &str) -> PyErr {
    PyValueError::new_err(format!(
        "credential_provider returned unknown key '{key}' for {store}; \
         valid keys are: {valid}"
    ))
}

#[cfg(feature = "aws")]
pub(crate) fn parse_aws(
    dict: &Bound<'_, PyDict>,
) -> PyResult<object_store::aws::AwsCredential> {
    let mut credential = object_store::aws::AwsCredential {
        key_id: String::new(),
        secret_key: String::new(),
        token: None,
    };
    for (key, value) in entries(dict)? {
        match &*key {
            "aws_access_key_id" => credential.key_id = value,
            "aws_secret_access_key" => credential.secret_key = value,
            "aws_session_token" => credential.token = Some(value),
            other => {
                return Err(unknown_key(
                    "S3",
                    other,
                    "aws_access_key_id, aws_secret_access_key, \
                     aws_session_token",
                ))
            }
        }
    }
    if credential.key_id.is_empty() || credential.secret_key.is_empty() {
        return Err(PyValueError::new_err(
            "credential_provider for S3 must return aws_access_key_id and \
             aws_secret_access_key",
        ));
    }
    Ok(credential)
}

#[cfg(feature = "gcp")]
pub(crate) fn parse_gcp(
    dict: &Bound<'_, PyDict>,
) -> PyResult<object_store::gcp::GcpCredential> {
    let mut bearer = String::new();
    for (key, value) in entries(dict)? {
        match &*key {
            "bearer_token" => bearer = value,
            other => return Err(unknown_key("GCS", other, "bearer_token")),
        }
    }
    if bearer.is_empty() {
        return Err(PyValueError::new_err(
            "credential_provider for GCS must return bearer_token",
        ));
    }
    Ok(object_store::gcp::GcpCredential { bearer })
}

#[cfg(feature = "azure")]
pub(crate) fn parse_azure(
    dict: &Bound<'_, PyDict>,
) -> PyResult<object_store::azure::AzureCredential> {
    use object_store::azure::{AzureAccessKey, AzureCredential};

    const VALID: &str = "bearer_token, account_key, sas_token";
    let mut credential = None;
    for (key, value) in entries(dict)? {
        credential = Some(match &*key {
            "bearer_token" => AzureCredential::BearerToken(value),
            "account_key" => AzureCredential::AccessKey(
                AzureAccessKey::try_new(&value)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
            ),
            "sas_token" => AzureCredential::SASToken(split_sas(&value)?),
            other => return Err(unknown_key("Azure", other, VALID)),
        });
    }
    credential.ok_or_else(|| {
        PyValueError::new_err(format!(
            "credential_provider for Azure must return one of: {VALID}"
        ))
    })
}

/// Split a SAS token into its query pairs. Adapted from object_store's
/// `split_sas`, by way of Polars.
#[cfg(feature = "azure")]
fn split_sas(sas: &str) -> PyResult<Vec<(String, String)>> {
    let invalid = |why: &str| {
        // Never echo the token: it is a secret.
        PyValueError::new_err(format!("invalid sas_token: {why}"))
    };
    let mut pairs = Vec::new();
    for pair in sas
        .trim_start_matches('?')
        .split('&')
        .filter(|s| !s.trim().is_empty())
    {
        let (key, value) = pair
            .trim()
            .split_once('=')
            .ok_or_else(|| invalid("missing '='"))?;
        let decode = |s: &str| {
            percent_encoding::percent_decode_str(s)
                .decode_utf8()
                .map(|v| v.into_owned())
                .map_err(|_| invalid("not valid UTF-8"))
        };
        pairs.push((decode(key)?, decode(value)?));
    }
    if pairs.is_empty() {
        return Err(invalid("empty"));
    }
    Ok(pairs)
}

/// Store settings the provider object suggests, from its optional
/// `_storage_options()` method (e.g. the region of an AWS profile). They fill
/// in settings the user did not give, and never override them.
pub(crate) fn suggested_options(
    func: &Py<PyAny>,
) -> PyResult<Vec<(String, String)>> {
    Python::attach(|py| {
        let func = func.bind(py);
        if !func.hasattr("_storage_options")? {
            return Ok(Vec::new());
        }
        let options = func.call_method0("_storage_options")?;
        let options = options.cast::<PyDict>().map_err(|_| {
            PyValueError::new_err("_storage_options() must return a dict")
        })?;
        options
            .iter()
            .map(|(k, v)| Ok((k.extract::<String>()?, v.extract::<String>()?)))
            .collect()
    })
}

#[cfg(all(test, feature = "cloud"))]
mod tests {
    use super::*;

    fn provider<C: Send + Sync + 'static>(
        code: &std::ffi::CStr,
        parse: Parse<C>,
    ) -> PyCredentialProvider<C> {
        Python::initialize();
        let func = Python::attach(|py| {
            let globals = PyDict::new(py);
            py.run(code, Some(&globals), None).unwrap();
            globals.get_item("provider").unwrap().unwrap().unbind()
        });
        PyCredentialProvider::new(Arc::new(func), parse)
    }

    #[test]
    fn caches_until_shortly_before_expiry() {
        let p = provider(
            c"import time\ncalls = []\ndef provider():\n    calls.append(1)\n    return {'aws_access_key_id': f'k{len(calls)}', 'aws_secret_access_key': 's'}, int(time.time()) + 3600\n",
            parse_aws,
        );
        let rt = crate::object_store_directory::runtime();
        let first = rt.block_on(p.get_credential()).unwrap();
        let second = rt.block_on(p.get_credential()).unwrap();
        assert_eq!(first.key_id, "k1");
        assert_eq!(second.key_id, "k1", "second call served from the cache");
    }

    #[test]
    fn refreshes_credentials_about_to_expire() {
        let p = provider(
            c"import time\ncalls = []\ndef provider():\n    calls.append(1)\n    return {'bearer_token': f't{len(calls)}'}, int(time.time()) + 30\n",
            parse_gcp,
        );
        let rt = crate::object_store_directory::runtime();
        assert_eq!(rt.block_on(p.get_credential()).unwrap().bearer, "t1");
        // Expires within REFRESH_MARGIN_SECS, so it is fetched again.
        assert_eq!(rt.block_on(p.get_credential()).unwrap().bearer, "t2");
    }

    #[test]
    fn rejects_bad_results() {
        let rt = crate::object_store_directory::runtime();
        let p = provider(
            c"def provider():\n    return {'region': 'x'}, None\n",
            parse_aws,
        );
        let err = rt.block_on(p.get_credential()).unwrap_err().to_string();
        assert!(err.contains("unknown key 'region'"), "{err}");
        assert!(
            err.contains("credential_provider failed: ValueError"),
            "{err}"
        );

        let p = provider(c"def provider():\n    return 'nope'\n", parse_gcp);
        let err = rt.block_on(p.get_credential()).unwrap_err().to_string();
        assert!(err.contains("must return a tuple"), "{err}");

        let p = provider(
            c"def provider():\n    return {'bearer_token': 't'}, 1\n",
            parse_gcp,
        );
        let err = rt.block_on(p.get_credential()).unwrap_err().to_string();
        assert!(err.contains("in the past"), "{err}");
    }

    #[test]
    fn parses_azure_sas_token() {
        let pairs = split_sas("?sv=2024&sig=a%2Bb%3D&se=x").unwrap();
        assert_eq!(
            pairs,
            vec![
                ("sv".to_string(), "2024".to_string()),
                ("sig".to_string(), "a+b=".to_string()),
                ("se".to_string(), "x".to_string()),
            ]
        );
        assert!(split_sas("novalue").is_err());
    }
}
