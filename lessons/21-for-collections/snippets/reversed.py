v = ["ana", "bo", "cy"]
try:
    reversed(enumerate(v))
except TypeError as e:
    print("reversed(enumerate(v)):", e)
print("reversed(list(enumerate(v))):", list(reversed(list(enumerate(v)))))
