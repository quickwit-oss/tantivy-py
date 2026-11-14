//! Builds the S3, GCS and Azure object stores from a URI and the user's
//! `storage_options`.
//!
//! Adapted from Polars (https://github.com/pola-rs/polars), file
//! `crates/polars-io/src/cloud/options.rs`: the HTTP client options, the
//! `~/.aws` file fallback, bucket region detection, and the S3 checksum and
//! payload-signing defaults. Copyright (c) 2025 Ritchie Vink, MIT License; see
//! `licenses/polars-LICENSE`.
//!
//! Deliberate differences from Polars:
//! - Unknown `storage_options` keys raise instead of being silently ignored.
//!   Polars ignores them because its dict also carries non-store settings;
//!   tantivy's does not, so an unknown key is almost always a typo.
//! - The `~/.aws` fallback reads only the `[default]` profile, and is skipped
//!   when `AWS_PROFILE` names another profile. Polars takes the first match in
//!   the file regardless of section, which can pick the wrong account's keys.
//! - Errors are returned instead of panicking on a bad environment variable.
//! - `object_store`'s retry defaults are kept. Polars's tighter defaults are
//!   tuned for its adaptive rate limiter, which is not ported.

use std::{collections::HashMap, sync::Arc, time::Duration};

use object_store::{ClientOptions, ObjectStore};
use pyo3::{exceptions, PyErr, PyResult};

/// Sent as the HTTP user agent on every request.
const USER_AGENT: &str = concat!("tantivy-py/", env!("CARGO_PKG_VERSION"));

/// Overrides the default 5 minute connection timeout.
const CONNECT_TIMEOUT_ENV: &str = "TANTIVY_HTTP_CONNECT_TIMEOUT_SECONDS";

fn value_error(context: &str, err: impl std::fmt::Display) -> PyErr {
    exceptions::PyValueError::new_err(format!("{context}: {err}"))
}

fn unknown_option(provider: &str, key: &str) -> PyErr {
    exceptions::PyValueError::new_err(format!(
        "Unknown storage option '{key}' for {provider}"
    ))
}

/// Parse untyped `storage_options` into a provider's typed config keys.
fn parse_options<K: std::str::FromStr>(
    provider: &str,
    options: HashMap<String, String>,
) -> PyResult<Vec<(K, String)>> {
    options
        .into_iter()
        .map(|(key, value)| {
            key.to_ascii_lowercase()
                .parse::<K>()
                .map(|k| (k, value))
                .map_err(|_| unknown_option(provider, &key))
        })
        .collect()
}

/// reqwest is built without a rustls crypto provider (see Cargo.toml), so
/// one must be installed before the first HTTP client is created. Ignoring
/// the error is correct: it only means a provider is already installed.
pub(crate) fn install_crypto_provider() {
    let _ = rustls::crypto::ring::default_provider().install_default();
}

/// HTTP client settings shared by every provider (from Polars'
/// `get_client_options`).
fn client_options() -> PyResult<ClientOptions> {
    install_crypto_provider();
    let connect_timeout = match std::env::var(CONNECT_TIMEOUT_ENV) {
        Ok(value) => value
            .parse::<u64>()
            .ok()
            .filter(|secs| *secs > 0)
            .ok_or_else(|| {
                value_error(
                    &format!("Invalid {CONNECT_TIMEOUT_ENV}"),
                    format!("expected a positive integer, got '{value}'"),
                )
            })?,
        Err(_) => 5 * 60,
    };
    Ok(ClientOptions::new()
        // No limit on how long a response body may take to download, so a
        // large segment read is not cut off part way through.
        .with_timeout_disabled()
        .with_connect_timeout(Duration::from_secs(connect_timeout))
        .with_user_agent(http::HeaderValue::from_static(USER_AGENT))
        // Allows `http://` endpoints such as a local MinIO or LocalStack.
        .with_allow_http(true))
}

#[cfg(feature = "aws")]
pub(crate) fn build_aws(
    uri: &str,
    options: HashMap<String, String>,
) -> PyResult<Arc<dyn ObjectStore>> {
    use object_store::aws::{AmazonS3Builder, AmazonS3ConfigKey, Checksum};

    let mut builder = AmazonS3Builder::from_env()
        .with_client_options(client_options()?)
        .with_url(uri);

    aws_files::fill_from_aws_files(&mut builder);

    for (key, value) in parse_options::<AmazonS3ConfigKey>("S3", options)? {
        builder = builder.with_config(key, value);
    }

    let has_region = builder
        .get_config_value(&AmazonS3ConfigKey::Region)
        .is_some()
        || builder
            .get_config_value(&AmazonS3ConfigKey::DefaultRegion)
            .is_some();
    if !has_region {
        if builder
            .get_config_value(&AmazonS3ConfigKey::Endpoint)
            .is_some()
        {
            // A custom endpoint (MinIO, R2, ...) still needs some region to
            // sign requests with. Polars issue #13042.
            builder =
                builder.with_config(AmazonS3ConfigKey::Region, "us-east-1");
        } else if let Some(bucket) =
            builder.get_config_value(&AmazonS3ConfigKey::Bucket)
        {
            if let Some(region) = bucket_region::detect(&bucket) {
                builder =
                    builder.with_config(AmazonS3ConfigKey::Region, region);
            }
        }
    }

    if builder
        .get_config_value(&AmazonS3ConfigKey::Checksum)
        .is_none()
    {
        // S3's default checksum, cheaper to compute than SHA256.
        builder = builder.with_checksum_algorithm(Checksum::CRC64NVME);
    }

    let store = builder
        // TLS already protects the body; skip hashing every upload.
        .with_unsigned_payload(true)
        .build()
        .map_err(|e| value_error("Could not configure S3", e))?;
    Ok(Arc::new(store))
}

#[cfg(feature = "gcp")]
pub(crate) fn build_gcp(
    uri: &str,
    options: HashMap<String, String>,
) -> PyResult<Arc<dyn ObjectStore>> {
    use object_store::gcp::{GoogleCloudStorageBuilder, GoogleConfigKey};

    let mut builder = GoogleCloudStorageBuilder::from_env()
        .with_client_options(client_options()?);
    for (key, value) in parse_options::<GoogleConfigKey>("GCS", options)? {
        builder = builder.with_config(key, value);
    }
    let store = builder
        .with_url(uri)
        .build()
        .map_err(|e| value_error("Could not configure GCS", e))?;
    Ok(Arc::new(store))
}

#[cfg(feature = "azure")]
pub(crate) fn build_azure(
    uri: &str,
    options: HashMap<String, String>,
) -> PyResult<Arc<dyn ObjectStore>> {
    use object_store::azure::{AzureConfigKey, MicrosoftAzureBuilder};

    // `from_env` may supply the storage account name.
    let mut builder = MicrosoftAzureBuilder::from_env()
        .with_client_options(client_options()?);
    for (key, value) in parse_options::<AzureConfigKey>("Azure", options)? {
        builder = builder.with_config(key, value);
    }
    let store = builder
        .with_url(uri)
        .build()
        .map_err(|e| value_error("Could not configure Azure", e))?;
    Ok(Arc::new(store))
}

/// Fallback to `~/.aws/config` and `~/.aws/credentials` for settings that the
/// environment did not provide (from Polars' `read_config`).
#[cfg(feature = "aws")]
mod aws_files {
    use std::path::PathBuf;

    use object_store::aws::{AmazonS3Builder, AmazonS3ConfigKey};

    pub(super) fn fill_from_aws_files(builder: &mut AmazonS3Builder) {
        match std::env::var("AWS_PROFILE") {
            Ok(profile) if profile != "default" => return,
            _ => {}
        }
        let Some(home) = std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
        else {
            return;
        };
        // `~/.aws/config` names the default profile `[default]` too.
        fill(
            builder,
            &home.join(".aws/config"),
            &[("region", AmazonS3ConfigKey::Region)],
        );
        // Only use the file's keys if the environment set none of them, so
        // a key from one source is never paired with a secret from another.
        let credential_keys = [
            AmazonS3ConfigKey::AccessKeyId,
            AmazonS3ConfigKey::SecretAccessKey,
            AmazonS3ConfigKey::Token,
        ];
        if credential_keys
            .iter()
            .all(|key| builder.get_config_value(key).is_none())
        {
            fill(
                builder,
                &home.join(".aws/credentials"),
                &[
                    ("aws_access_key_id", AmazonS3ConfigKey::AccessKeyId),
                    (
                        "aws_secret_access_key",
                        AmazonS3ConfigKey::SecretAccessKey,
                    ),
                    ("aws_session_token", AmazonS3ConfigKey::Token),
                ],
            );
        }
    }

    fn fill(
        builder: &mut AmazonS3Builder,
        path: &std::path::Path,
        keys: &[(&str, AmazonS3ConfigKey)],
    ) {
        if keys
            .iter()
            .all(|(_, key)| builder.get_config_value(key).is_some())
        {
            return;
        }
        let Ok(content) = std::fs::read_to_string(path) else {
            return;
        };
        let values = default_section(&content);
        for (name, key) in keys {
            if builder.get_config_value(key).is_some() {
                continue;
            }
            if let Some(value) = values.get(*name) {
                *builder = std::mem::take(builder).with_config(*key, value);
            }
        }
    }

    /// The `key = value` pairs of the `[default]` section of an INI file.
    pub(super) fn default_section(
        content: &str,
    ) -> std::collections::HashMap<String, String> {
        let mut in_default = false;
        let mut values = std::collections::HashMap::new();
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('[') {
                in_default = line == "[default]";
            } else if in_default && !line.starts_with(['#', ';']) {
                if let Some((key, value)) = line.split_once('=') {
                    values.insert(
                        key.trim().to_ascii_lowercase(),
                        value.trim().to_string(),
                    );
                }
            }
        }
        values
    }
}

/// Look up a bucket's region from the `x-amz-bucket-region` header that S3
/// returns even to unauthenticated requests (from Polars' `build_aws`).
#[cfg(feature = "aws")]
mod bucket_region {
    use std::{
        collections::HashMap,
        sync::{Mutex, OnceLock},
    };

    fn cache() -> &'static Mutex<HashMap<String, String>> {
        static CACHE: OnceLock<Mutex<HashMap<String, String>>> =
            OnceLock::new();
        CACHE.get_or_init(Default::default)
    }

    pub(super) fn detect(bucket: &str) -> Option<String> {
        if let Some(region) = cache().lock().unwrap().get(bucket) {
            return Some(region.clone());
        }
        let response = crate::object_store_directory::block_on(async {
            reqwest::Client::builder()
                .user_agent(super::USER_AGENT)
                .build()
                .ok()?
                .head(format!("https://{bucket}.s3.amazonaws.com"))
                .send()
                .await
                .ok()
        })?;
        let region = response
            .headers()
            .get("x-amz-bucket-region")?
            .to_str()
            .ok()?
            .to_string();
        cache()
            .lock()
            .unwrap()
            .insert(bucket.to_string(), region.clone());
        Some(region)
    }
}

#[cfg(all(test, feature = "aws"))]
mod tests {
    use super::aws_files::default_section;

    #[test]
    fn reads_only_the_default_profile() {
        let content = "\
[profile prod]
region = eu-west-1
aws_access_key_id = PROD

[default]
region = ap-southeast-2
; comment = ignored
aws_access_key_id = DEFAULT

[other]
aws_access_key_id = OTHER
";
        let values = default_section(content);
        assert_eq!(values.get("region").unwrap(), "ap-southeast-2");
        assert_eq!(values.get("aws_access_key_id").unwrap(), "DEFAULT");
        assert!(!values.contains_key("comment"));
    }
}
