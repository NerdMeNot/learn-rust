import gc
import weakref


class Node:
    def __init__(self, name):
        self.name = name
        self.other = None

    def __del__(self):
        print("freed", self.name)


def make_cycle():
    a = Node("a")
    b = Node("b")
    a.other = b
    b.other = a
    return weakref.ref(a)  # does not keep a alive


gc.disable()  # keep the automatic cycle collector out of the way

lone = Node("lone")
del lone  # the count falls to zero: freed on the spot, like an Rc
print("after del lone")

ref = make_cycle()  # each node is still referenced by the other
print("after make_cycle: a and b not freed, weakref alive:", ref() is not None)
found = gc.collect()  # the cycle collector finds them
print("gc.collect found garbage:", found > 0)
print("weakref alive now:", ref() is not None)
