def longest(a, b):
    return a if len(a) >= len(b) else b


def join(*parts):
    # builds a new str at run time, not a constant
    return " ".join(parts)


s1 = join("a", "long", "first", "string")
if True:
    s2 = join("short", "string")
    result = longest(s1, s2)
    s3 = join("an", "even", "longer", "second", "string")
    other = longest(s1, s3)
print("longest:", result)
print("with a longer s3:", other)
