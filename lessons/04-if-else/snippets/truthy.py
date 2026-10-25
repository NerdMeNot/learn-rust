print("0 or 5:", 0 or 5)
print("[] or 'empty':", [] or "empty")
print("'a' and 'b':", "a" and "b")
print("1 < 3 < 5:", 1 < 3 < 5)
d = {"a": 0}
print('d.get("a") or 9:', d.get("a") or 9)
print('d.get("a", 9):', d.get("a", 9))
if []:
    print("a list is true")
else:
    print("an empty list is false")
