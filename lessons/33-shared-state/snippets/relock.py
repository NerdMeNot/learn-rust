import threading

plain = threading.Lock()
plain.acquire()
print("Lock, second acquire without blocking:", plain.acquire(blocking=False))

reentrant = threading.RLock()
reentrant.acquire()
print("RLock, second acquire without blocking:", reentrant.acquire(blocking=False))
