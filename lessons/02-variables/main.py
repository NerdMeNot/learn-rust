name = "gopher"
count = 1
count += 1

count = count * 10  # Python just reassigns; there is no separate "shadowing"
print(f"{name} {count}")
