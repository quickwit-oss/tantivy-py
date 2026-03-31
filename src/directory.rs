use std::fmt;
use std::io::{self, BufWriter, Write};
use std::ops::Range;
use std::path::Path;
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyBytes;
use tantivy::directory::error::{DeleteError, OpenReadError, OpenWriteError};
use tantivy::directory::{
    AntiCallToken, FileHandle, OwnedBytes, TerminatingWrite, WatchCallback,
    WatchHandle, WritePtr,
};
use tantivy::HasLen;

fn is_file_not_found(py: Python<'_>, err: &PyErr) -> bool {
    err.is_instance_of::<pyo3::exceptions::PyFileNotFoundError>(py)
}

// ---------------------------------------------------------------------------
// PyDirectory — wraps a Python object implementing the Directory protocol
// ---------------------------------------------------------------------------

pub(crate) struct PyDirectory {
    py_object: Py<PyAny>,
}

impl PyDirectory {
    pub fn new(py_object: Py<PyAny>) -> Self {
        Self { py_object }
    }
}

impl Clone for PyDirectory {
    fn clone(&self) -> Self {
        Python::attach(|py| Self {
            py_object: self.py_object.clone_ref(py),
        })
    }
}

impl fmt::Debug for PyDirectory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PyDirectory")
    }
}

impl tantivy::Directory for PyDirectory {
    fn get_file_handle(
        &self,
        path: &Path,
    ) -> Result<Arc<dyn FileHandle>, OpenReadError> {
        let path_str = path.to_string_lossy().to_string();

        let data: Vec<u8> = Python::attach(|py| {
            self.py_object
                .call_method1(py, "get_file_handle", (&path_str,))
                .and_then(|result| result.extract::<Vec<u8>>(py))
                .map_err(|e| {
                    if is_file_not_found(py, &e) {
                        OpenReadError::FileDoesNotExist(path.to_path_buf())
                    } else {
                        OpenReadError::IoError {
                            io_error: Arc::new(io::Error::other(e.to_string())),
                            filepath: path.to_path_buf(),
                        }
                    }
                })
        })?;

        let handle = PyFileHandle {
            data: OwnedBytes::new(data),
        };
        Ok(Arc::new(handle))
    }

    fn delete(&self, path: &Path) -> Result<(), DeleteError> {
        let path_str = path.to_string_lossy().to_string();

        Python::attach(|py| {
            self.py_object.call_method1(py, "delete", (path_str,))
        })
        .map_err(|e: PyErr| DeleteError::IoError {
            io_error: Arc::new(io::Error::other(e.to_string())),
            filepath: path.to_path_buf(),
        })?;
        Ok(())
    }

    fn exists(&self, path: &Path) -> Result<bool, OpenReadError> {
        let path_str = path.to_string_lossy().to_string();

        Python::attach(|py| {
            self.py_object
                .call_method1(py, "exists", (&path_str,))
                .and_then(|result| result.extract::<bool>(py))
        })
        .map_err(|e: PyErr| OpenReadError::IoError {
            io_error: Arc::new(io::Error::other(e.to_string())),
            filepath: path.to_path_buf(),
        })
    }

    fn open_write(&self, path: &Path) -> Result<WritePtr, OpenWriteError> {
        let path_str = path.to_string_lossy().to_string();

        let writer_id: u64 = Python::attach(|py| {
            self.py_object
                .call_method1(py, "open_write", (&path_str,))
                .and_then(|result| result.extract::<u64>(py))
        })
        .map_err(|e: PyErr| OpenWriteError::IoError {
            io_error: Arc::new(io::Error::other(e.to_string())),
            filepath: path.to_path_buf(),
        })?;

        let writer = Python::attach(|py| PyWritePtr {
            py_object: self.py_object.clone_ref(py),
            writer_id,
        });
        Ok(BufWriter::new(Box::new(writer)))
    }

    fn atomic_read(&self, path: &Path) -> Result<Vec<u8>, OpenReadError> {
        let path_str = path.to_string_lossy().to_string();

        Python::attach(|py| {
            self.py_object
                .call_method1(py, "atomic_read", (&path_str,))
                .and_then(|result| result.extract::<Vec<u8>>(py))
                .map_err(|e| {
                    if is_file_not_found(py, &e) {
                        OpenReadError::FileDoesNotExist(path.to_path_buf())
                    } else {
                        OpenReadError::IoError {
                            io_error: Arc::new(io::Error::other(e.to_string())),
                            filepath: path.to_path_buf(),
                        }
                    }
                })
        })
    }

    fn atomic_write(&self, path: &Path, data: &[u8]) -> io::Result<()> {
        let path_str = path.to_string_lossy().to_string();
        let data = data.to_vec();

        Python::attach(|py| {
            let py_bytes = PyBytes::new(py, &data);
            self.py_object.call_method1(
                py,
                "atomic_write",
                (path_str, py_bytes),
            )
        })
        .map_err(|e: PyErr| io::Error::other(e.to_string()))?;
        Ok(())
    }

    fn sync_directory(&self) -> io::Result<()> {
        Python::attach(|py| self.py_object.call_method0(py, "sync_directory"))
            .map_err(|e: PyErr| io::Error::other(e.to_string()))?;
        Ok(())
    }

    fn watch(
        &self,
        _watch_callback: WatchCallback,
    ) -> tantivy::Result<WatchHandle> {
        Ok(WatchHandle::empty())
    }
}

// ---------------------------------------------------------------------------
// PyFileHandle — in-memory file content returned by get_file_handle
// ---------------------------------------------------------------------------

struct PyFileHandle {
    data: OwnedBytes,
}

impl fmt::Debug for PyFileHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PyFileHandle(len={})", self.data.len())
    }
}

impl HasLen for PyFileHandle {
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl FileHandle for PyFileHandle {
    fn read_bytes(&self, range: Range<usize>) -> io::Result<OwnedBytes> {
        Ok(self.data.slice(range))
    }
}

// ---------------------------------------------------------------------------
// PyWritePtr — bridges Rust Write + TerminatingWrite to Python callbacks
// ---------------------------------------------------------------------------

struct PyWritePtr {
    py_object: Py<PyAny>,
    writer_id: u64,
}

impl Write for PyWritePtr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let data = buf.to_vec();
        let len = data.len();

        Python::attach(|py| {
            let py_bytes = PyBytes::new(py, &data);
            self.py_object
                .call_method1(py, "write", (self.writer_id, py_bytes))
        })
        .map_err(|e: PyErr| io::Error::other(e.to_string()))?;

        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        Python::attach(|py| {
            self.py_object.call_method1(py, "flush", (self.writer_id,))
        })
        .map_err(|e: PyErr| io::Error::other(e.to_string()))?;
        Ok(())
    }
}

impl TerminatingWrite for PyWritePtr {
    fn terminate_ref(&mut self, _: AntiCallToken) -> io::Result<()> {
        Python::attach(|py| {
            self.py_object
                .call_method1(py, "terminate", (self.writer_id,))
        })
        .map_err(|e: PyErr| io::Error::other(e.to_string()))?;
        Ok(())
    }
}
