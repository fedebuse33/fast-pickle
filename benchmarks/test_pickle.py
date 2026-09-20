import pickle

import pytest
import fastpickle


CASES = {
    "none": None,
    "bool": True,
    "int": 123456789,
    "float": 3.1415926535,
    "string": "hello world",
    "bytes": b"hello world",
    "list_1k": list(range(1000)),
    "tuple_1k": tuple(range(1000)),
    "set_1k": set(range(1000)),
    "dict_1k": {str(i): i for i in range(1000)},
}


@pytest.mark.parametrize("name,value", CASES.items(), ids=CASES.keys())
class TestDumps:
    def test_pickle(self, benchmark, name, value):
        benchmark.group = f"dumps[{name}]"
        benchmark.extra_info["library"] = "pickle"
        benchmark(pickle.dumps, value)

    def test_fastpickle(self, benchmark, name, value):
        benchmark.group = f"dumps[{name}]"
        benchmark.extra_info["library"] = "fastpickle"
        benchmark(fastpickle.dumps, value)


@pytest.mark.parametrize("name,value", CASES.items(), ids=CASES.keys())
class TestLoads:
    def test_pickle(self, benchmark, name, value):
        data = pickle.dumps(value)
        benchmark.group = f"loads[{name}]"
        benchmark(pickle.loads, data)

    def test_fastpickle(self, benchmark, name, value):
        data = fastpickle.dumps(value)
        benchmark.group = f"loads[{name}]"
        benchmark(fastpickle.loads, data)


def test_circular_reference():
    a = []
    a.append(a)
    with pytest.raises(RecursionError):
        fastpickle.dumps(a)
