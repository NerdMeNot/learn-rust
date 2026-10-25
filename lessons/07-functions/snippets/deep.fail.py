import sys


def depth(n):
    return 0 if n == 0 else 1 + depth(n - 1)


print("recursion limit:", sys.getrecursionlimit())
print("depth(500):", depth(500))
print("depth(100_000_000):", depth(100_000_000))
