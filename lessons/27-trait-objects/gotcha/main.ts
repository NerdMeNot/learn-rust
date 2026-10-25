interface Shape {
  area(): number;
}

class Square implements Shape {
  constructor(private side: number) {}
  area() { return this.side * this.side; }
}

class Rect implements Shape {
  constructor(private w: number, private h: number) {}
  area() { return this.w * this.h; }
}

const shapes: Shape[] = [new Square(3), new Rect(4, 6)];

console.log("total:", shapes.reduce((sum, s) => sum + s.area(), 0));
