import ctypes
import sys

lib = ctypes.CDLL(sys.argv[1])

# ctypes has not been told the signature, so it guesses C int (32 bits).
print("no signature:", lib.wordstats_add(2**40, 1))

lib.wordstats_add.argtypes = [ctypes.c_int64, ctypes.c_int64]
lib.wordstats_add.restype = ctypes.c_int64
print("with signature:", lib.wordstats_add(2**40, 1))
