class Item:
    def __init__(self, name, count):
        self.name = name
        self.count = count


class Inventory:
    def __init__(self, items):
        self.items = items
        self.log = []

    def check(self):
        for item in self.items:
            if item.count == 0:
                self.log.append("out of stock: " + item.name)


inv = Inventory([Item("bolt", 0), Item("nut", 12), Item("washer", 0)])
inv.check()
print("log: [" + ", ".join('"' + line + '"' for line in inv.log) + "]")
