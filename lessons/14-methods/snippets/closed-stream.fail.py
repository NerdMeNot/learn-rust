import io

f = io.StringIO("hello")
f.close()
print(f.read())  # ValueError: the object is closed, found out when the program runs
