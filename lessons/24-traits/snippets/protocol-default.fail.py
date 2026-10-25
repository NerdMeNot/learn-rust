from typing import Protocol


class Speaker(Protocol):
    def speak(self) -> str: ...

    def shout(self) -> str:  # a default body
        return self.speak().upper()


class Dog(Speaker):  # names the Protocol as a base: gets the default
    def speak(self) -> str:
        return "woof"


class Robot:  # fits by shape only
    def speak(self) -> str:
        return "beep"


print("Dog:", Dog().shout())
print("Robot:", Robot().shout())
