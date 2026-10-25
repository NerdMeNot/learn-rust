class Counter:
    def __init__(self):
        self.hits = 0


a = Counter()
b = a

a.hits += 1
b.hits += 1
print("hits:", a.hits)
