# Backs the Python and TypeScript cells of the table in lesson.mdx. Never shown.
from typing import Protocol, Self


class Comparable(Protocol):
    def __gt__(self, other: Self, /) -> bool: ...


class Stack[T]:
    def __init__(self) -> None:
        self.items: list[T] = []

    def push(self, item: T) -> None:
        self.items.append(item)


s = Stack[int]()
s.push(1)
assert s.items == [1]
assert type(s) is type(Stack[str]())
assert type([1]) is type(["a"])
mixed: list[int | str] = [1, "a"]
assert len(mixed) == 2
print("ok")
