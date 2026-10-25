line = "  name=Ana, age=30, city=Lisbon  "

clean = line.strip()  # a new str: Python strings never change in place
print(f"[{clean}]")

# split on ',', strip each piece, split once on '='
for field in clean.split(","):
    key, sep, value = field.strip().partition("=")
    if sep:
        print(f"{key:>5} -> {value}")

# text to number: int raises ValueError on bad input
try:
    age = int("30")
except ValueError:
    age = 0
print(f"next year: {age + 1}")

shout = clean.upper().replace(", ", " | ")
print(shout)

print(f"starts with name: {clean.startswith('name')}")
print(f"position of age: {clean.find('age')}")  # -1 when missing

words = ["build", "with", "rust"]
print("-".join(words))

# collect pieces in a list and join once, like strings.Builder
parts = []
for i in range(1, 4):
    parts.append(f"{i};")
print("".join(parts))
