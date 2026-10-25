import sys
import threading

THREADS, ROUNDS = 8, 1_000_000


class Box:
    n = 0


box = Box()
d = {"k": 0}
lst = [0]
x = 0
lock = threading.Lock()


def one():
    return 1


def attr():
    for _ in range(ROUNDS):
        box.n += 1


def item():
    for _ in range(ROUNDS):
        d["k"] += 1


def index():
    for _ in range(ROUNDS):
        lst[0] += 1


def spelled_out():
    global x
    for _ in range(ROUNDS):
        x = x + 1


def call_between():
    global x
    for _ in range(ROUNDS):
        x += one()


def call_between_locked():
    global x
    for _ in range(ROUNDS):
        with lock:
            x += one()


def run(work, read):
    global x
    x = 0
    box.n = 0
    d["k"] = 0
    lst[0] = 0
    threads = [threading.Thread(target=work) for _ in range(THREADS)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    return "exact" if read() == THREADS * ROUNDS else "lost updates"


print("python", sys.version.split()[0], "GIL enabled:", sys._is_gil_enabled())
print("obj.n += 1:            ", run(attr, lambda: box.n))
print("d[k] += 1:             ", run(item, lambda: d["k"]))
print("lst[0] += 1:           ", run(index, lambda: lst[0]))
print("x = x + 1:             ", run(spelled_out, lambda: x))
print("x += one():            ", run(call_between, lambda: x))
print("x += one() under a Lock:", run(call_between_locked, lambda: x))
