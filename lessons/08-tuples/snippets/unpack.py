t = (1, 2, 3, 4)
first, *rest = t
print("first:", first, "rest:", rest, type(rest).__name__)

print("(1, 10) < (1, 9):", (1, 10) < (1, 9))

try:
    t[0] = 9
except TypeError as e:
    print("t[0] = 9:", e)
