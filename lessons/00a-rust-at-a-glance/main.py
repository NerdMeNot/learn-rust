# Greet someone, then add up 1 to 4.
def greet(name: str) -> str:
    return f"Hello, {name}!"


def add(a: int, b: int) -> int:
    return a + b


print(greet("Ferris"))
total = 0
for n in range(1, 5):
    total = add(total, n)
print(f"total: {total}")
