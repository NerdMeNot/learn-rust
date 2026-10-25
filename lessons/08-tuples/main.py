def min_max(xs: list[int]) -> tuple[int, int]:
    lo = hi = xs[0]
    for x in xs:
        if x < lo:
            lo = x
        if x > hi:
            hi = x
    return lo, hi


lo, hi = min_max([4, 9, 1, 7])
print(f"{lo} {hi}")

_, only_hi = min_max([3, 5])  # _ ignores a value
print(only_hi)
