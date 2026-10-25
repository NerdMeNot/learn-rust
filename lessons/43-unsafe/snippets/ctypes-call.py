import ctypes

libc = ctypes.CDLL(None)  # the C library already loaded into this process
libc.strlen.argtypes = [ctypes.c_char_p]  # nothing checks these declarations against C
libc.strlen.restype = ctypes.c_size_t
print("strlen('héllo') =", libc.strlen("héllo".encode()), "bytes")
print("len('héllo') =", len("héllo"), "characters")
