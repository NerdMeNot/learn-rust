import sys
import threading
import time

print("GIL enabled:", sys._is_gil_enabled())


def waiting():
    time.sleep(0.2)  # what waiting for I/O looks like to the interpreter


def counting():
    n = 0
    for _ in range(3_000_000):  # Python bytecode, which needs the lock
        n += 1


def run_four_threads(work):
    """Wall-clock seconds and CPU seconds (all threads added up) for four threads at once."""
    threads = [threading.Thread(target=work) for _ in range(4)]
    wall, cpu = time.perf_counter(), time.process_time()
    for t in threads:
        t.start()
    for t in threads:
        t.join()
    return time.perf_counter() - wall, time.process_time() - cpu


wall, _ = run_four_threads(waiting)
print("4 threads that wait took less than 2 sleeps:", wall < 0.4)  # one after another: 4

wall, cpu = run_four_threads(counting)
# more CPU time than wall time would mean that two threads ran at the same moment
print("4 threads counting ran at the same moment:", cpu > 1.3 * wall)
