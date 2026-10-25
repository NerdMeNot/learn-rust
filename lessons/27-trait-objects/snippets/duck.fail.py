class Square:
    def area(self):
        return 9


class Rock:
    pass  # no area() method


shapes = [Square(), Rock()]  # building the list is fine
print("list built:", len(shapes))
for s in shapes:
    print("area:", s.area())
