import queue
import threading
import time

q = queue.Queue()


def receive():
    got = q.get()
    time.sleep(0.01)
    print("receiver sees", got)


r = threading.Thread(target=receive)
r.start()
msg = [1, 2]
q.put(msg)
msg[0] = 99  # sender keeps changing what it already sent
r.join()
