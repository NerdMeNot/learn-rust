import sys
import weakref

a = []
print("getrefcount, one name:", sys.getrefcount(a))  # the call's own argument counts too
b = a
print("getrefcount, two names:", sys.getrefcount(a))

for value in ([1], 5):
    try:
        weakref.ref(value)
    except TypeError as e:
        print("weakref.ref fails:", e)
