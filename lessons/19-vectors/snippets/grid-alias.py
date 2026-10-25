shared = [[0] * 3] * 2           # two names for ONE row
shared[1][2] = 7
print("[[0] * 3] * 2:         ", shared)

separate = [[0] * 3 for _ in range(2)]
separate[1][2] = 7
print("list comprehension:    ", separate)
