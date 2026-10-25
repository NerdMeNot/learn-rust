import threading

N, THREADS = 8_000_000, 8  # many numbers, so each thread runs long enough to be interrupted
total = 0  # shared by every thread


def term(i):
    return i


def add(lo, hi):
    global total
    for i in range(lo, hi):
        total = total + term(i)  # read total, call a function, write total


step = N // THREADS
threads = [threading.Thread(target=add, args=(p * step + 1, (p + 1) * step + 1)) for p in range(THREADS)]
for t in threads:
    t.start()
for t in threads:
    t.join()
print("total:", total, "expected:", N * (N + 1) // 2)
