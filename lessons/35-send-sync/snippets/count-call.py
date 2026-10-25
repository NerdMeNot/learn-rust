import threading

count = 0
N = 2_000_000


def add_one(n):
    return n + 1


def work():
    global count
    for _ in range(N):
        count = add_one(count)  # read, call a function, write back


threads = [threading.Thread(target=work) for _ in range(4)]
for t in threads:
    t.start()
for t in threads:
    t.join()
print("expected:", 4 * N)
print("got:     ", count)
