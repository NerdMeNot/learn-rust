# Backs the Python cells of the tables in lesson.mdx. Never shown.
v = ["ana", "bo", "cy"]
assert list(enumerate(v)) == [(0, "ana"), (1, "bo"), (2, "cy")]
assert list(zip(v, [31, 27])) == [("ana", 31), ("bo", 27)]
assert list(reversed(v)) == ["cy", "bo", "ana"]
assert v[::2] == ["ana", "cy"]
try:
    list(zip(v, [31, 27], strict=True))
    raise SystemExit("strict zip should raise")
except ValueError:
    pass
# removing the current item skips the next one, silently
v = [1, 2, 3, 4]
saw = []
for x in v:
    saw.append(x)
    if x == 2:
        v.remove(x)
assert v == [1, 3, 4] and saw == [1, 2, 4]
# appending: the loop reaches the new item
v = [1, 2, 3, 4]
saw = []
for x in v:
    saw.append(x)
    if x == 2:
        v.append(99)
assert saw == [1, 2, 3, 4, 99]
# a string loops over code points; a dict keeps insertion order
assert list("h\u00e9llo\U0001F600") == ["h", "\u00e9", "l", "l", "o", "\U0001F600"]
assert list({"b": 1, "a": 2, "c": 3}) == ["b", "a", "c"]
print("ok")
