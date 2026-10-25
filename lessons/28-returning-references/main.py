def make_greeting(name: str) -> str:
    return f"hello {name}"


def first_word(text: str) -> str:  # returns a new string
    return text.split(" ")[0]


g = make_greeting("rust")
w = first_word(g)
print(f"{g} / {w}")
