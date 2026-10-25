from collections.abc import Callable, Sequence
from typing import Any, Protocol, TypeVar


class Comparable(Protocol):
    def __gt__(self, other: Any) -> bool: ...


T = TypeVar("T", bound=Comparable)


def largest(items: Sequence[T]) -> T:
    best = items[0]
    for item in items:
        if item > best:
            best = item
    return best


def apply(f: Callable[[int], int], x: int) -> int:
    return f(x)


def find(d: dict[str, int], k: str) -> int | None:
    return d.get(k)


def read(path: str) -> str:
    with open(path) as fh:
        return fh.read()


class Log:
    def __init__(self) -> None:
        self.lines: list[str] = []

    def add(self, line: str) -> None:
        self.lines.append(line)

    def count(self) -> int:
        return len(self.lines)


def largest_new[T: Comparable](items: Sequence[T]) -> T:
    # Python 3.12 and later: the type parameter sits after the name
    best = items[0]
    for item in items:
        if item > best:
            best = item
    return best


assert largest_new([3, 9, 4]) == 9
assert str.upper("abc") == "abc".upper()
assert largest([3, 9, 4]) == 9
assert apply(lambda x: x + 1, 1) == 2
assert find({}, "a") is None
try:
    read("/nope")
    raise AssertionError("read")
except FileNotFoundError:
    pass  # not in the signature
log = Log()
log.add("hello")
assert log.count() == 1
# hints are not enforced when the program runs
bad: int = "text"
print("big:", 2**100)
print("python rows ok")
