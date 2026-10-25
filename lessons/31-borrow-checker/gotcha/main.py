class Item:
    def __init__(self, name, count):
        self.name = name
        self.count = count


class Inventory:
    def __init__(self, items):
        self.items = items
        self.log = []

    def note(self, text):
        self.log.append(text)

    def check(self):
        for item in self.items:
            if item.count == 0:
                self.note("out of stock: " + item.name)


inv = Inventory([Item("bolt", 0), Item("nut", 12), Item("washer", 0)])
inv.check()
print("log: [" + ", ".join('"' + line + '"' for line in inv.log) + "]")
