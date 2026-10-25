speed = 90
print("first:", type(speed).__name__)
speed = speed * 1.6  # same name, new type: Python rebinding allows it
print("second:", type(speed).__name__, speed)
