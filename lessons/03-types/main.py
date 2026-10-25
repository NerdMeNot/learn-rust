a: int = 7  # Python ints have no fixed size
b: int = 10
price: float = 2.5
ok: bool = True

total = a + b
cost = a * price  # int * float gives a float automatically
print(f"{total} {cost} {str(ok).lower()}")
