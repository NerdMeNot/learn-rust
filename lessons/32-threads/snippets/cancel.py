import threading

stop = threading.Event()


def work():
    stop.wait()  # a real worker checks stop.is_set() between pieces of work
    print("worker: the event was set")


t = threading.Thread(target=work)
t.start()
stop.set()
t.join()
