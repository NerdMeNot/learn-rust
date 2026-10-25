class Plain:
    def __init__(self, major, minor):
        self.major = major
        self.minor = minor


class WithEq(Plain):
    def __eq__(self, other):
        return (self.major, self.minor) == (other.major, other.minor)


notes = {Plain(1, 2): "stable"}
print("plain class:", notes.get(Plain(1, 2)))
print("equal by __eq__:", WithEq(1, 2) == WithEq(1, 2))
try:
    {WithEq(1, 2): "stable"}
except TypeError as e:
    print("with __eq__ only:", e)
