def area(width, height=1):
    return width * height


def parse(s):
    return int(s) + 1


def double(x):
    return x * 2


# there is no x++; x += 1 is the only form, and ++x / --x are legal and do nothing
x = 5
x += 1
assert x == 6
y = 5
assert ++y == 5 and --y == 5 and y == 5

# the conditional expression
label = "big" if x > 5 else "small"
assert label == "big"

# truthiness: empty and zero values count as false
assert not []
assert not ""
assert not 0
assert not None
assert [0]

# keyword and default arguments
assert area(3) == 3
assert area(width=3, height=4) == 12

# floor division, power, chained comparison, in, is
assert 7 // 2 == 3 and -7 // 2 == -4
assert 2**10 == 1024
n = 3
assert 1 < n < 5
assert n in [1, 2, 3]
assert None is None
assert True and not False

# an empty string is falsy, so `or` replaces it too
assert ("" or "Hello") == "Hello"

# and, or, not
assert (True and not False) or False

# one value can be an int or a float, and a function takes any type
assert 1 + 2.5 == 3.5
assert 1 == 1.0
assert double(2) == 4 and double("ab") == "abab"

# a name can change type
z = 1
z = "a"
assert z == "a"

# exceptions
try:
    parse("x")
    assert False, "no exception"
except ValueError:
    caught = True
assert caught
assert parse("41") == 42

# indexing a string gives a one-character string
assert "héllo"[0] == "h"
assert "héllo"[1] == "é"

# the match statement does not fall through and takes several values
out = ""
match 2:
    case 1 | 2:
        out = "small"
    case 3:
        out = "three"
assert out == "small"

# a global needs the global statement to be assigned from a function
visits = 0


def visit():
    global visits
    visits += 1


visit()
assert visits == 1

print("python: all checks passed")
