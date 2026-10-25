# What other languages write with ?:, i++, null and switch, written in Python.
from typing import Optional


def size(n: int) -> str:
    if n in (1, 2):
        return "small"
    if n == 3:
        return "medium"
    return "large"


def first_even(items: list[int]) -> Optional[int]:
    for x in items:
        if x % 2 == 0:
            return x
    return None


total = 0
for i in range(4):
    total += i
label = "big" if total > 5 else "small"
print(f"total: {total} ({label})")
print(f"size: {size(2)} {size(9)}")
even = first_even([3, 5, 8])
if even is not None:
    print(f"first even: {even}")
else:
    print("no even number")
none = first_even([1, 3])
print(f"none: {-1 if none is None else none}")
wide = total * 1_000_000_000
print(f"wide: {wide}")
