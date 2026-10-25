import copy


class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age

    def __repr__(self):
        return f"{self.name} {self.age}"


ages = [30, 40]
for age in ages:
    age += 1  # rebinds the name `age`; the list is not touched
print("ages after for age in ages: age += 1:", ages)

for i in range(len(ages)):
    ages[i] += 1  # assigns into the list
print("ages after ages[i] += 1:", ages)

people = [Person("Ada", 36), Person("Linus", 54)]
for p in people:
    p.age += 1  # p names the object that is in the list
print("people after p.age += 1:", people)


def add_item(items):
    items.append("x")  # changes the caller's list


def replace_items(items):
    items = ["y"]  # rebinds the parameter only


mine = []
add_item(mine)
replace_items(mine)
print("after add_item and replace_items:", mine)

# A slice of a list is a new list
nums = [1, 2, 3]
part = nums[0:2]
part[0] = 99
print("nums after part = nums[0:2]; part[0] = 99:", nums)

# Copies are shallow unless you ask
grid = [[1, 2], [3, 4]]
shallow = copy.copy(grid)
deep = copy.deepcopy(grid)
grid[0][0] = 99
print("shallow[0][0]:", shallow[0][0], " deep[0][0]:", deep[0][0])
rows = [[0]] * 2  # two names for one inner list
rows[0][0] = 7
print("rows after [[0]] * 2 and rows[0][0] = 7:", rows)

# Closures capture the name, not the value
fs = [lambda: i for i in range(3)]
print("lambdas in a loop:", [f() for f in fs])
