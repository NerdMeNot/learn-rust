names = ["ana", "bo"]

for n in names:
    print(f"hi {n}")

nums = [1, 2, 3]
for i in range(len(nums)):  # "n *= 10" would only rebind n, so assign through the index
    nums[i] *= 10

for i, n in enumerate(nums):
    print(f"{i}: {n}")
print(f"{len(names)} names, {nums}")
