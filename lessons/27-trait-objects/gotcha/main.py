class Square:
    def __init__(self, side):
        self.side = side

    def area(self):
        return self.side * self.side


class Rect:
    def __init__(self, w, h):
        self.w = w
        self.h = h

    def area(self):
        return self.w * self.h


shapes = [Square(3), Rect(4, 6)]

print("total:", sum(s.area() for s in shapes))
