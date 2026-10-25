import heapq
from collections import Counter, deque

text = "the cat saw the dog and the cat ran"

# Counter: count words, then sort by count and then by word
counts = Counter(text.split())
top = sorted(counts.items(), key=lambda p: (-p[1], p[0]))[:3]
print(f"top: {top}")

# no sorted dict in std: dicts keep insertion order, so sort the items
by_name = {"zoe": 31, "ana": 30, "mia": 28}
print(f"sorted: {dict(sorted(by_name.items()))}")

# set: unique items, fast "is it in there?"
seen = set(text.split())
print(f"{len(seen)} unique words, has dog: {'dog' in seen}")

# deque: a queue. Append at the back, pop at the front, both cheap.
queue = deque(["a", "b"])
queue.append("c")
first = queue.popleft()
print(f"served {first!r}, waiting {list(queue)}")

# heapq turns a plain list into a heap that pops the smallest first
heap: list[int] = []
for n in [5, 1, 8, 3]:
    heapq.heappush(heap, n)
print("smallest first:", end="")
while heap:
    print(f" {heapq.heappop(heap)}", end="")
print()
