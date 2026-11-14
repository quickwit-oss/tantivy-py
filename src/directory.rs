//! Turns the `path` a user passes to `Index` into a tantivy `Directory`.
//!
//! A plain filesystem path opens an `MmapDirectory`, as it always has. A URI
//! with a scheme (`s3://bucket/prefix`, `gs://...`, `az://...`,
//! `memory:///...`) opens an object store directory, which is only available
//! when the extension is built with the `cloud` feature (or one of the
//! `aws`, `gcp`, `azure` features).

use std::collections::HashMap;

use pyo3::{exceptions, PyResult};
use tantivy as tv;
use tantivy::directory::{Directory, MmapDirectory};

use crate::to_pyerr;

/// Open the directory that `path` refers to.
///
/// `storage_options` configures the object store (credentials, region,
/// endpoint, ...). It is an error to pass it with a filesystem path.
pub(crate) fn open_directory(
    path: &str,
    storage_options: Option<HashMap<String, String>>,
) -> PyResult<Box<dyn Directory>> {
    match uri_scheme(path) {
        None => {
            if storage_options.is_some() {
                return Err(exceptions::PyValueError::new_err(
                    "storage_options can only be used with an object store URI \
                     such as 's3://bucket/path', not a filesystem path",
                ));
            }
            // Convert through `TantivyError` so error messages match the ones
            // `Index::open_in_dir` produced before this function existed.
            let directory = MmapDirectory::open(path)
                .map_err(|e| to_pyerr(tv::TantivyError::from(e)))?;
            Ok(Box::new(directory))
        }
        Some(_scheme) => open_object_store(path, storage_options),
    }
}

#[cfg(feature = "object-store")]
fn open_object_store(
    uri: &str,
    storage_options: Option<HashMap<String, String>>,
) -> PyResult<Box<dyn Directory>> {
    let directory = crate::object_store_directory::ObjectStoreDirectory::open(
        uri,
        storage_options.unwrap_or_default(),
    )?;
    Ok(Box::new(directory))
}

#[cfg(not(feature = "object-store"))]
fn open_object_store(
    uri: &str,
    _storage_options: Option<HashMap<String, String>>,
) -> PyResult<Box<dyn Directory>> {
    Err(exceptions::PyValueError::new_err(format!(
        "Cannot open '{uri}': this build of tantivy has no object store \
         support. Install a build compiled with the `cloud` feature, or pass \
         a filesystem path."
    )))
}

/// The scheme of `path` if it is a URI such as `s3://bucket/key`, else `None`.
///
/// `file://` is deliberately not treated as a URI, so such paths behave as
/// they did before object store support existed. A single-letter scheme is
/// rejected so that Windows paths like `C://index` stay filesystem paths.
fn uri_scheme(path: &str) -> Option<&str> {
    let (scheme, _) = path.split_once("://")?;
    let mut chars = scheme.chars();
    let valid = scheme.len() > 1
        && chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c));
    (valid && !scheme.eq_ignore_ascii_case("file")).then_some(scheme)
}

#[cfg(test)]
mod tests {
    use super::uri_scheme;

    #[test]
    fn detects_object_store_uris() {
        assert_eq!(uri_scheme("s3://bucket/idx"), Some("s3"));
        assert_eq!(uri_scheme("gs://bucket"), Some("gs"));
        assert_eq!(uri_scheme("memory:///idx"), Some("memory"));
    }

    #[test]
    fn keeps_filesystem_paths() {
        assert_eq!(uri_scheme("/tmp/index"), None);
        assert_eq!(uri_scheme("relative/index"), None);
        assert_eq!(uri_scheme("C://index"), None);
        assert_eq!(uri_scheme("file:///tmp/index"), None);
        assert_eq!(uri_scheme("weird dir://x"), None);
    }
}
