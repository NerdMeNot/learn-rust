import sys

from cffi import FFI

ffi = FFI()
ffi.cdef("int64_t wordstats_add(int64_t a, int64_t b);")
lib = ffi.dlopen(sys.argv[1])

print("cffi:", lib.wordstats_add(2**40, 1))
try:
    lib.wordstats_add(2**63, 1)
except OverflowError as e:
    print("OverflowError:", e)
