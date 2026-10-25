from dataclasses import dataclass


@dataclass
class User:
    name: str
    age: int


a = User("ana", 30)
b = User("ana", 30)
print("a == b:", a == b)
print("a is b:", a is b)

a.nickname = "an"  # a new attribute on one object: allowed
print("nickname:", a.nickname)

c = User(name=1, age="x")  # the annotations are not checked when it runs
print("c:", c)
