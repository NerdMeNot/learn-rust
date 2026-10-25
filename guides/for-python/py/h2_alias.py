a = [1, 2, 3]
b = a
b.append(4)
print(a)


def add_bonus(scores):
    scores.append(100)


add_bonus(a)
print(a)
