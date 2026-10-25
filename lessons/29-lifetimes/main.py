# Nothing to annotate: an object stays alive while anything refers to it.
def longest(a, b):
    return a if len(a) >= len(b) else b


class Excerpt:
    def __init__(self, part):
        self.part = part


novel = "Call me Ishmael. Some years ago..."
first = novel.split(".")[0]
excerpt = Excerpt(first)
l = longest(excerpt.part, "short")
print("longest:", l)
print("excerpt length:", len(excerpt.part))
