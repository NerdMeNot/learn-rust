from dataclasses import dataclass, replace
from typing import NewType

# 1. NewType is for the type checker only: at run time a UserId is the int.
UserId = NewType("UserId", int)
OrderId = NewType("OrderId", int)


def load_user(user_id: UserId) -> str:
    return f"user #{user_id}"


print("NewType, wrong id:", load_user(OrderId(7)))  # runs; a checker such as mypy would flag it
print("type(UserId(7)):", type(UserId(7)).__name__)


# 2. A frozen dataclass that checks itself: an invalid Port never exists.
@dataclass(frozen=True)
class Port:
    number: int

    def __post_init__(self):
        if not 1 <= self.number <= 65535:
            raise ValueError(f"{self.number} is outside 1-65535")


for n in (8080, 0):
    try:
        print("Port:", Port(n))
    except ValueError as e:
        print("rejected:", e)


# 3. Defaults and keyword arguments, and replace() as "copy with".
@dataclass(frozen=True)
class Config:
    host: str = "localhost"
    port: int = 8080
    retries: int = 3


print(Config(port=9000))
print(replace(Config(), retries=0))
