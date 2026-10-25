import queue
import threading

# queue.Queue allows several consumers. One None marker stops only one of them.
q: queue.Queue[int | None] = queue.Queue()
stopped = []
still_waiting = []


def consume() -> None:
    try:
        if q.get(timeout=0.3) is None:
            stopped.append(1)
    except queue.Empty:
        still_waiting.append(1)  # nothing more arrived: it would have waited for ever


consumers = [threading.Thread(target=consume) for _ in range(2)]
for t in consumers:
    t.start()
q.put(None)  # one marker for two consumers
for t in consumers:
    t.join()
print("consumers that saw the marker:", len(stopped))
print("consumers still waiting:", len(still_waiting))

# Either side can shut a Queue down: after a consumer calls shutdown(), put raises
q2: queue.Queue[int] = queue.Queue()
q2.shutdown()
try:
    q2.put(1)
except queue.ShutDown:
    print("put after the consumer's shutdown(): queue.ShutDown")
