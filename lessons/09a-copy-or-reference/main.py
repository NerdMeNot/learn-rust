ages = [30, 40, 50]

# A copy: `age` is a name for an int, and += builds a new int.
for age in ages:
    age += 1
    print("the copy is now", age)
print("ages after the copy loop:    ", ages)

# The elements themselves: assign into the list.
for i in range(len(ages)):
    ages[i] += 1
print("ages after the in-place loop:", ages)
