import queue

# maxsize=0 means "no limit" here; in Rust sync_channel(0) is the opposite, a rendezvous
q = queue.Queue(maxsize=0)
for i in range(1000):
    q.put_nowait(i)
print("Queue(maxsize=0) holds", q.qsize(), "items")

b = queue.Queue(maxsize=2)  # like sync_channel(2): put_nowait is try_send
b.put_nowait(1)
b.put_nowait(2)
try:
    b.put_nowait(3)
except queue.Full:
    print("put_nowait on a full queue: queue.Full")
try:
    queue.Queue().get_nowait()
except queue.Empty:
    print("get_nowait on an empty queue: queue.Empty")

# Python 3.13 added a close: shutdown(). Rust has no call for it; it drops the senders.
b.shutdown()
try:
    b.put(4)
except queue.ShutDown:
    print("put after shutdown: queue.ShutDown")
print("get after shutdown still drains:", b.get(), b.get())
try:
    b.get()
except queue.ShutDown:
    print("get when drained: queue.ShutDown")
