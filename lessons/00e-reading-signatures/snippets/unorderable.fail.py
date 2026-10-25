def largest(items):
    best = items[0]
    for item in items:
        if item > best:
            best = item
    return best


print("largest:", largest([3, "nine", 4]))
