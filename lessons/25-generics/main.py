from typing import Protocol, Self


class Comparable(Protocol):
    def __gt__(self, other: Self, /) -> bool: ...


def largest[T: Comparable](items: list[T]) -> T:
    best = items[0]
    for item in items:
        if item > best:
            best = item
    return best


print(largest([3, 9, 2]))
print(largest(["kiwi", "apple", "pear"]))
