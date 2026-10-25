from dataclasses import dataclass


@dataclass
class Point:
    """A point on a grid."""

    x: int
    y: int


p = Point(3, 4)  # a line comment
items = [1, 2, 3]
assert len(items) == 3  # stops with an error if false
print(f"point: Point {{ x: {p.x}, y: {p.y} }}")
print(f"items: {items}")
print(f"row: [{42:>5}] [{42:<5}] [{42:05}]")
pi = f"{3.14159:.2f}"
print(f"pi: {pi} {{done}}")
