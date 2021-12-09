from setuptools import setup
from setuptools_rust import Binding, RustExtension # type: ignore


setup_requires = ["setuptools-rust>=0.9.2"]

with open("README.md") as r:
    long_description = r.read()

setup(
    name="piston_rspy",
    version="0.1.2",
    description="Python bindings for piston_rs.",
    author="Jonxslays",
    long_description=long_description,
    long_description_content_type="text/markdown",
    license="MIT",
    url="https://github.com/Jonxslays/piston_rspy",
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
        "Programming Language :: Python :: Implementation :: CPython",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Typing :: Typed",
    ],
    rust_extensions=[
        RustExtension(
            "piston_rspy.piston_rspy", "Cargo.toml", binding=Binding.PyO3, # type: ignore
        )
    ],
    setup_requires=setup_requires,
    include_package_data=True,
    packages=["piston_rspy"],
    zip_safe=False,
    python_requires=">=3.7,<3.11",
)
