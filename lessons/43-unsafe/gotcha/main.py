import ctypes

prices = (ctypes.c_int * 3)(10, 20, 30)
p = ctypes.cast(prices, ctypes.POINTER(ctypes.c_int))
second = p[1]
print(f"second: {second}")
