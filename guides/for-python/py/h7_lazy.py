def square(n):
    print("squaring", n)
    return n * n


nums = [1, 2, 3]
as_list = [square(n) for n in nums]
print("list built")
as_gen = (square(n) for n in nums)
print("generator made")
print(list(as_gen))
