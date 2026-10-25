def total(xs: list[int]) -> int:
    result = 0
    for x in xs:
        result += x
    return result


nums = [1, 2, 3]
t = total(nums)
print(t, nums)
