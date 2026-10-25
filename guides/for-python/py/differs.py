import json


def show(label, value):
    print(f"{label}: {json.dumps(value, ensure_ascii=False)}")


for text in ["1_000", " 12 "]:
    show(f"int({text!r})", int(text))

xs = [1, 2, 3, 4]
show("xs[1:10]", xs[1:10])

words = ["éé", "abc"]
show("sorted(words, key=len)", sorted(words, key=len))
