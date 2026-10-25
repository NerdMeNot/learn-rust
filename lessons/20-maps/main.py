from collections import Counter

ages: dict[str, int] = {}
ages["ana"] = 30
ages["bo"] = 25

# get returns None when the key is missing
age = ages.get("ana")
if age is not None:
    print(f"ana is {age}")
else:
    print("no ana")

# a default on purpose: 0 when the key is missing
zed = ages.get("zed", 0)
print(f"zed is {zed}")

counts = Counter("a b a c a".split(" "))
print(f"a appears {counts['a']} times")
