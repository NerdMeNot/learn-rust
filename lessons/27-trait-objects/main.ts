interface Shape {
  name(): string;
  area(): number;
}

class Square implements Shape {
  constructor(private side: number) {}
  name() { return "square"; }
  area() { return this.side * this.side; }
}

class Rect implements Shape {
  constructor(private w: number, private h: number) {}
  name() { return "rect"; }
  area() { return this.w * this.h; }
}

const shapes: Shape[] = [new Square(3), new Rect(4, 6)];

let total = 0;
for (const s of shapes) {
  console.log(`${s.name()}: ${s.area()}`);
  total += s.area();
}
console.log(`total: ${total}`);
