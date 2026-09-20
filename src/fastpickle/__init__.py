from typing import Self, cast
from fastpickle._core import dumps, loads



class Serializable:
    """
    Base class for objects that can be serialized with fast-pickle.

    Subclasses are registered automatically by their ``__qualname__``, which
    lets ``loads`` find the right class when decoding. Only attributes stored
    in the instance ``__dict__`` are serialized, and they must be made of
    supported types (see ``fastpickle.dumps``).

    Example:
        >>> class User(Serializable):
        ...     pass
        >>> user = User()
        >>> user.name = "Brad"
        >>> User.loads(user.dumps()).name
        'Brad'

    Note:
        Two subclasses with the same ``__qualname__`` overwrite each other in
        the registry, even if they live in different modules.
    """
    _registry: dict[str, type["Serializable"]] = {}

    def __init_subclass__(cls, **kwargs):
        super().__init_subclass__(**kwargs)
        cls._registry[cls.__qualname__] = cls

    def dumps(self) -> bytes:
        """
        Serialize this object to bytes.

        Returns:
            The serialized object, including its class name and the contents
            of its ``__dict__``.

        Raises:
            TypeError: If an attribute has an unsupported type.
            OverflowError: If an attribute is an integer outside the signed
                64-bit range.
            RecursionError: If the object is nested deeper than 256 levels,
                or contains a circular reference.
        """
        return dumps(self)

    @classmethod
    def loads(cls, stream: bytes) -> Self:
        """
        Rebuild an object from bytes produced by ``dumps``.

        The instance is created with ``__new__``, so ``__init__`` is **not**
        called: the ``__dict__`` is simply filled with the decoded state.

        Args:
            stream: Bytes produced by ``Serializable.dumps``.

        Returns:
            An instance of the class stored in the payload, which must be
            ``cls`` or one of its subclasses.

        Raises:
            ValueError: If ``stream`` is malformed or truncated.
            KeyError: If the class stored in the payload is not registered.
            TypeError: If the stored class is not a subclass of ``cls``.

        Warning:
            Do not decode data you do not trust. Decoding never executes
            code, but it does instantiate any registered subclass with
            whatever state the payload contains.
        """
        class_name, state = cast(tuple[str, dict], loads(stream))
        target_cls = Serializable._registry[class_name]
        if not issubclass(target_cls, cls):
            raise TypeError(f"{class_name} is not a {cls.__name__}")
        instance = target_cls.__new__(target_cls)
        instance.__dict__.update(state)
        return instance

__all__ = ["dumps", "loads", "Serializable"]
