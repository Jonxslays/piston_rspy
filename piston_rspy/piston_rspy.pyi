from __future__ import annotations

from dataclasses import dataclass, field

__all__: list[str] = [
    "Runtime",
    "File",
    "ExecResult",
    "ExecResponse",
    "Executor",
    "Client",
    "FailedRequest",
]


@dataclass(frozen=True)
class Runtime:
    """A runtime available to be used by Piston.

    Args:
        language: `str`
            The language.
        version: `str`
            The version of the language.
        aliases: `list[str]`
            The aliases of the language.

    NOTE:
        Runtimes are not meant to be created manually. Instead, they
        should be fetched from Piston using `Client.fetch_runtimes` and
        stored. The Python bindings for `piston_rs` do allow you to
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
        name: str
            The name of the file. Defaults to "".
        content: str
            **Required** The content of the file.
        encoding: str
            The encoding of the file. Defaults to "utf8".
    """

    name: str = ""
    content: str = ""
    encoding: str = "utf8"
    def set_name(self, name: str) -> File:
        """Sets the name of the file.

        Args:
            name: `str`
                The name to use.

        Returns:
            `File`: The file, for chained calls.
        """
        ...
    def set_content(self, content: str) -> File:
        """Sets the content of the file.

        Args:
            content: `str`
                The content to use.

        Returns:
            `File`: The file, for chained calls.
        """
        ...
    def set_encoding(self, encoding: str) -> File:
        """Sets the encoding of the file.

        Args:
            encoding: `str`
                The encoding to use.

        Returns:
            `File`: The file, for chained calls.
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
    stdout: str
    stderr: str
    output: str
    code: int
    signal: str | None
    def is_ok(self) -> bool: ...
    def is_err(self) -> bool: ...

@dataclass(init=False, frozen=True)
class ExecResponse:
    language: str
    version: str
    run: ExecResult
    compile: ExecResult | None
    def is_ok(self) -> bool: ...
    def is_err(self) -> bool: ...

@dataclass
class Executor:
    language: str = ""
    version: str = ""
    files: list[File] = field(default_factory=list)
    stdin: str = ""
    args: list[str] = field(default_factory=list)
    compile_timeout: int = 10000
    run_timeout: int = 3000
    compile_memory_limit: int = -1
    run_memory_limit: int = -1
    def reset(self) -> None: ...
    def copy(self) -> Executor: ...
    def set_language(self, language: str) -> Executor: ...
    def set_version(self, version: str) -> Executor: ...
    def add_file(self, file: File) -> Executor: ...
    def add_files(self, files: list[File]) -> Executor: ...
    def set_files(self, files: list[File]) -> Executor: ...
    def set_stdin(self, stdin: str) -> Executor: ...

@dataclass(frozen=True)
class Client:
    url: str = field(init=False, default="https://emkc.org/api/v2/piston")
    headers: dict[str, str] = field(
        init=False,
        default_factory=lambda: {
            "Accept": "application/json",
            "User-Agent": "piston-rs",
        },
    )
    @staticmethod
    def with_key(key: str) -> Client: ...
    def get_headers(self) -> dict[str, str]: ...
    def get_url(self) -> str: ...
    async def fetch_runtimes(self) -> list[Runtime]: ...
    async def execute(self, executor: Executor) -> ExecResponse: ...

class FailedRequest(Exception):
    """Raised when a request to Piston fails."""

    ...
