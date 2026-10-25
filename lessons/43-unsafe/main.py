import ctypes


def get(a, i):
    return f"Some({a[i]})" if i < len(a) else "None"


prices = (ctypes.c_int * 3)(10, 20, 30)
p = ctypes.cast(prices, ctypes.POINTER(ctypes.c_int))
second = p[1]  # a ctypes pointer is not bounds-checked
print(f"second, read by offset: {second}")
print(f"get(1): {get(prices, 1)}")
print(f"get(9): {get(prices, 9)}")
