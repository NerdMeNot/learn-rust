# in Python `_` is an ordinary variable: unpacking binds it, and so does any later assignment
_, second = 1, 2
print(f"after unpacking: _ = {_}")
_ = "later"
print(f"after assigning: _ = {_}")
