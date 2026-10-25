import threading

lock = threading.Lock()
data = [1, 2, 3]


def work():
    try:
        with lock:
            data.append(4)
            raise RuntimeError("worker failed halfway")
    except RuntimeError as e:
        print("worker died:", e)


worker = threading.Thread(target=work)
worker.start()
worker.join()

with lock:  # not poisoned: leaving the with block released the lock
    print("main got the lock; data:", data)
