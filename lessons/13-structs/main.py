from dataclasses import dataclass


@dataclass  # also gives a readable repr, like #[derive(Debug)]
class User:
    name: str
    age: int
    active: bool


u = User(name="ana", age=30, active=True)
print(f"{u.name} is {u.age}, active={u.active}")
print(u)
