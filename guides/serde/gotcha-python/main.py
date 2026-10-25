import json

cy = json.loads('{ "fullName": "Cy" }')  # no "age"
print(cy)
try:
    print(cy["age"])
except KeyError as e:
    print("KeyError:", e)
