data = bytearray(b"hello world")
view = memoryview(data)[:5]  # a view of the first five bytes, not a copy
data[0] = ord("J")  # change the owner in place
print("view:", bytes(view))
try:
    data.append(33)  # resize the owner while the view exists
except BufferError as e:
    print("BufferError:", e)

xs = [1, 2, 3]
part = xs[:2]  # slicing a list makes a new list
part[0] = 100
print("xs:", xs)
