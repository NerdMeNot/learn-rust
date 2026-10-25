import copy
from dataclasses import dataclass, field


@dataclass
class Box:
    items: list = field(default_factory=list)


try:
    int("4x")
    raise AssertionError("int accepted 4x")
except ValueError:
    pass
assert int("42") == 42

a = Box([1, 2])
b = copy.copy(a)
c = copy.deepcopy(a)
a.items.append(3)
assert b.items == [1, 2, 3]  # copy.copy shares the list
assert c.items == [1, 2]  # deepcopy does not
assert a == Box([1, 2, 3]) and a is not Box([1, 2, 3])  # a dataclass compares fields

# a list can change while it is a key, so it has no hash; the tuple holding one has none either
try:
    hash((1, [2]))
    raise AssertionError("hash accepted a list")
except TypeError:
    pass


class V:
    def __init__(self, x):
        self.x = x

    def __rmul__(self, other):  # the reflected operator: 2.5 * V(2)
        return V(other * self.x)


assert (2.5 * V(2)).x == 5.0
print("all checks passed")
