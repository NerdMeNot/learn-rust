# Remove 3 (the second-to-last item) while looping: no error, and 4 is skipped.
items = [1, 2, 3, 4]
seen = []
for x in items:
    seen.append(x)
    if x == 3:
        items.remove(3)
print("remove in a loop: saw", seen, "list is", items)

# Add 99 when the loop sees 2: no error, and the loop reaches 99 too.
items = [1, 2, 3, 4]
seen = []
for x in items:
    seen.append(x)
    if x == 2:
        items.append(99)
print("add in a loop:    saw", seen, "list is", items)

# A dict does notice.
d = {"a": 1, "b": 2}
try:
    for key in d:
        d["c"] = 3
except RuntimeError as e:
    print("dict:", e)
