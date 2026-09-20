import pytest
import fastpickle

def test_circular_reference():
    a = []
    a.append(a)
    with pytest.raises(RecursionError):
        fastpickle.dumps(a)
