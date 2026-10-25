class Item {
  constructor(
    readonly name: string,
    readonly count: number,
  ) {}
}

class Inventory {
  log: string[] = [];

  constructor(readonly items: Item[]) {}

  check(): void {
    for (const item of this.items) {
      if (item.count === 0) {
        this.log.push("out of stock: " + item.name);
      }
    }
  }
}

const inv = new Inventory([new Item("bolt", 0), new Item("nut", 12), new Item("washer", 0)]);
inv.check();
console.log("log: [" + inv.log.map((line) => '"' + line + '"').join(", ") + "]");
