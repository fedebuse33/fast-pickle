import pytest
import fastpickle

def test_int_overflow():
    with pytest.raises(OverflowError):
        fastpickle.dumps(2**70)

def test_int_limits():
    for n in (2**63 - 1, -(2**63)):
        assert fastpickle.loads(fastpickle.dumps(n)) == n
