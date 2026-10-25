v: list[int] = []
v.append(10)
v.append(20)

w = [1, 2, 3]
w.append(4)

part = w[1:3]  # a slice of a list is a new list (a copy)
print(f"{v} {w} {part} len={len(w)}")
