class Point:
    def __init__(self, x):
        self.x = x


def report(kind, changed):
    print(f"{kind:<8}", "shares: the other name sees the change" if changed else "copy:   the other name is untouched")


# Every name leads to an object. "Copy or shared?" is "can that object change?"
n = 1
m = n
m += 1  # an int cannot change: += makes a new int and rebinds m
report("int", n == 2)

s = "hi"
t = s
t += "!"  # a str cannot change either
report("str", s == "hi!")

tup = (1, 2)
tup2 = tup
tup2 += (3,)  # a tuple cannot change
report("tuple", tup == (1, 2, 3))

lst = [1, 2]
lst2 = lst
lst2 += [3]  # a list can change, and += changes it in place
report("list", lst == [1, 2, 3])

lst3 = lst
lst3 = lst3 + [4]  # + builds a new list and rebinds lst3
print("after lst3 = lst3 + [4], lst is", lst)

d = {"a": 1}
d2 = d
d2["a"] = 99
report("dict", d["a"] == 99)

p = Point(1)
q = p
q.x = 99
report("object", p.x == 99)

# Identity: `is` asks "the same object?", == asks "equal contents?"
r = Point(99)
print("p is q:", p is q, " (same object)")
print("p is r:", p is r, " (same x, different object)")
u = [1, 2, 3]
print("lst == u:", lst == u, " lst is u:", lst is u, " (equal contents, different objects)")
print("id(p) == id(q):", id(p) == id(q))
