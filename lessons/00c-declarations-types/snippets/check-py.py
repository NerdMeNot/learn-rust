def must(ok, what):
    if not ok:
        raise AssertionError(what)


def add(a: int, b: int) -> int:
    return a + b


count: int = 5
wrong: int = "not an int"  # Python does not check the annotation when it runs
must(count == 5 and wrong == "not an int", "annotations are not enforced")

MAX_LEVEL = 99
MAX_LEVEL = 100  # a constant by convention only
must(MAX_LEVEL == 100, "constants")

a, b = 1, "two"
must(a == 1 and b == "two", "unpacking")
inc = lambda x: x + 1
must(inc(1) == 2 and add(1, 2) == 3, "lambda")

must(int(2.9) == 2 and int(-2.9) == -2 and float(7) == 7.0, "conversion")
must('R' == "R", "quotes")
must(int("42") == 42, "int of text")
from dataclasses import dataclass


@dataclass
class Point:
    x: float
    y: float


must(Point(1.0, 2.0).x == 1.0, "dataclass")
raw = r"C:\temp"
must(len(raw) == 7, "raw string")
must(1_000 == 1000 and 0xFF == 255 and 0b101 == 5 and 0o17 == 15, "literals")
must(b"a"[0] == 97 and list(range(5)) == [0, 1, 2, 3, 4], "bytes, range")
