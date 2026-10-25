import heapq
from collections import deque

h = [3, 9, 4, 1]
heapq.heapify(h)
print("heapq pops the smallest:", heapq.heappop(h))
big = [-n for n in [3, 9, 4, 1]]  # a max-heap by negating the numbers
heapq.heapify(big)
print("negated, the largest:", -heapq.heappop(big))
spread = [9, 3, 7, 1, 4]
heapq.heapify(spread)
print("heapify([9, 3, 7, 1, 4]) leaves the list as", spread)

print("heapq.nlargest(3, ...):", heapq.nlargest(3, iter([4, 9, 1, 7, 9, 3])))
jobs = [(2, "test"), (9, "alert"), (2, "write"), (9, "deploy")]
by_priority = [(-p, i, name) for i, (p, name) in enumerate(jobs)]  # (priority, counter, item)
heapq.heapify(by_priority)
print("(-priority, counter, name) pops:", [heapq.heappop(by_priority)[2] for _ in jobs])

q = deque(["b"])
q.appendleft("a")
print("deque:", q)
print("popleft:", q.popleft())

a, b = {1, 2, 3, 4}, {3, 4, 5}
print("a & b:", a & b, " a | b:", a | b, " a - b:", a - b, " a ^ b:", a ^ b)

print("dict.fromkeys keeps first-seen order:", list(dict.fromkeys([3, 1, 3, 2, 1])))

import bisect
keys = []
for k in [40, 10, 25]:
    bisect.insort(keys, k)  # a sorted list, kept sorted by hand
print("insort:", keys, "bisect_left(25):", bisect.bisect_left(keys, 25))

try:
    import sortedcontainers  # a third-party package, not in the standard library
    print("sortedcontainers found")
except ImportError as e:
    print("no sorted map in the standard library:", e)
