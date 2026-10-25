import threading

go = threading.Event()
never = threading.Event()


def worker():
    go.wait()
    print("worker: finished after main returned")


threading.Thread(target=worker).start()  # non-daemon by default
threading.Thread(target=never.wait, daemon=True).start()  # blocks forever

print("main: returning")
go.set()
