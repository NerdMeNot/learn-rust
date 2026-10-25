from dataclasses import dataclass, replace


@dataclass
class Team:
    name: str
    members: list


a = Team("red", ["ana"])
b = a  # two names for one object
b.name = "blue"
print("a.name after b = a, b.name = 'blue':", a.name)

c = replace(a, name="green")  # a new object, but a shallow copy
c.members.append("bob")
print("a.members:", a.members)
print("same list object:", a.members is c.members)
