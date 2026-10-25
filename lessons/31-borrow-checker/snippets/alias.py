a = [1, 2, 3]
b = a
b.append(4)
print("a:", a)
print("same object:", a is b)

c = list(a)
c.append(5)
print("a:", a, "c:", c)
