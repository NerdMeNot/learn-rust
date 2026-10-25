from dataclasses import dataclass


@dataclass
class User:
    name: str
    age: int
    active: bool


u = User(name="ana")
print(u.name)
