from abc import ABC, abstractmethod


class Speaker(ABC):
    @abstractmethod
    def speak(self) -> str: ...


class Rock(Speaker):  # forgot to write speak
    pass


print("before the instance")
Rock()
