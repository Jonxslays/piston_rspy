from __future__ import annotations

import typing

from setuptools import setup
from setuptools_rust import Binding, RustExtension, Strip  # type: ignore


setup_requires = ["setuptools-rust>=0.9.2"]

with open("README.md") as r:
    long_description = r.read()

with open("pyproject.toml") as f:
    meta: dict[str, typing.Any] = {}

    for line in f.readlines():
        if line.startswith("\n"):
            break

        if line.startswith("[project]"):
            continue

        k, v = line.split(" = ")
        meta[k] = v.strip().replace('"', "")

setup(
    name=meta["name"],
    version=meta["version"],
    description=meta["description"],
    author="Jonxslays",
    long_description=long_description,
    long_description_content_type="text/markdown",
    license="MIT",
    url="https://github.com/Jonxslays/piston_rspy",
    project_urls={
        "Documentation": "https://jonxslays.github.io/piston_rspy/piston_rspy",
        "Source": "https://github.com/Jonxslays/piston_rspy",
        "Bug Tracker": "https://github.com/Jonxslays/piston_rspy/issues",
    },
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Framework :: AsyncIO",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Rust",
        "Programming Language :: Python :: 3 :: Only",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: Implementation :: CPython",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Typing :: Typed",
    ],
    rust_extensions=[
        RustExtension(
            "piston_rspy.piston_rspy",
            "Cargo.toml",
            binding=Binding.PyO3,  # type: ignore
            strip=Strip.Debug,  # type: ignore
        )
    ],
    setup_requires=setup_requires,
    include_package_data=True,
    packages=["piston_rspy"],
    zip_safe=False,
    python_requires=">=3.7,<3.12",
)
