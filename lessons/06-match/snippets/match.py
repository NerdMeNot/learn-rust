from dataclasses import dataclass


@dataclass
class Point:
    x: int
    y: int


def classify(n):
    match n:
        case 0:
            return "zero"
        case (1 | 2 | 3) as few:
            return f"few ({few})"
        case n if n < 0:
            return "negative"
        case _:
            return "many"


def ends(items):
    match items:
        case []:
            return "empty"
        case [only]:
            return f"only {only}"
        case [first, *_, last]:
            return f"{first} to {last}"


def place(p):
    match p:
        case Point(x=0, y=0):
            return "origin"
        case Point(x=0):
            return "on the y axis"
        case {"mode": m}:
            return f"a dict with mode {m}"


for n in [0, 3, 12, -1]:
    print(n, "->", classify(n))
print(ends([]), ends([7]), ends([1, 2, 3, 4]), ends((5, 6)), sep=" | ")
print(place(Point(0, 0)), place(Point(0, 4)), place({"mode": "fast"}), sep=" | ")
