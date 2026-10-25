# No interface to declare: any object with name() and area() will do.
class Square:
    def __init__(self, side):
        self.side = side

    def name(self):
        return "square"

    def area(self):
        return self.side * self.side


class Rect:
    def __init__(self, w, h):
        self.w = w
        self.h = h

    def name(self):
        return "rect"

    def area(self):
        return self.w * self.h


shapes = [Square(3), Rect(4, 6)]

total = 0
for s in shapes:
    print(f"{s.name()}: {s.area()}")
    total += s.area()
print(f"total: {total}")
