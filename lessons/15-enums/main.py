from dataclasses import dataclass


@dataclass
class Circle:
    r: float


@dataclass
class Rect:
    w: float
    h: float


Shape = Circle | Rect


def area(s: Shape) -> float:
    match s:
        case Circle(r):
            return 3.14 * r * r
        case Rect(w, h):
            return w * h


for s in [Circle(1.0), Rect(2.0, 3.0)]:
    print(area(s))
