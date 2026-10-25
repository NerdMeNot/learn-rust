/** A point on a grid. */
interface Point {
  x: number;
  y: number;
}

const p: Point = { x: 3, y: 4 }; // a line comment
const items = [1, 2, 3];
if (items.length !== 3) throw new Error("items should have 3 elements");
console.log(`point: Point { x: ${p.x}, y: ${p.y} }`);
console.log(`items: [${items.join(", ")}]`);
console.log(`row: [${String(42).padStart(5)}] [${String(42).padEnd(5)}] [${String(42).padStart(5, "0")}]`);
const pi = (3.14159).toFixed(2);
console.log(`pi: ${pi} {done}`);
