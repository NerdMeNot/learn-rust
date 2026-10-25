from typing import NamedTuple


class Point(NamedTuple):
    x: int
    y: int


# unpacking is by position
x, y = Point(3, -2)
print(f"x = {x}, y = {y}")

for i, c in enumerate("abc"):
    print(f"{i}: {c}")

key, sep, value = "mode=fast".partition("=")
if sep:
    print(f"{key} -> {value}")

# * collects the items in the middle
first, *_, last = [10, 20, 30, 40]
print(f"first = {first}, last = {last}")
