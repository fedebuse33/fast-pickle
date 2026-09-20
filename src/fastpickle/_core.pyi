def dumps(obj: object) -> bytes:
    """
    Serialize a Python object to bytes.

    Supported types: ``None``, ``bool``, ``int`` (signed 64-bit), ``float``,
    ``str``, ``bytes``, ``list``, ``tuple``, ``set``, ``dict``, and objects
    that have a ``__dict__``.

    Args:
        obj: The object to serialize.

    Returns:
        The serialized bytes.

    Raises:
        TypeError: If ``obj`` (or something nested in it) has an
            unsupported type, such as ``frozenset``.
        OverflowError: If an integer is outside the signed 64-bit range.
        RecursionError: If the nesting is deeper than 256 levels, or if
            the object contains a circular reference.
    """
    ...
def loads(data: bytes) -> object:
    """
    Deserialize bytes produced by ``dumps``.

    Objects that were serialized from custom classes come back as a plain
    ``(class_name, state_dict)`` tuple. To get real instances, subclass
    ``fastpickle.Serializable`` and use ``YourClass.loads`` instead.

    Args:
        data: The serialized bytes.

    Returns:
        The decoded Python object.

    Raises:
        ValueError: If ``data`` is malformed, truncated, or nested deeper
            than 256 levels.
        TypeError: If ``data`` is not a ``bytes`` object.
    """
    ...
