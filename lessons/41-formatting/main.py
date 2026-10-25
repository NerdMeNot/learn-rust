from dataclasses import dataclass
from pprint import pprint


class Money:
    def __init__(self, cents: int):
        self.cents = cents

    # __str__: how Money looks with print and f"{}"
    def __str__(self) -> str:
        return f"${self.cents // 100}.{self.cents % 100:02}"


@dataclass  # writes __repr__ for you, like derive(Debug)
class Item:
    name: str
    qty: int


n = 42
print(f"[{n:>6}] [{n:<6}] [{n:^6}] [{n:06}]")
print(f"{3.14159:.2f} {1234.5:e} {n:#x} {n:#b}")

price = Money(1999)
print(f"total: {price}")

label = f"{price} each"  # an f-string builds a str
print(f"{label} ({len(label)} chars)")

item = Item("pen", 3)
print(f"{item.qty} x {item.name}")
print(f"{item!r}")       # repr, written by @dataclass
pprint(item, width=20)  # pretty print, splits when it doesn't fit
