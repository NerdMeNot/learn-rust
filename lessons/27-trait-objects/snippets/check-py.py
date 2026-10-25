from abc import ABC, abstractmethod
from typing import Protocol


class Shape(ABC):
    @abstractmethod
    def area(self): ...


class Square(Shape):
    def __init__(self, side):
        self.side = side

    def area(self):
        return self.side * self.side


class Rect:  # no base class at all
    def area(self):
        return 24


class HasArea(Protocol):
    def area(self) -> int: ...


shapes = [Square(3), Rect()]
assert isinstance(shapes[0], Square) and not isinstance(shapes[1], Square)
assert isinstance(shapes[0], Shape) and not isinstance(shapes[1], Shape)
assert sum(s.area() for s in shapes) == 33

# the abstract method is enforced when you instantiate, at run time
try:
    Shape()
    raise SystemExit("should not get here")
except TypeError:
    pass

# a None in the list is allowed
assert [Square(1), None][1] is None
# an attribute, not a method, on the shared shape
class Named:
    def __init__(self):
        self.name = "named"


assert Named().name == "named"
print("ok")
