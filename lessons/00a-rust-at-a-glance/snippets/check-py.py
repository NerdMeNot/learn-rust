from typing import Final


def must(ok, what):
    if not ok:
        raise AssertionError(what)


def add(a: int, b: int) -> int:
    return a + b


x = 5
x = 6
Y: Final = 5
must(x == 6 and Y == 5, "variables")
must(add(2, 3) == 5, "def")

seen = []
for i in range(3):
    seen.append(i)
must(seen == [0, 1, 2], "range")

joined = ""
for w in ["a", "b"]:
    joined += w
must(joined == "ab", "for in")

n = 1
match n:
    case 1:
        s = "one"
    case _:
        s = "many"
must(s == "one", "match")

inc = lambda x: x + 1
must(inc(1) == 2, "lambda")

name = "Ana"
age = 30
print(f"{name} is {age}")

try:
    int("x")
    raise AssertionError("int did not raise")
except ValueError:
    pass
print("all python cells run and hold")
