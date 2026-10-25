import threading

results = []


def work(i):
    results.append(i * 10)  # no lock!


threads = [threading.Thread(target=work, args=(i,)) for i in range(3)]
for t in threads:
    t.start()
for t in threads:
    t.join()
print(sorted(results))
