use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use piston_rs::File as File_;
use piston_rs::Runtime as Runtime_;
use pyo3::types::PyType;

/// A runtime available to be used by Piston.
///
/// ### Note:
///
/// >> Runtimes are not meant to be created manually. Instead, they
/// >> should be fetched from Piston using `Client.fetch_runtimes` and
/// >> stored. The Python bindings for `piston_rs` do allow you to
/// >> instantiate the class, however.
#[pyclass]
#[derive(Clone)]
#[pyo3(text_signature = "(language: str, version: str, aliases: list[str], /) -> Runtime")]
pub struct Runtime {
    inner: Runtime_,
}

impl Runtime {
    /// Generates a new `Runtime` from a `piston_rs.Runtime`.
    pub fn from_runtime(runtime: Runtime_) -> Self {
        Self { inner: runtime }
    }
}

#[pyproto]
impl PyObjectProtocol for Runtime {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }
}

#[pymethods]
impl Runtime {
    #[new]
    fn new(language: String, version: String, aliases: Vec<String>) -> Self {
        Self {
            inner: Runtime_ {
                language,
                version,
                aliases,
            },
        }
    }

    /// `str`: The language.
    #[getter]
    fn language(&self) -> String {
        self.inner.language.clone()
    }

    /// `str`: The version of the language.
    #[getter]
    fn version(&self) -> String {
        self.inner.version.clone()
    }

    /// `list[str]`: The aliases of the language.
    #[getter]
    fn aliases(&self) -> Vec<String> {
        self.inner.aliases.clone()
    }

    /// Copies the runtime, leaving the existing one unchanged.
    ///
    /// ### Returns:
    ///
    /// - `Runtime`: A copy of the runtime.
    #[pyo3(text_signature = "(self) -> Runtime")]
    fn copy(&self) -> Self {
        self.clone()
    }
}

/// A file that contains the source code to be executed.
#[pyclass]
#[derive(Clone)]
#[pyo3(
    text_signature = "(name: str = \"\", content: str = \"\", encoding: str = \"utf8\", /) -> File"
)]
pub struct File {
    inner: File_,
}

#[pyproto]
impl PyObjectProtocol for File {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }
}

impl File {
    /// Generates a new `File` from the inner `piston_rs.File`.
    pub fn from_inner(inner: &File_) -> Self {
        Self {
            inner: inner.clone(),
        }
    }

    /// Converts the `File` into a `piston_rs.File`.
    pub fn convert(&self) -> File_ {
        File_ {
            name: self.inner.name.to_string(),
            content: self.inner.content.to_string(),
            encoding: self.inner.encoding.to_string(),
        }
    }
}

#[pymethods]
impl File {
    #[new]
    #[args(
        name = "\"\".to_string()",
        content = "\"\".to_string()",
        encoding = "\"utf8\".to_string()"
    )]
    fn new(name: String, content: String, encoding: String) -> Self {
        Self {
            inner: File_ {
                name,
                content,
                encoding,
            },
        }
    }

    /// `str`: The name of the file.
    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[setter(name)]
    fn name_setter(&mut self, name: String) {
        self.inner.name = name;
    }

    /// `str`: **Required by Piston** The content of the file.
    #[getter]
    fn content(&self) -> String {
        self.inner.content.clone()
    }

    #[setter(content)]
    fn content_setter(&mut self, content: String) {
        self.inner.content = content;
    }

    /// `str`: The encoding of the file.
    #[getter]
    fn encoding(&self) -> String {
        self.inner.encoding.clone()
    }

    #[setter(encoding)]
    fn encoding_setter(&mut self, encoding: String) {
        self.inner.encoding = encoding;
    }

    /// Sets the name of the file.
    ///
    /// ### Args:
    ///
    /// - name `str`:
    /// The name to use.
    ///
    /// ### Returns:
    ///
    /// - `File`: The file, for chained method calls.
    #[pyo3(text_signature = "(self, name: str, /) -> File")]
    fn set_name(mut slf: PyRefMut<Self>, name: String) -> PyRefMut<Self> {
        slf.inner.name = name;
        slf
    }

    /// Sets the content of the file.
    ///
    /// ### Args:
    ///
    /// - content `str`:
    /// The content to use.
    ///
    /// ### Returns:
    ///
    /// - `File`: The file, for chained method calls.
    #[pyo3(text_signature = "(self, content: str, /) -> File")]
    fn set_content(mut slf: PyRefMut<Self>, content: String) -> PyRefMut<Self> {
        slf.inner.content = content;
        slf
    }

    /// Sets the encoding of the file.
    ///
    /// ### Args:
    ///
    /// - encoding `str`:
    /// The encoding to use.
    ///
    /// ### Returns:
    ///
    /// - `File`: The file, for chained method calls.
    #[pyo3(text_signature = "(self, encoding: str, /) -> File")]
    fn set_encoding(mut slf: PyRefMut<Self>, encoding: String) -> PyRefMut<Self> {
        slf.inner.encoding = encoding;
        slf
    }

    /// Creates a new `File` from an existing file on disk.
    ///
    /// ### Args:
    ///
    /// - path `str`:
    /// The path to the file.
    ///
    /// ### Returns:
    ///
    /// - `File`: The new file.
    #[classmethod]
    #[pyo3(text_signature = "(cls, path: str, /) -> File")]
    fn load_from(_cls: &PyType, path: String) -> PyResult<Self> {
        match File_::load_from(path.as_str()) {
            Ok(file) => Ok(Python::with_gil(|_| Self { inner: file })),
            Err(err) => Err(Python::with_gil(|_| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("{:?}", err.details))
            })),
        }
    }

    /// Sets the content of the file to the contents of an existing
    /// file on disk.
    ///
    /// ### Args:
    ///
    /// - path `str`:
    /// The path to the file.
    ///
    /// ### Returns:
    ///
    /// - `File`: The file, for chained method calls.
    #[pyo3(text_signature = "(self, path: str, /) -> File")]
    fn load_content_from(mut slf: PyRefMut<Self>, path: String) -> PyResult<PyRefMut<Self>> {
        let file = slf.inner.clone();

        match file.load_content_from(path.as_str()) {
            Ok(file) => {
                slf.inner.content = file.content;
                Ok(slf)
            }
            Err(err) => Err(Python::with_gil(|_| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("{:?}", err.details))
            })),
        }
    }

    /// Copies the file, leaving the existing one unchanged.
    ///
    /// ### Returns:
    ///
    /// - `File`: A copy of the file.
    #[pyo3(text_signature = "(self) -> File")]
    fn copy(&self) -> Self {
        self.clone()
    }
}
