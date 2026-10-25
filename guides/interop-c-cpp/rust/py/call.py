import ctypes
import sys

lib = ctypes.CDLL(sys.argv[1])


class Point(ctypes.Structure):
    _fields_ = [("x", ctypes.c_double), ("y", ctypes.c_double)]


lib.point_distance.argtypes = [Point, Point]
lib.point_distance.restype = ctypes.c_double
print("distance:", lib.point_distance(Point(0, 0), Point(3, 4)))

lib.greeting.argtypes = [ctypes.c_char_p]
lib.greeting.restype = ctypes.c_void_p  # keep the address, to give it back to greeting_free
lib.greeting_free.argtypes = [ctypes.c_void_p]
text = lib.greeting(b"Python")
print(ctypes.string_at(text).decode())
lib.greeting_free(text)
