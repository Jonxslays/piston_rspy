use pyo3::prelude::*;

use piston_rs::File as File_;
use piston_rs::Runtime as Runtime_;

#[pyclass]
#[derive(Clone)]
pub struct Runtime {
    inner: Runtime_,
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
}

#[pyclass]
#[derive(Clone)]
pub struct File {
    inner: File_,
}

#[pymethods]
impl File {
    #[new]
    fn new(name: String, content: String, encoding: String) -> Self {
        Self {
            inner: File_ {
                name,
                content,
                encoding,
            },
        }
    }

    #[staticmethod]
    fn default() -> Self {
        Self {
            inner: File_::default(),
        }
    }

    #[getter]
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[getter]
    fn content(&self) -> String {
        self.inner.content.clone()
    }

    #[getter]
    fn encoding(&self) -> String {
        self.inner.encoding.clone()
    }
}
