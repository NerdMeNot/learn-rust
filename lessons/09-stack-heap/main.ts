interface Point {
  x: number;
  y: number;
}

const a: Point = { x: 1, y: 2 };
const b = a; // b is another name for the same object
b.x = 99;
console.log(`a = (${a.x}, ${a.y})`);
console.log(`b = (${b.x}, ${b.y})`);

// To get a separate point, you build one.
const c: Point = { ...a };
c.x = 7;
console.log(`a after c.x = 7: (${a.x}, ${a.y})`);
