# A dict literal is part of the language: no macro needed
ages = {"Ada": 36, "Linus": 28, "Grace": 45}

for name in sorted(ages):
    print(f"{name}: {ages[name]}")
print(f"entries: {len(ages)}")
