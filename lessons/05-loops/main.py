# 1, 2, 3 (the end is not included)
for i in range(1, 4):
    print(f"for {i}")

n = 3
while n > 0:
    n -= 1

# no loop-with-value in Python: use while True and set a variable
tries = 0
while True:
    tries += 1
    if tries == 3:
        found = tries * 10
        break
print(f"n={n} found={found}")
