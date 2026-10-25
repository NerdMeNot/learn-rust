import threading


def work():
    raise RuntimeError("worker failed")


t = threading.Thread(target=work, name="worker-1")
t.start()
t.join()  # join does not raise
print("main: join returned normally")
