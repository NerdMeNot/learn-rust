class Counter:
    def __init__(self) -> None:  # called as Counter()
        self.count = 0

    def get(self) -> int:
        return self.count

    def bump(self) -> None:  # any method can change self
        self.count += 1


c = Counter()
c.bump()
c.bump()
print(c.get())
