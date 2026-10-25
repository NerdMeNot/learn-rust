def total(xs: list[int]) -> int:  # gets a reference to the same list
    return sum(xs)


nums = [1, 2, 3]
t = total(nums)
print(f"{t} {nums}")
