import threading

results: list[int] = []
lock = threading.Lock()


def work(i: int) -> None:
    with lock:  # lock, append, unlock
        results.append(i * 10)


threads = [threading.Thread(target=work, args=(i,)) for i in range(3)]
for t in threads:
    t.start()
for t in threads:
    t.join()

results.sort()
print(results)
