from typing import Protocol, runtime_checkable


@runtime_checkable
class Speaker(Protocol):
    def speak(self) -> str: ...


class Robot:  # never mentions Speaker
    def speak(self) -> str:
        return "beep"


class Rock:
    pass


print("Robot is a Speaker:", isinstance(Robot(), Speaker))
print("Rock is a Speaker:", isinstance(Rock(), Speaker))
