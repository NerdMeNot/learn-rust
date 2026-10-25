nums = [1, 2, 3]
for n in nums:
    n *= 10  # rebinds the name n; the list is untouched
print("after n *= 10:", nums)

rows = [[1], [2]]
for row in rows:
    row.append(0)  # changes the list that row names
print("after row.append:", rows)

print("zip stops at the shorter:", list(zip(["ana", "bo", "cy"], [31, 27])))
