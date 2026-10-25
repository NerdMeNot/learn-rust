interface Shape {
  area(): number;
}

class Square implements Shape {
  area() { return 9; }
}

class Rock {}

const shapes: Shape[] = [new Square(), new Rock()];
console.log("list built:", shapes.length);
