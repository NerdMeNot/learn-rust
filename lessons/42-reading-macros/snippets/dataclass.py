from dataclasses import dataclass
import inspect


def announce(cls):
    print("decorator runs for", cls.__name__)
    return cls


print("before the class statement")


@announce
@dataclass
class Point:
    x: int
    y: int


print("after the class statement")
print("generated __init__:", inspect.signature(Point.__init__))
print("generated __repr__:", Point(1, 2))
