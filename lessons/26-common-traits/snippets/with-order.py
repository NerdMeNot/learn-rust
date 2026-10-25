class Noisy:
    def __init__(self, name):
        self.name = name

    def __enter__(self):
        return self

    def __exit__(self, *exc):
        print("  exit", self.name)


with Noisy("a") as a, Noisy("b") as b:
    print("  end of block")
