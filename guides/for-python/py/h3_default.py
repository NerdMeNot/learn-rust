def add_item(item, bucket=[]):
    bucket.append(item)
    return bucket


print(add_item("a"))
print(add_item("b"))


def add_item_fixed(item, bucket=None):
    if bucket is None:
        bucket = []
    bucket.append(item)
    return bucket


print(add_item_fixed("a"))
print(add_item_fixed("b"))
