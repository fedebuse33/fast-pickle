from typing import Self, cast
from fastpickle._core import dumps, loads



class Serializable:
    _registry: dict[str, type["Serializable"]] = {}

    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        cls._registry[cls.__qualname__] = cls

    def dumps(self) -> bytes:
        return dumps(self)

    @classmethod
    def loads(cls, stream: bytes) -> Self:
        class_name, state = cast(tuple[str, dict], loads(stream))
        target_cls = Serializable._registry[class_name]
        if not issubclass(target_cls, cls):
            raise TypeError(f"{class_name} is not a {cls.__name__}")
        instance = target_cls.__new__(target_cls)
        instance.__dict__.update(state)
        return instance

__all__ = ["dumps", "loads", "Serializable"]
