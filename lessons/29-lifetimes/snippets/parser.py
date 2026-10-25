class Parser:
    def __init__(self, text):
        self.rest = text

    def next_word(self):
        trimmed = self.rest.lstrip()
        if not trimmed:
            return None
        word, _, self.rest = trimmed.partition(" ")
        return word


parser = Parser("lifetimes are regions")
a = parser.next_word()
b = parser.next_word()  # a is still usable: no annotation needed
print("a:", a)
print("b:", b)
again = Parser("lifetimes are regions").next_word()
print("same object as a:", a is again)
