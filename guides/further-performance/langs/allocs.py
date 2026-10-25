import sys

# Blocks CPython's allocator holds right now. Python has no per-call counter like Go's
# testing.AllocsPerRun, so this compares before and after.
before = sys.getallocatedblocks()
numbers = list(range(1000))
print("blocks gained by list(range(1000)):", sys.getallocatedblocks() - before)

before = sys.getallocatedblocks()
small = list(range(200))
print("blocks gained by list(range(200)): ", sys.getallocatedblocks() - before)
