def log(msg):
    print("log: " + msg)


n = 7

size = "big" if n > 5 else "small"
print("ternary:", size)

w = 3
h = 4
area = w * h
print("block:", area)

match 2:
    case 1:
        name = "one"
    case 2:
        name = "two"
    case _:
        name = "many"
print("switch:", name)

a, b = 1, 2
a, b = b, a
print("swap:", a, b)

log("first")
c = 2
print("comma:", c)


def add_one(x):
    y = x + 1
    return y


print("closure:", add_one(41), (lambda x: x + 1)(41))
