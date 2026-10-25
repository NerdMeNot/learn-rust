class Counter:
    def __init__(self) -> None:
        self.count = 0

    def bump(self) -> None:
        self.count += 1


c = Counter()
c.bump()
print(c.count)
