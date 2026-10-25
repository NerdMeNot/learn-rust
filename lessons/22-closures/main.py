factor = 3
times = lambda x: x * factor  # reads factor
print(times(5))

count = 0


def inc() -> None:
    global count  # needed to change a variable from outside
    count += 1


inc()
inc()
print(f"count={count}")
