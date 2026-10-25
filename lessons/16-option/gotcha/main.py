from dataclasses import dataclass


@dataclass
class User:
    name: str


def find_user(id: int) -> User | None:
    return User("ana") if id == 1 else None


u = find_user(2)
print(u.name)
