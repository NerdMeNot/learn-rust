owned = "héllo"  # str is a sequence of characters, and immutable
view = owned[0:1]  # slicing makes a new string

s = "go"
s += " + rust"  # builds a new string

print(f"{owned} {view} {s}")
print(f"bytes={len(owned.encode())} chars={len(owned)}")
for c in owned:
    print(f"[{c}]", end="")
print()
