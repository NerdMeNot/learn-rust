from typing import NamedTuple


class Point(NamedTuple):
    x: int
    y: int


# nested, with * for the rest of a sequence
(a, b), (c, d) = Point(1, 2), Point(3, 4)
head, *tail = [10, 20, 30]
print(f"a={a} b={b} c={c} d={d} head={head} tail={tail}")

# unpacking only gives the same objects second names: the source stays whole
point = Point(5, 6)
x, y = point
print(f"point is still {point}")

# a for loop unpacks each item
for name, (x, y) in {"p": Point(1, 2), "q": Point(3, 4)}.items():
    print(f"{name}: x={x} y={y}")
