from hypothesis import given, settings, strategies as st


def wrap32(n):
    # Python ints never overflow, so wrap by hand to mimic a 32-bit int (Java's int, C's int32_t).
    return (n + 2**31) % 2**32 - 2**31


def average_naive(xs):
    total = 0
    for x in xs:
        total = wrap32(total + x)
    return int(total / len(xs))


# derandomize=True makes the run repeatable; database=None keeps it from writing .hypothesis/.
@settings(derandomize=True, database=None)
@given(st.lists(st.integers(-(2**31), 2**31 - 1), min_size=1, max_size=7))
def check_between_min_and_max(xs):
    assert min(xs) <= average_naive(xs) <= max(xs)


check_between_min_and_max()
