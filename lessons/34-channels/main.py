import queue
import threading

q: queue.Queue[str | None] = queue.Queue()


def produce() -> None:
    for i in range(1, 4):
        q.put(f"job {i}")
    q.put(None)  # no "close" on a Queue, so send a marker


threading.Thread(target=produce).start()

while (msg := q.get()) is not None:
    print(f"got {msg}")
