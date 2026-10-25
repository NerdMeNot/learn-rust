import sys
import threading

N, THREADS = int(sys.argv[1]), 8
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
print("exact" if total == N * (N + 1) // 2 else "lost", end=" ")
