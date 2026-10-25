def clamp(x):
    if x < 0:
        return 0
    return 100 if x > 100 else x


n = 7
label = "even" if n % 2 == 0 else "odd"
w = 3
h = 4
area = w * h
print(f"{n} is {label}")
print(f"area: {area}")
print("clamp:", clamp(-5), clamp(50), clamp(250))
