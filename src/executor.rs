use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use piston_rs::ExecResponse as ExecResponse_;
use piston_rs::ExecResult as ExecResult_;
use piston_rs::Executor as Executor_;

use super::models::File;

#[pyclass]
#[derive(Clone)]
pub struct ExecResult {
    inner: ExecResult_,
}

#[pyproto]
impl PyObjectProtocol for ExecResult {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }
}

impl ExecResult {
    pub fn new(
        stdout: String,
        stderr: String,
        output: String,
        code: isize,
        signal: Option<String>,
    ) -> Self {
        Self {
            inner: ExecResult_ {
                stdout,
                stderr,
                output,
                code,
                signal,
            },
        }
    }

    pub fn from_inner(result: &ExecResult_) -> Self {
        Self {
            inner: result.clone(),
        }
    }
}

#[pymethods]
impl ExecResult {
    #[new]
    fn new_() -> PyResult<Self> {
        Err(PyTypeError::new_err("ExecResult can not be instantiated"))
    }

    #[getter]
    fn stdout(&self) -> String {
        self.inner.stdout.clone()
    }

    #[getter]
    fn stderr(&self) -> String {
        self.inner.stderr.clone()
    }

    #[getter]
    fn output(&self) -> String {
        self.inner.output.clone()
    }

    #[getter]
    fn code(&self) -> isize {
        self.inner.code
    }

    #[getter]
    fn signal(&self) -> Option<String> {
        self.inner.signal.clone()
    }

    /// True if there was a zero status code returned from execution.
    #[pyo3(text_signature = "($self) -> bool")]
    fn is_ok(&self) -> bool {
        self.inner.is_ok()
    }

    /// True if there was a non zero status code returned from execution.
    #[pyo3(text_signature = "($self) -> bool")]
    fn is_err(&self) -> bool {
        self.inner.is_err()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ExecResponse {
    inner: ExecResponse_,
}

#[pyproto]
impl PyObjectProtocol for ExecResponse {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }
}

#[pymethods]
impl ExecResponse {
    #[new]
    fn new_() -> PyResult<Self> {
        Err(PyTypeError::new_err("ExecResponse can not be instantiated"))
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
    fn run(&self) -> ExecResult {
        ExecResult::from_inner(&self.inner.run)
    }

    #[getter]
    fn compile(&self) -> Option<ExecResult> {
        if let Some(compile) = &self.inner.compile {
            Some(ExecResult::from_inner(compile))
        } else {
            None
        }
    }

    /// True if there was a non zero status code returned from execution.
    #[pyo3(text_signature = "($self) -> bool")]
    fn is_ok(&self) -> bool {
        self.inner.is_ok()
    }

    /// True if there was a non zero status code returned from execution.
    #[pyo3(text_signature = "($self) -> bool")]
    fn is_err(&self) -> bool {
        self.inner.is_err()
    }
}

#[pyclass]
#[pyo3(text_signature = "() -> bool")]
pub struct Executor {
    inner: Executor_,
}

#[pyproto]
impl PyObjectProtocol for Executor {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.inner))
    }

    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }
}

#[pymethods]
impl Executor {
    #[new]
    #[args(
        language = "\"\".to_string()",
        version = "\"\".to_string()",
        files = "vec![]",
        stdin = "\"\".to_string()",
        args = "vec![]",
        compile_timeout = "10000",
        run_timeout = "3000",
        compile_memory_limit = "-1",
        run_memory_limit = "-1"
    )]
    fn new(
        language: String,
        version: String,
        files: Vec<File>,
        stdin: String,
        args: Vec<String>,
        compile_timeout: isize,
        run_timeout: isize,
        compile_memory_limit: isize,
        run_memory_limit: isize,
    ) -> Self {
        Self {
            inner: Executor_ {
                language,
                version,
                files: files.iter().map(|f| f.convert()).collect(),
                stdin,
                args,
                compile_timeout,
                run_timeout,
                compile_memory_limit,
                run_memory_limit,
            },
        }
    }

    #[getter]
    fn language(&self) -> String {
        self.inner.language.clone()
    }

    #[setter]
    #[pyo3(name = "language")]
    fn language_setter(&mut self, language: String) {
        self.inner.language = language;
    }

    #[getter]
    fn version(&self) -> String {
        self.inner.version.clone()
    }

    #[setter]
    #[pyo3(name = "version")]
    fn version_setter(&mut self, version: String) {
        self.inner.version = version;
    }

    #[getter]
    fn files(&self) -> Vec<File> {
        self.inner
            .files
            .clone()
            .iter()
            .map(|f| File::from_inner(f))
            .collect()
    }

    #[setter]
    #[pyo3(name = "files")]
    fn files_setter(&mut self, files: Vec<File>) {
        self.inner
            .set_files(files.iter().map(|f| f.convert()).collect());
    }

    #[getter]
    fn stdin(&self) -> String {
        self.inner.stdin.clone()
    }

    #[setter]
    #[pyo3(name = "stdin")]
    fn stdin_setter(&mut self, stdin: String) {
        self.inner.stdin = stdin;
    }

    #[getter]
    fn args(&self) -> Vec<String> {
        self.inner.args.clone()
    }

    #[setter]
    #[pyo3(name = "args")]
    fn args_setter(&mut self, args: Vec<String>) {
        self.inner.args = args;
    }

    #[getter]
    fn compile_timeout(&self) -> isize {
        self.inner.compile_timeout
    }

    #[setter]
    #[pyo3(name = "compile_timeout")]
    fn compile_timeout_setter(&mut self, timeout: isize) {
        self.inner.compile_timeout = timeout;
    }

    #[getter]
    fn run_timeout(&self) -> isize {
        self.inner.compile_timeout
    }

    #[setter]
    #[pyo3(name = "run_timeout")]
    fn run_timeout_setter(&mut self, timeout: isize) {
        self.inner.run_timeout = timeout;
    }

    #[getter]
    fn compile_memory_limit(&self) -> isize {
        self.inner.compile_memory_limit
    }

    #[setter]
    #[pyo3(name = "compile_memory_limit")]
    fn compile_memory_limit_setter(&mut self, limit: isize) {
        self.inner.compile_memory_limit = limit;
    }

    #[getter]
    fn run_memory_limit(&self) -> isize {
        self.inner.run_memory_limit
    }

    #[setter]
    #[pyo3(name = "run_memory_limit")]
    fn run_memory_limit_setter(&mut self, limit: isize) {
        self.inner.run_memory_limit = limit;
    }
}
