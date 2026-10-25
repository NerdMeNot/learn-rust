v = [10, 20, 30]

try:
    v[10]
except IndexError as e:
    print(f"v[10]: IndexError: {e}")

print(f"v[-1]: {v[-1]}")
print(f"v[1:99]: {v[1:99]}")
print(f"v[10:20]: {v[10:20]}")
