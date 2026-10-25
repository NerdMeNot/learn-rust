class Id:
    def __init__(self, n):
        self.n = n

    def __str__(self):
        return f"#{self.n}"


print(f"[{Id(42)!s:>8}]")  # convert to str first, then pad
try:
    print(f"[{Id(42):>8}]")
except TypeError as e:
    print("TypeError:", e)
print(f"{0.5:.0f} {1.5:.0f} {2.5:.0f}")
print(f"{1234567.5:,.2f}")

x = 5
print(f"{1234.5:e} {-255:x} [{True:5}] {x=}")
try:
    print(f"{7:.3}")
except ValueError as e:
    print("ValueError:", e)
import pprint
pprint.pprint({"user": "ada", "roles": ["admin", "dev"]}, width=20)
