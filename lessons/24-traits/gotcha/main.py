class Robot:
    def __init__(self, id: int):
        self.id = id

    def speak(self) -> str:
        return f"beep {self.id}"


def introduce(s):
    print(s.speak())


introduce(Robot(7))
