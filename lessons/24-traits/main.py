from typing import Protocol


class Speaker(Protocol):
    def speak(self) -> str: ...

    def shout(self) -> str:  # default method
        return self.speak().upper()


class Dog(Speaker):
    def speak(self) -> str:
        return "woof"


class Robot(Speaker):
    def __init__(self, id: int) -> None:
        self.id = id

    def speak(self) -> str:
        return f"beep {self.id}"


def introduce(s: Speaker) -> None:
    print(f"{s.speak()} / {s.shout()}")


introduce(Dog())
introduce(Robot(7))
