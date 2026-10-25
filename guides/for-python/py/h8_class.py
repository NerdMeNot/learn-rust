from dataclasses import dataclass


@dataclass
class Point:
    x: int
    y: int


p = Point(1, 2)
p.xx = 10  # a typo, and also a brand new attribute
p.z = 3
print(p.__dict__)
