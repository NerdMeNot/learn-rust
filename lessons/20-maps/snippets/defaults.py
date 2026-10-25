from collections import Counter, defaultdict

words = ["fig", "apple", "kiwi", "pear", "fig"]

by_length = {}
for w in words:
    by_length.setdefault(len(w), []).append(w)
print("setdefault:", by_length)

grouped = defaultdict(list)
for w in words:
    grouped[len(w)].append(w)
print("defaultdict:", dict(grouped))

print("Counter:", Counter(words))
