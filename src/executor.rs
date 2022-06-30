use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;

use piston_rs::ExecResponse as ExecResponse_;
use piston_rs::ExecResult as ExecResult_;
use piston_rs::Executor as Executor_;

use super::File;

/// The result of code execution returned by Piston.
///
/// **NOTE**:
///
/// - This object cannot be instantiated, and is immutable.
/// - It can only be created with a call to `Client.execute`.
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
    /// Creates a new executor.
    pub fn new(
        stdout: String,
        stderr: String,
        output: String,
        code: Option<isize>,
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

    /// Creates a new ExecResult, from a `piston_rs.ExecResult`.
    pub fn from_result(result: &ExecResult_) -> Self {
        Self {
            inner: result.clone(),
        }
    }
}

#[pymethods]
impl ExecResult {
    #[new]
    /// Raises a TypeError because this class cannot be instantiated.
    fn new_() -> PyResult<Self> {
        Err(PyTypeError::new_err("ExecResult can not be instantiated"))
    }

    /// `str`: The text sent to `stdout` during execution.
    #[getter]
    fn stdout(&self) -> String {
        self.inner.stdout.clone()
    }

    /// `str`: The text sent to `stderr` during execution.
    #[getter]
    fn stderr(&self) -> String {
        self.inner.stderr.clone()
    }

    /// `str`: The text sent to both `stdout`, and `stderr` during execution.
    #[getter]
    fn output(&self) -> String {
        self.inner.output.clone()
    }

    /// `int | None`: The optional exit code returned by the process.
    #[getter]
    fn code(&self) -> Option<isize> {
        self.inner.code
    }

    /// `str` | `None`: The optional signal sent to the process. (`SIGKILL` etc)
    #[getter]
    fn signal(&self) -> Option<String> {
        self.inner.signal.clone()
    }

    /// Whether or not the execution was ok.
    ///
    /// ### Returns:
    ///
    /// - `bool`: `True` if the execution returned a zero exit code.
    #[pyo3(text_signature = "(self) -> bool")]
    fn is_ok(&self) -> bool {
        self.inner.is_ok()
    }

    /// Whether or not the execution produced errors.
    ///
    /// ### Returns:
    ///
    /// - `bool`: `True` if the execution returned a non zero exit code.
    #[pyo3(text_signature = "(self) -> bool")]
    fn is_err(&self) -> bool {
        self.inner.is_err()
    }
}

/// A response from the Piston api when sending a request to execute code.
///
/// **NOTE**:
///
/// - This object cannot be instantiated, and is immutable.
/// - It can only be created with a call to `Client.execute`.
#[pyclass]
#[derive(Clone)]
pub struct ExecResponse {
    inner: ExecResponse_,
}

impl ExecResponse {
    /// Creates a new ExecResponse from a `piston_rs.ExecResponse`.
    pub fn from_response(response: ExecResponse_) -> Self {
        Self { inner: response }
    }
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
    /// Raises a TypeError because this class cannot be instantiated.
    fn new_() -> PyResult<Self> {
        Err(PyTypeError::new_err("ExecResponse can not be instantiated"))
    }

    /// `str`: The language that was used.
    #[getter]
    fn language(&self) -> String {
        self.inner.language.clone()
    }

    /// `str`: The version of the language that was used.
    #[getter]
    fn version(&self) -> String {
        self.inner.version.clone()
    }

    /// `ExecResult`: The result Piston sends detailing execution.
    #[getter]
    fn run(&self) -> ExecResult {
        ExecResult::from_result(&self.inner.run)
    }

    /// `ExecResult` | `None`: The optional result Piston sends detailing compilation.
    /// This will be `None` for non-compiled languages.
    #[getter]
    fn compile(&self) -> Option<ExecResult> {
        self.inner.compile.as_ref().map(ExecResult::from_result)
    }

    /// `int`: The response status returned by Piston.
    #[getter]
    fn status(&self) -> u16 {
        self.inner.status
    }

    /// Whether or not the request to Piston succeeded.
    ///
    /// ### Returns:
    ///
    /// - `bool`: `True` if a 200 status code was received.
    #[pyo3(text_signature = "(self) -> bool")]
    fn is_ok(&self) -> bool {
        self.inner.is_ok()
    }

    /// Whether or not the request to Piston failed.
    ///
    /// ### Returns:
    ///
    /// - `bool`: `True` if a non 200 status code was received.
    #[pyo3(text_signature = "(self) -> bool")]
    fn is_err(&self) -> bool {
        self.inner.is_err()
    }
}

/// An object containing information about the code being executed.
///
/// A convenient builder flow is provided by the methods associated with
/// the `Executor`. These consume self and return self for chained calls.
///
/// - For `compile_memory_limit` and `run_memory_limit` -1 can be used
/// to signify no limit.
#[pyclass]
#[derive(Clone)]
#[pyo3(
    text_signature = "(language: str = \"\", version: str = \"*\", files: list[File] = [], stdin: str = \"\", args: list[str] = [], compile_timeout: int = 10000, run_timeout: int = 3000, compile_memory_limit: int = -1, run_memory_limit: int = -1, /) -> Executor"
)]
pub struct Executor {
    inner: Executor_,
}

impl Executor {
    /// Converts the Executor into a piston_rs.Executor.
    pub fn convert(&self) -> Executor_ {
        self.inner.clone()
    }
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
        version = "\"*\".to_string()",
        files = "vec![]",
        stdin = "\"\".to_string()",
        args = "vec![]",
        compile_timeout = "10000",
        run_timeout = "3000",
        compile_memory_limit = "-1",
        run_memory_limit = "-1"
    )]
    #[allow(clippy::too_many_arguments)]
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

    /// `str`: The language to use for execution.
    #[getter]
    fn language(&self) -> String {
        self.inner.language.clone()
    }

    #[setter(language)]
    fn language_setter(&mut self, language: String) {
        self.inner.language = language.to_lowercase();
    }

    /// `str`: The version of the language to use for execution.
    #[getter]
    fn version(&self) -> String {
        self.inner.version.clone()
    }

    #[setter(version)]
    fn version_setter(&mut self, version: String) {
        self.inner.version = version;
    }

    /// `list[File]`: A list of files to send to Piston. The first file
    /// in the list is considered the main file.
    #[getter]
    fn files(&self) -> Vec<File> {
        self.inner
            .files
            .clone()
            .iter()
            .map(File::from_inner)
            .collect()
    }

    #[setter(files)]
    fn files_setter(&mut self, files: Vec<File>) {
        self.inner
            .set_files(files.iter().map(|f| f.convert()).collect());
    }

    /// `str`: The text to pass as stdin to the program.
    #[getter]
    fn stdin(&self) -> String {
        self.inner.stdin.clone()
    }

    #[setter(stdin)]
    fn stdin_setter(&mut self, stdin: String) {
        self.inner.stdin = stdin;
    }

    /// `list[str]`: The command line arguments to pass to the program.
    #[getter]
    fn args(&self) -> Vec<String> {
        self.inner.args.clone()
    }

    #[setter(args)]
    fn args_setter(&mut self, args: Vec<String>) {
        self.inner.args = args;
    }

    /// `int`: The maximum allowed time for compilation in milliseconds.
    #[getter]
    fn compile_timeout(&self) -> isize {
        self.inner.compile_timeout
    }

    #[setter(compile_timeout)]
    fn compile_timeout_setter(&mut self, timeout: isize) {
        self.inner.compile_timeout = timeout;
    }

    /// `int`: The maximum allowed time for execution in milliseconds.
    #[getter]
    fn run_timeout(&self) -> isize {
        self.inner.compile_timeout
    }

    #[setter(run_timeout)]
    fn run_timeout_setter(&mut self, timeout: isize) {
        self.inner.run_timeout = timeout;
    }

    /// `int`: The maximum allowed memory usage for compilation in bytes.
    #[getter]
    fn compile_memory_limit(&self) -> isize {
        self.inner.compile_memory_limit
    }

    #[setter(compile_memory_limit)]
    fn compile_memory_limit_setter(&mut self, limit: isize) {
        self.inner.compile_memory_limit = limit;
    }

    /// `int`: The maximum allowed memory usage for execution in bytes.
    #[getter]
    fn run_memory_limit(&self) -> isize {
        self.inner.run_memory_limit
    }

    #[setter(run_memory_limit)]
    fn run_memory_limit_setter(&mut self, limit: isize) {
        self.inner.run_memory_limit = limit;
    }

    /// Copies the executor, leaving the existing one unchanged.
    ///
    /// ### Returns:
    ///
    /// - `Executor`: A copy of the executor.
    #[pyo3(text_signature = "(self) -> Executor")]
    fn copy(&self) -> Self {
        self.clone()
    }

    /// Resets the executor back to a `new` state, ready to be
    /// configured again and sent to Piston after metadata is added.
    ///
    /// This method mutates the executor in place.
    #[pyo3(text_signature = "(self) -> None")]
    fn reset(&mut self) {
        self.inner.reset();
    }

    /// Sets the language to use for execution.
    ///
    /// ### Args:
    ///
    /// - language `str`:
    /// The language to use.
    ///
    /// ### Returns:
    ///
    /// - `Executor`: The executor, for chained method calls.
    #[pyo3(text_signature = "(self, language: str, /) -> Executor")]
    fn set_language(mut slf: PyRefMut<Self>, language: String) -> PyRefMut<Self> {
        slf.inner.language = language.to_lowercase();
        slf
    }

    /// Sets the version of the language to use for execution.
    ///
    /// ### Args:
    ///
    /// - version `str`:
    /// The version to use.
    ///
    /// ### Returns:
    ///
    /// - `Executor`: The executor, for chained method calls.
    #[pyo3(text_signature = "(self, version: str, /) -> Executor")]
    fn set_version(mut slf: PyRefMut<Self>, version: String) -> PyRefMut<Self> {
        slf.inner.version = version;
        slf
    }

    /// Adds a `File` containing the code to be executed.
    ///
    /// Does not overwrite any existing files.
    ///
    /// ### Args:
    ///
    /// - file `File`:
    /// The file to add.
    ///
    /// ### Returns:
    ///
    /// - `Executor`: The executor, for chained method calls.
    #[pyo3(text_signature = "(self, file: File, /) -> Executor")]
    fn add_file(mut slf: PyRefMut<Self>, file: File) -> PyRefMut<Self> {
        slf.inner.files.push(file.convert());
        slf
    }

    /// Adds multiple `File`'s containing code to be executed.
    ///
    /// Does not overwrite any existing files.
    ///
    /// ### Args:
    ///
    /// - files `list[File]`:
    /// The file to add.
    ///
    /// ### Returns:
    ///
    /// - `Executor`: The executor, for chained method calls.
    #[pyo3(text_signature = "(self, files: list[File], /) -> Executor")]
    fn add_files(mut slf: PyRefMut<Self>, files: Vec<File>) -> PyRefMut<Self> {
        slf.inner.files.extend(files.iter().map(|f| f.convert()));
        slf
    }

    /// Adds multiple `File`'s containing the code to be executed.
    ///
    /// This method mutates the executor in place.
    /// **Overwrites any existing files.**
    ///
    /// ### Args:
    ///
    /// - files `list[File]`:
    /// The files to replace existing files with.
    #[pyo3(text_signature = "(self, files: list[File], /) -> None")]
    fn set_files(&mut self, files: Vec<File>) {
        self.inner.files = files.iter().map(|f| f.convert()).collect();
    }

    /// Sets the text to pass as `stdin` to the program.
    ///
    /// ### Args:
    ///
    /// - stdin `str`:
    /// The text to set.
    ///
    /// ### Returns:
    ///
    /// - `Executor`: The executor, for chained method calls.
    #[pyo3(text_signature = "(self, stdin: str, /) -> Executor")]
    fn set_stdin(mut slf: PyRefMut<Self>, stdin: String) -> PyRefMut<Self> {
        slf.inner.stdin = stdin;
        slf
    }

    /// Adds an arg to be passed as a command line argument.
    ///
    /// Does not overwrite any existing args.
    ///
    /// ### Args:
    ///
    /// - arg `str`:
    /// The arg to add.
    ///
    /// ### Returns:
    ///
    /// - `Executor`: The executor, for chained method calls.
    #[pyo3(text_signature = "(self, arg: str, /) -> Executor")]
    fn add_arg(mut slf: PyRefMut<Self>, arg: String) -> PyRefMut<Self> {
        slf.inner.args.push(arg);
        slf
    }

    /// Adds multiple args to be passed as a command line argument.
    ///
    /// Does not overwrite any existing args.
    ///
    /// ### Args:
    ///
    /// - args `list[str]`:
    /// The args to add.
    ///
    /// ### Returns:
    ///
    /// - `Executor`: The executor, for chained method calls.
    #[pyo3(text_signature = "(self, args: list[str], /) -> Executor")]
    fn add_args(mut slf: PyRefMut<Self>, args: Vec<String>) -> PyRefMut<Self> {
        slf.inner.args.extend(args);
        slf
    }

    /// Adds multiple args to be passed as command line arguments.
    ///
    /// This method mutates the executor in place.
    /// **Overwrites any existing args.**
    ///
    /// ### Args:
    ///
    /// - args `list[str]`:
    /// The args to replace existing args with.
    #[pyo3(text_signature = "(self, args: list[str], /) -> None")]
    fn set_args(&mut self, args: Vec<String>) {
        self.inner.args = args;
    }

    /// Sets the maximum allowed time for compilation in milliseconds.
    ///
    /// ### Args:
    ///
    /// - timeout `int`:
    /// The timeout to set.
    ///
    /// ### Returns:
    ///
    /// - `Executor`: The executor, for chained method calls.
    #[pyo3(text_signature = "(self, timeout: int, /) -> Executor")]
    fn set_compile_timeout(mut slf: PyRefMut<Self>, timeout: isize) -> PyRefMut<Self> {
        slf.inner.compile_timeout = timeout;
        slf
    }

    /// Sets the maximum allowed time for execution in milliseconds.
    ///
    /// ### Args:
    ///
    /// - timeout `int`:
    /// The timeout to set.
    ///
    /// ### Returns:
    ///
    /// - `Executor`: The executor, for chained method calls.
    #[pyo3(text_signature = "(self, timeout: int, /) -> Executor")]
    fn set_run_timeout(mut slf: PyRefMut<Self>, timeout: isize) -> PyRefMut<Self> {
        slf.inner.run_timeout = timeout;
        slf
    }

    /// Sets the maximum allowed memory usage for compilation in bytes.
    ///
    /// ### Args:
    ///
    /// - limit `int`:
    /// The limit to set.
    ///
    /// ### Returns:
    ///
    /// - `Executor`: The executor, for chained method calls.
    #[pyo3(text_signature = "(self, limit: int, /) -> Executor")]
    fn set_compile_memory_limit(mut slf: PyRefMut<Self>, limit: isize) -> PyRefMut<Self> {
        slf.inner.compile_memory_limit = limit;
        slf
    }

    /// Sets the maximum allowed memory usage for execution in bytes.
    ///
    /// ### Args:
    ///
    /// - limit `int`:
    /// The limit to set.
    ///
    /// ### Returns:
    ///
    /// - `Executor`: The executor, for chained method calls.
    #[pyo3(text_signature = "(self, limit: int, /) -> Executor")]
    fn set_run_memory_limit(mut slf: PyRefMut<Self>, limit: isize) -> PyRefMut<Self> {
        slf.inner.run_memory_limit = limit;
        slf
    }
}
