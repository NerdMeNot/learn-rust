d = {"one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6}
print("keys:", list(d))

del d["one"]
d["one"] = 1
print("after deleting and adding 'one':", list(d))
