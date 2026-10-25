class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __repr__(self):
        return f"({self.x}, {self.y})"


a = Point(1, 2)
b = a  # b is another name for the same object
b.x = 99
print("a =", a)
print("b =", b)

# To get a separate point, you build one.
c = Point(a.x, a.y)
c.x = 7
print("a after c.x = 7:", a)
