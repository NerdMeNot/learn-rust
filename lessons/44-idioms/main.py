class Port:
    """The leading underscore is a convention: Python does not stop Port(0)."""

    def __init__(self, number):
        self._number = number

    @classmethod
    def parse(cls, text):
        try:
            n = int(text)
        except ValueError:
            raise ValueError("not a number") from None
        if n < 1 or n > 65535:
            raise ValueError(f"{n} is outside 1-65535")
        return cls(n)

    @property
    def number(self):
        return self._number


def listen(port):
    print("listening on", port.number)


for text in ["8080", "0", "http"]:
    try:
        listen(Port.parse(text))
    except ValueError as e:
        print(f'"{text}": {e}')
