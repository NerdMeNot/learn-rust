from dataclasses import dataclass


@dataclass
class Circle:
    r: float


@dataclass
class Rect:
    w: float
    h: float


@dataclass
class Triangle:  # new!
    b: float
    h: float


def area(s: Circle | Rect | Triangle) -> float:
    match s:
        case Circle(r):
            return 3.14 * r * r
        case Rect(w, h):
            return w * h


print(area(Triangle(2.0, 3.0)))
