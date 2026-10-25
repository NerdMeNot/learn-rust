from dataclasses import dataclass


# order=True writes <, > from the fields in order; frozen=True makes it hashable.
@dataclass(order=True, frozen=True)
class Version:
    major: int = 0
    minor: int = 0

    # __str__ is the user-facing text (the dataclass's __repr__ is the debug text).
    def __str__(self):
        return f"v{self.major}.{self.minor}"


a = Version(1, 10)
b = Version(1, 9)
print("display:", a)
print("a == b:", str(a == b).lower())
print("a > b:", str(a > b).lower())

all_versions = sorted([a, b, Version()])
print("sorted:", " ".join(str(v) for v in all_versions))

seen = set()
for v in [a, b, Version(), a]:
    seen.add(v)
print("distinct:", len(seen))
