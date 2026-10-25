fs = [lambda: i for i in range(3)]
print("late binding:", [f() for f in fs])

gs = [lambda i=i: i for i in range(3)]
print("default argument:", [g() for g in gs])

x = 1
show = lambda: print("x:", x)
x = 2
show()
