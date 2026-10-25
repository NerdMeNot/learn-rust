def largest(items):
    best = items[0]
    for item in items:
        if item > best:
            best = item
    return best


numbers = [3, 9, 4]
words = ["pear", "fig", "apple"]
print("largest number:", largest(numbers))
print("largest word:", largest(words))
