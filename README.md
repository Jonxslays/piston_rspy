# piston_rspy

Python bindings for [`piston_rs`](https://github.com/Jonxslays/piston_rs).

# What is piston_rspy

piston_rspy provides Python users the ability to interact with the
[Piston](https://github.com/engineer-man/piston) code execution engine,
but behind the scenes it is powered by [piston_rs](https://github.com/Jonxslays/piston_rs),
a Rust library designed for the same purpose.

# Getting started

piston_rspy officially supports Python versions 3.7, 3.8, 3.9, and 3.10.
For an in depth look at the API, check out the [documentation](https://jonxslays.github.io/piston_rspy/piston_rspy/)!

#### Installation

```bash
pip install piston_rspy
```

#### Usage

Fetching the available runtimes from Piston.
```py
import asyncio

import piston_rspy


async def main() -> None:
    client = piston_rspy.Client()
    runtimes = await client.fetch_runtimes()

    print(runtimes)


if __name__ == "__main__":
    asyncio.run(main())
```

---

Executing python code via Piston.
```py
import asyncio

import piston_rspy


async def main() -> None:
    file = piston_rspy.File(
        name="main.py",
        content="for i in range(10): print(i)",
    )

    executor = piston_rspy.Executor(
        language="python",
        version="3.10",
        files=[file],
    )

    client = piston_rspy.Client()
    response = await client.execute(executor)

    print(f"Language: {response.language} v{response.version}")

    if response.compile:
        print(f"Compilation:\n{response.compile.output}")

    print(f"Output:\n{response.run.output}")


if __name__ == "__main__":
    asyncio.run(main())
```

---

The builder flow that is used in `piston_rs` is available in
`piston_rspy` as well.
```py
import asyncio

import piston_rspy


async def main() -> None:
    client = piston_rspy.Client()

    response = await client.execute(
        piston_rspy.Executor()
        .set_language("python")
        .set_version("3.10")
        .add_file(
            piston_rspy.File(
                name="main.py",
                content="for i in range(10): print(i)",
            )
        )
    )

    print(f"Language: {response.language} v{response.version}")

    if response.compile:
        print(f"Compilation:\n{response.compile.output}")

    print(f"Output:\n{response.run.output}")


if __name__ == "__main__":
    asyncio.run(main())
```

## License

piston_rspy is licensed under the [MIT License](https://github.com/Jonxslays/piston_rspy/blob/master/LICENSE).
