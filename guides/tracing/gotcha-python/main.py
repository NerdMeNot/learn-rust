class Span:
    def __init__(self, name):
        self.name = name

    def __enter__(self):
        print("enter", self.name)
        return self

    def __exit__(self, *exc):
        print("exit", self.name)


with Span("request"):
    print("listed notes")  # __exit__ runs when the with block ends

_ = Span("forgotten").__enter__()  # nothing ever calls __exit__
print("after")
