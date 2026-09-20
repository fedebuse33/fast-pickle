# fast-pickle

A pickle-like serializer written in Rust, with a name that is best described as _aspirational_.

> **Is it faster than `pickle`?**
> Only if you serialize `None`.

`fast-pickle` was built to learn how to ship a Rust extension to PyPI with [maturin](https://www.maturin.rs/). It works, it is tested, and it is slower than the standard library on anything that contains more than one element. You have been warned, in the README, before installing.

## Installation

```bash
pip install fast-pickle
```

Requires Python 3.13+. Wheels are built with `abi3`, so one wheel covers every Python version from 3.13 onwards.

## Usage

```python
import fastpickle

data = {"name": "Federico", "scores": [10.5, 20.75, -3], "tags": {"a", "b"}}

blob = fastpickle.dumps(data)
assert fastpickle.loads(blob) == data
```

### Custom classes

Subclass `Serializable` to get `dumps()` / `loads()` on your own objects:

```python
class User(fastpickle.Serializable):
    name: str
    age: int

user = User()
user.name = "Fede"
user.age = 13

restored = User.loads(user.dumps())
print(restored.__dict__)  # {'name': 'Fede', 'age': 13}
```

Note: `__init__` is **not** called when loading. The instance is created empty and its `__dict__` is filled in.

## Supported types

`None`, `bool`, `int` (signed 64-bit), `float`, `str`, `bytes`, `list`, `tuple`, `set`, `dict`, and objects that have a `__dict__`.

Things that will not work:

- **Integers outside the signed 64-bit range** raise `OverflowError`.
- **`frozenset`** and any other type without a `__dict__` raise `TypeError`.
- **Circular references** raise `RecursionError`. The maximum nesting depth is 256.
- **Module-level `fastpickle.loads()`** returns objects as a plain `(class_name, state_dict)` tuple. Use `Serializable.loads()` to get real instances back.
- Class names are stored by `__qualname__`, so two `Serializable` subclasses with the same name in different modules will collide.

## Benchmarks

Measured with `pytest-benchmark` against the standard library `pickle`. Your numbers will differ, but the shape should not.

| Operation                   | fast-pickle | pickle      | Result          |
| --------------------------- | ----------- | ----------- | --------------- |
| `dumps(None)`               | 113 ns      | 618 ns      | 5x faster       |
| `dumps(True)`               | 143 ns      | 610 ns      | 4x faster       |
| `dumps(int)`                | 235 ns      | 627 ns      | 2.7x faster     |
| `dumps(float)`              | 570 ns      | 629 ns      | about the same  |
| `dumps(str)`                | 934 ns      | 648 ns      | 1.4x slower     |
| `dumps(bytes)`              | 946 ns      | 638 ns      | 1.5x slower     |
| `dumps(list of 1000)`       | 106 µs      | 12.6 µs     | 8x slower       |
| `dumps(set of 1000)`        | 108 µs      | 14.5 µs     | 7.5x slower     |
| `dumps(dict of 1000)`       | 815 µs      | 47 µs       | 17x slower      |
| `loads(scalar)`             | ~105-155 ns | ~250-310 ns | about 2x faster |
| `loads(collection of 1000)` | 52-184 µs   | 20-91 µs    | 2-2.6x slower   |

The scalar wins are not a performance achievement: `pickle` has a fixed setup cost per call, and `fast-pickle` is a single function call. As soon as there is real work to do, the standard library's C implementation, which builds objects directly, wins.

You can run the benchmarks yourself:

```bash
uv run pytest benchmarks --benchmark-warmup=on --benchmark-warmup-iterations=1000 --benchmark-disable-gc
```

## Security

Unlike `pickle`, decoding never executes arbitrary code: the format only describes plain data. The decoder validates lengths and nesting depth, so malformed input produces an error instead of a crash.

That said, `Serializable.loads()` will instantiate any registered `Serializable` subclass with whatever state is in the payload, without calling `__init__`. Do not feed it data you do not trust.

## Wire format

Every value is a one-byte tag followed by its payload. Integers and lengths are little-endian 64-bit.

| Tag    | Type                        |
| ------ | --------------------------- |
| `0x00` | `None`                      |
| `0x01` | `bool`                      |
| `0x02` | `int`                       |
| `0x03` | `float`                     |
| `0x04` | `str` (length + UTF-8)      |
| `0x05` | `bytes` (length + data)     |
| `0x06` | `list`                      |
| `0x07` | `tuple`                     |
| `0x08` | `dict`                      |
| `0x09` | `set`                       |
| `0x0A` | object (class name + state) |

The format is not versioned and not guaranteed stable before 1.0.

## Development

```bash
uv sync --group tests
uv run maturin develop --release
uv run pytest            # correctness tests
cargo test               # Rust unit tests
```

## License

MIT
