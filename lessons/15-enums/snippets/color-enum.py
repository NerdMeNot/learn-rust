from enum import Enum, auto


class Color(Enum):
    RED = auto()
    GREEN = auto()


print("RED:", Color.RED, "value:", Color.RED.value)
print("Color(2):", Color(2))
try:
    Color(7)
except ValueError as e:
    print("Color(7):", e)
