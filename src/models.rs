use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use piston_rs::File as File_;
use piston_rs::Runtime as Runtime_;

#[pyclass]
#[pyo3(text_signature = "(language: str, version: str, aliases: list[str], /) -> Runtime")]
#[derive(Clone)]
pub struct Runtime {
    inner: Runtime_,
}

impl Runtime {
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

    #[getter]
    fn language(&self) -> String {
        self.inner.language.clone()
    }

    #[getter]
    fn version(&self) -> String {
        self.inner.version.clone()
    }

    #[getter]
    fn aliases(&self) -> Vec<String> {
        self.inner.aliases.clone()
    }

    #[pyo3(text_signature = "($self) -> Runtime")]
    fn copy(&self) -> Self {
        self.clone()
    }
}

#[pyclass]
#[pyo3(
    text_signature = "(name: str = \"\", content: str = \"\", encoding: str = \"utf8\", /) -> File"
)]
#[derive(Clone)]
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
    pub fn from_inner(inner: &File_) -> Self {
        Self {
            inner: inner.clone(),
        }
    }

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

    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[setter]
    #[pyo3(name = "name")]
    fn name_setter(&mut self, name: String) {
        self.inner.name = name;
    }

    #[getter]
    fn content(&self) -> String {
        self.inner.content.clone()
    }

    #[setter]
    #[pyo3(name = "content")]
    fn content_setter(&mut self, content: String) {
        self.inner.content = content;
    }

    #[getter]
    fn encoding(&self) -> String {
        self.inner.encoding.clone()
    }

    #[setter]
    #[pyo3(name = "encoding")]
    fn encoding_setter(&mut self, encoding: String) {
        self.inner.encoding = encoding;
    }

    #[pyo3(text_signature = "($self, name: str, /) -> $self")]
    fn set_name(mut slf: PyRefMut<Self>, name: String) -> PyRefMut<Self> {
        slf.inner.name = name;
        slf
    }

    #[pyo3(text_signature = "($self, content: str, /) -> $self")]
    fn set_content(mut slf: PyRefMut<Self>, content: String) -> PyRefMut<Self> {
        slf.inner.content = content;
        slf
    }

    #[pyo3(text_signature = "($self, encoding: str, /) -> $self")]
    fn set_encoding(mut slf: PyRefMut<Self>, encoding: String) -> PyRefMut<Self> {
        slf.inner.encoding = encoding;
        slf
    }

    #[pyo3(text_signature = "($self) -> File")]
    fn copy(&self) -> Self {
        self.clone()
    }
}
