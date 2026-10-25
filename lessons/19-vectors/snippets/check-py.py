v = [3, 1, 2]
v.append(4)
assert v.pop() == 4
try:
    [].pop()
    raise SystemExit("pop on empty list did not raise")
except IndexError:
    pass

v.extend([7, 8])
assert v == [3, 1, 2, 7, 8]
assert v.pop(1) == 1 and v == [3, 2, 7, 8]
del v[0]
assert v == [2, 7, 8]
v.insert(1, 5)
assert v == [2, 5, 7, 8]
assert v[-1] == 8 and v[1:3] == [5, 7]

assert 5 in v and v.index(7) == 2
try:
    v.index(99)
    raise SystemExit("index did not raise")
except ValueError:
    pass

s = sorted(v, reverse=True)
assert s == [8, 7, 5, 2] and v == [2, 5, 7, 8]
words = ["kiwi", "fig", "banana"]
words.sort(key=len)
assert words == ["fig", "kiwi", "banana"]
v.reverse()
assert v == [8, 7, 5, 2]
c = v.copy()
c.clear()
assert c == [] and v == [8, 7, 5, 2] and v[:] == v and len(v) == 4
assert [0] * 3 == [0, 0, 0]
z = [1, 2, 3, 2]
z.remove(2)
assert z == [1, 3, 2], "remove(x) removes the first item EQUAL to x"
e1, e2 = [1], [5]
e1.extend(e2)
assert e2 == [5] and e1 == [1, 5]
assert sum(v) == 22 and min(v) == 2 and max(v) == 8
part = v[1:3]
part[0] = 99
assert v[1] == 7, "slicing a list copies"
print("all checks passed")
