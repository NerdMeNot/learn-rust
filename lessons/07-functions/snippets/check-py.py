def total_with_tax(price):
    rate = 20

    def tax(p):
        return p * rate // 100  # a nested def may use `rate`

    return price + tax(price)


def area(w, h=1):
    return w * h


def total(*args, **kwargs):
    return sum(args) + sum(kwargs.values())


def divmod_pair(a, b):
    return a // b, a % b


def no_return():
    1 + 1


def fill(xs):
    xs.append("x")


def calls_later():
    return defined_later()


def defined_later():
    return 5


assert calls_later() == 5  # a body may call a def that comes later
try:
    not_yet()
    raised = False
except NameError:
    raised = True
assert raised  # at module level, a call must come after the def


def not_yet():
    return 1


xs = []
fill(xs)
assert xs == ["x"]  # the caller sees the callee's change
assert total_with_tax(50) == 60
assert area(3) == 3 and area(3, 4) == 12 and area(h=5, w=2) == 10
assert total(1, 2, x=3) == 6
q, r = divmod_pair(7, 2)
assert (q, r) == (3, 1)
assert no_return() is None


def hinted(x: int) -> int:
    return x


assert hinted("not an int") == "not an int"  # hints are not checked at run time
print("all checks passed")
