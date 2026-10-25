from wordstats import Counter

c = Counter()
for word in "the cat saw the other cat and The dog".split():
    c.add(word)

print(c)
print(c.distinct)
print(c.top(2))

other = Counter()
other.add("dog")
other.add("dog")
c.merge(other)
print(c.top(3))

try:
    c.merge(c)
except RuntimeError as e:
    print("RuntimeError:", e)

try:
    c.distinct = 5
except AttributeError as e:
    print("AttributeError:", e)
