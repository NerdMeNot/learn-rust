nums = [1, 2, 3, 4, 5, 6]

evens_squared = [n * n for n in nums if n % 2 == 0]
total = sum(nums)
print(f"{evens_squared} {total}")
