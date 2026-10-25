class Square:
    def area(self):
        return 4


class Rock:
    pass


shapes = [Square(), Rock()]
for shape in shapes:
    print(shape.area())
