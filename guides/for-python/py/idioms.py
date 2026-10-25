import json
from pathlib import Path


def show(label, value):
    print(f"{label}: {json.dumps(value, ensure_ascii=False)}")


class Guard:
    def __enter__(self):
        print("open")

    def __exit__(self, *exc):
        print("close")


def port_or_default(text):
    try:
        return int(text)
    except ValueError:
        return -1


nums = [1, 2, 3, 4]
words = ["pear", "fig", "apple"]
text = "a,b,c"
s = "héllo"
xs = [1, 2, 3, 4]
d = {"a": 1}
x = None
items = [7]
Path("note.txt").write_text("hello\n")

show("map", [n * n for n in nums])
show("filter", [n for n in nums if n % 2 == 0])
show("sorted by length", sorted(words, key=len))
show("enumerate", [f"{i}:{w}" for i, w in enumerate(words)])
show("contains", "b," in text)
show("split", text.split(","))
show("join", ", ".join(words))
show("chars", len(s))
show("bytes", len(s.encode()))
show("slice", xs[1:3])
show("get default", d.get("c", 0))
show("none default", x if x is not None else 5)
show("parse", port_or_default("4x"))
show("floor division", -7 // 2)
show("big int", 2**100)
show("truthy list", bool(items))
show("file", Path("note.txt").read_text())
show("format", f"{42:>5}")
with Guard():
    print("work")
