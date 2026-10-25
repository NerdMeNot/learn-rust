from dataclasses import dataclass


@dataclass
class User:
    name: str


def find_user(id: int) -> User | None:  # None plays the role of Option's None
    if id == 1:
        return User("ana")
    return None


u = find_user(1)
if u is not None:
    print(f"found {u.name}")
else:
    print("no user")

if (u := find_user(2)) is not None:
    print(f"found {u.name}")
else:
    print("no user")
