assert list(range(4)) == [0, 1, 2, 3]
assert list(range(1, 4)) == [1, 2, 3]
assert list(range(0, 10, 3)) == [0, 3, 6, 9]
assert list(range(3, -1, -1)) == [3, 2, 1, 0]
assert list(range(9, 0, -3)) == [9, 6, 3]
assert list(enumerate(["a", "b"])) == [(0, "a"), (1, "b")]

n = 0
while True:
    n += 1
    if n == 3:
        break
assert n == 3

# the loop variable is a fresh name each pass: changing it lasts one pass (like Rust)
seen = []
for i in range(5):
    if i == 1:
        i += 2
    seen.append(i)
assert seen == [0, 3, 2, 3, 4]
assert list(range(len([5, 6, 7]))) == [0, 1, 2]
print("ok")
