import threading

names = ["a", "b", "c"]
t = threading.Thread(target=lambda: print(len(names)))
t.start()
t.join()
