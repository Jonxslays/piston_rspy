from __future__ import annotations

__all__ = ["Runtime", "File", "ExecResult", "ExecResponse", "Executor", "Client"]

import typing as t
from dataclasses import dataclass, field

@dataclass(frozen=True)
class Runtime:
    language: str
    version: str
    aliases: list[str]
    def copy(self) -> Runtime: ...

@dataclass
class File:
    name: str = ""
    content: str = ""
    encoding: str = "utf8"
    def set_name(self, name: str) -> File: ...
    def set_content(self, content: str) -> File: ...
    def set_encoding(self, encoding: str) -> File: ...
    def copy(self) -> File: ...

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
    url: str = "https://emkc.org/api/v2/piston"
    headers: dict[str, str] = {
        "Accept": "application/json",
        "User-Agent": "piston-rs",
    }
    @staticmethod
    def with_key(key: str) -> Client: ...
    def get_headers(self) -> dict[str, str]: ...
    def get_url(self) -> str: ...
    async def fetch_runtimes(self) -> list[Runtime]: ...
    async def execute(self, executor: Executor) -> ExecResponse: ...

class FailedRequest(Exception): ...
