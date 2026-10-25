counts: dict[str, int] = {}
counts["cat"] = 0
for w in "cat cat".split(" "):
    counts[w] += 1
print(counts)
