interface Shape {
  area(): number;
}

class Square implements Shape {
  constructor(public side: number) {}
  area() { return this.side * this.side; }
}

class Rect {
  area() { return 24; }
}

// Rect never says `implements Shape`, but it has the right shape
const shapes: Shape[] = [new Square(3), new Rect()];
if (!(shapes[0] instanceof Square) || shapes[1] instanceof Square) throw new Error("instanceof");
if (shapes.reduce((sum, s) => sum + s.area(), 0) !== 33) throw new Error("sum");
const maybe: (Shape | null)[] = [new Square(1), null];
if (maybe[1] !== null) throw new Error("null");
// an interface can declare an attribute, not only methods
interface Named {
  name: string;
}
const named: Named = { name: "named" };
if (named.name !== "named") throw new Error("attribute");
console.log("ok");
