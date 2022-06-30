from __future__ import annotations

import typing as t
from dataclasses import dataclass, field

__all__: list[str] = [
    "Runtime",
    "File",
    "ExecResult",
    "ExecResponse",
    "Executor",
    "Client",
]

@dataclass(frozen=True)
class Runtime:
    """A runtime available to be used by Piston.

    All properties of the runtime are immutable.

    Args:
        language: `str`
            The language.
        version: `str`
            The version of the language.
        aliases: `list[str]`
            The aliases of the language.

    NOTE:
    Runtimes are not meant to be created manually. Instead, they
    should be fetched from Piston using `Client.fetch_runtimes()`
    and stored. The Python bindings for `piston_rs` do allow you to
    instantiate the class, however.
    """

    language: str
    version: str
    aliases: list[str]
    def copy(self) -> Runtime:
        """Copies the runtime, leaving the existing one unchanged.

        Returns:
            `Runtime`: A copy of the runtime.
        """
        ...

@dataclass
class File:
    """A file that contains the source code to be executed.

    Args:
        name: str = ""
            The name of the file.
        content: str = ""
            **Required by Piston** The content of the file.
        encoding: str = "utf8"
            The encoding of the file.
    """

    name: str = ""
    content: str = ""
    encoding: str = "utf8"
    @classmethod
    def load_from(cls, path: str) -> File:
        """Creates a new `File` from an existing file on disk.

        Args:
            path: `str`
                The path to the file.

        Returns:
            `File`: The new file.
        """
        ...
    def set_content(self, content: str) -> File:
        """Sets the content of the file.

        Args:
            content: `str`
                The content to use.

        Returns:
            `File`: The file, for chained method calls.
        """
        ...
    def load_content_from(self, path: str) -> File:
        """Sets the content of the file to the contents of an existing
        file on disk.

        Args:
            path: `str`
                The path to the file.

        Returns:
            `File`: The file, for chained method calls.
        """
        ...
    def set_name(self, name: str) -> File:
        """Sets the name of the file.

        Args:
            name: `str`
                The name to use.

        Returns:
            `File`: The file, for chained method calls.
        """
        ...
    def set_encoding(self, encoding: str) -> File:
        """Sets the encoding of the file.

        Args:
            encoding: `str`
                The encoding to use.

        Returns:
            `File`: The file, for chained method calls.
        """
        ...
    def copy(self) -> File:
        """Copies the file, leaving the existing one unchanged.

        Returns:
            `File`: A copy of the file.
        """
        ...

@dataclass(init=False, frozen=True)
class ExecResult:
    """The result of code execution returned by Piston.

    NOTE:
        - This object cannot be instantiated, and is immutable.
        - It can only be created with a call to `Client.execute()`.

    Properties:
        stdout: `str`
            The text sent to `stdout` during execution.
        stderr: `str`
            The text sent to `stderr` during execution.
        output: `str`
            The text sent to both `stdout`, and `stderr` during
            execution.
        code: `int | None`
            The optional exit code returned by the process.
        signal: `str | None`
            The optional signal sent to the process. (`SIGKILL` etc)

    Raises:
        `TypeError`: If the class is instantiated manually.
    """

    stdout: str
    stderr: str
    output: str
    code: int | None
    signal: str | None
    def is_ok(self) -> bool:
        """Whether or not the execution was ok.

        Returns:
            `bool`: `True` if the execution returned a zero exit code.
        """
        ...
    def is_err(self) -> bool:
        """Whether or not the execution produced errors.

        Returns:
            `bool`: `True` if the execution returned a non zero exit
            code.
        """
        ...

@dataclass(init=False, frozen=True)
class ExecResponse:
    """A response from the Piston api when sending a request to execute
    code.

    NOTE:
        - This object cannot be instantiated, and is immutable.
        - It can only be created with a call to `Client.execute()`.

    Properties:
        language: `str`
            The language that was used.
        version: `str`
            The version of the language that was used.
        run: `ExecResult`
            The result Piston sends detailing execution.
        compile: `ExecResult | None`
            The optional result Piston sends detailing compilation.
            This will be `None` for non-compiled languages.
        status: `int`
            The response status returned by Piston.

    Raises:
        `TypeError`: If the class is instantiated manually.
    """

    language: str
    version: str
    run: ExecResult
    compile: ExecResult | None
    status: int
    def is_ok(self) -> bool:
        """Whether or not the request to Piston succeeded.

        Returns:
            `bool`: `True` if a 200 status code was received.
        """
        ...
    def is_err(self) -> bool:
        """Whether or not the request to Piston failed.

        Returns:
            `bool`: `True` if a non 200 status code was received.
        """
        ...

@dataclass
class Executor:
    """An object containing information about the code being executed.

    A convenient builder flow is provided by the methods associated with
    the `Executor`. These consume self and return self for chained method calls.

    - For `compile_memory_limit` and `run_memory_limit` -1 can be used
    to signify no limit.

    Args:
        language: `str` = ""
            The language to use for execution.
        version: `str` = "*"
            The version of the language to use for execution.
        files: `list[File]` = []
            A list of files to send to Piston. The first file in the
            list is considered the main file.
        stdin: `str` = ""
            The text to pass as stdin to the program.
        args: `list[str]` = []
            The command line arguments to pass to the program.
        compile_timeout: `int` = 30000
            The maximum allowed time for compilation in milliseconds.
        run_timeout: `int` = 3000
            The maximum allowed time for execution in milliseconds.
        compile_memory_limit: `int` = -1
            The maximum allowed memory usage for compilation in bytes.
        run_memory_limit: `int` = -1
            The maximum allowed memory usage for execution in bytes.

    """

    language: str = ""
    version: str = "*"
    files: list[File] = field(default_factory=list)
    stdin: str = ""
    args: list[str] = field(default_factory=list)
    compile_timeout: int = 10000
    run_timeout: int = 3000
    compile_memory_limit: int = -1
    run_memory_limit: int = -1
    def copy(self) -> Executor:
        """Copies the executor, leaving the existing one unchanged.

        Returns:
            `Executor`: A copy of the executor.
        """
        ...
    def reset(self) -> None:
        """Resets the executor back to a `new` state, ready to be
        configured again and sent to Piston after metadata is added.

        - This method mutates the executor in place.
        """
        ...
    def set_language(self, language: str) -> Executor:
        """Sets the language to use for execution.

        Args:
            language: `str`
                The language to use.

        Returns:
            `Executor`: The executor, for chained method calls.
        """
        ...
    def set_version(self, version: str) -> Executor:
        """Sets the version of the language to use for execution.

        Args:
            version: `str`
                The version to use.

        Returns:
            `Executor`: The executor, for chained method calls.
        """
        ...
    def add_file(self, file: File) -> Executor:
        """Adds a `File` containing the code to be executed.

        - Does not overwrite any existing files.

        Args:
            file: `File`
                The file to add.

        Returns:
            `Executor`: The executor, for chained method calls.
        """
        ...
    def add_files(self, files: list[File]) -> Executor:
        """Adds multiple `File`'s containing the code to be executed.

        - Does not overwrite any existing files.

        Args:
            files: `list[File]`
                The files to add.

        Returns:
            `Executor`: The executor, for chained method calls.
        """
        ...
    def set_files(self, files: list[File]) -> None:
        """Adds multiple `File`'s containing the code to be executed.

        - This method mutates the executor in place.
        - **Overwrites any existing files.**

        Args:
            files: `list[File]`
                The files to replace existing files with.
        """
        ...
    def set_stdin(self, stdin: str) -> Executor:
        """Sets the text to pass as `stdin` to the program.

        Args:
            stdin: `str`
                The text to set.

        Returns:
            `Executor`: The executor, for chained method calls.
        """
        ...
    def add_arg(self, arg: str) -> Executor:
        """Adds an arg to be passed as a command line argument.

        - Does not overwrite any existing args.

        Args:
            arg: `str`
                The arg to add.

        Returns:
            `Executor`: The executor, for chained method calls.
        """
        ...
    def add_args(self, args: list[str]) -> Executor:
        """Adds multiple args to be passed as a command line argument.

        - Does not overwrite any existing args.

        Args:
            args: `list[str]`
                The args to add.

        Returns:
            `Executor`: The executor, for chained method calls.
        """
        ...
    def set_args(self, args: list[str]) -> None:
        """Adds multiple args to be passed as command line arguments.

        - This method mutates the executor in place.
        - **Overwrites any existing args.**

        Args:
            args: `list[str]`
                The args to replace existing args with.
        """
        ...
    def set_compile_timeout(self, timeout: int) -> Executor:
        """Sets the maximum allowed time for compilation in
        milliseconds.

        Args:
            timeout: `int`
                The timeout to set.

        Returns:
            `Executor`: The executor, for chained method calls.
        """
        ...
    def set_run_timeout(self, timeout: int) -> Executor:
        """Sets the maximum allowed time for execution in milliseconds.

        Args:
            timeout: `int`
                The timeout to set.

        Returns:
            `Executor`: The executor, for chained method calls.
        """
        ...
    def set_compile_memory_limit(self, limit: int) -> Executor:
        """Sets the maximum allowed memory usage for compilation in
        bytes.

        Args:
            limit: `int`
                The limit to set.

        Returns:
            `Executor`: The executor, for chained method calls.
        """
        ...
    def set_run_memory_limit(self, limit: int) -> Executor:
        """Sets the maximum allowed memory usage for execution in bytes.

        Args:
            limit: `int`
                The limit to set.

        Returns:
            `Executor`: The executor, for chained method calls.
        """
        ...

@dataclass(frozen=True)
class Client:
    """A client used to send requests to Piston.

    NOTE:
        - The client is immutable, and by extension its properties are
        also immutable.

    Properties:
        url: `str`
            The base url for the Piston v2 api.
        headers: `dict[str, str]`
            The headers being sent with requests.
    """

    url: str = field(init=False, default="https://emkc.org/api/v2/piston")
    headers: dict[str, str] = field(
        init=False,
        default_factory=lambda: {
            "Accept": "application/json",
            "User-Agent": "piston-rs",
        },
    )
    @staticmethod
    def with_key(key: str) -> Client:
        """Creates a new client, with an api key.

        Args:
            key: `str`
                The api key to use.

        Returns:
            `Client`: The new client.
        """
        ...
    @staticmethod
    def with_url(url: str) -> Client:
        """Creates a new client with a custom url.

        Args:
            url: `str`
                The url to use as the underlying Piston backend.

        Returns:
            `Client`: The new client.
        """
        ...
    @staticmethod
    def with_url_and_key(url: str, key: str) -> Client:
        """Creates a new client with a custom url, and an api key.

        Args:
            url: `str`
                The url to use as the underlying Piston backend.
            key: `str`
                The api key to use.

        Returns:
            `Client`: The new client.
        """
        ...
    def get_headers(self) -> dict[str, str]:
        """The headers being sent with requests.

        Returns:
            `dict[str, str]`: The headers.
        """
        ...
    def get_url(self) -> str:
        """The base url for the Piston v2 api.

        Returns:
            `str`: The url.
        """
        ...
    async def fetch_runtimes(self) -> list[Runtime]:
        """`async` Fetches the runtimes from Piston. This is an http
        request.

        Returns:
            `asyncio.Future[list[Runtime]]`: The available Piston
            runtimes.

        Raises:
            `RuntimeError`: If the request to Piston failed.
        """
        ...
    async def execute(self, executor: Executor) -> ExecResponse:
        """`async` Executes code using a given executor. This is an http
        request.

        Args:
            executor: `Executor`
                The executor to use for the request.

        Returns:
            `asyncio.Future[ExecResponse]`: The response from Piston.

        Raises:
            `RuntimeError`: If the request to Piston failed.
        """
        ...
