import sys
import threading

print("GIL enabled:", sys._is_gil_enabled())
count = 0
N = 2_000_000


def work():
    global count
    for _ in range(N):
        count += 1  # read, add, write back


threads = [threading.Thread(target=work) for _ in range(4)]
for t in threads:
    t.start()
for t in threads:
    t.join()
print("expected:", 4 * N)
print("got:     ", count)
