class Point {
  constructor(public x: number) {}
}

function report(kind: string, changed: boolean): void {
  console.log(kind.padEnd(8), changed ? "shares: the other name sees the change" : "copy:   the other name is untouched");
}

let n = 1;
let m = n;
m = 99;
report("number", n === 99);

let s = "hi";
let t = s;
t += "!"; // strings cannot change; this builds a new one
report("string", s === "hi!");

const p = new Point(1);
const q = p;
q.x = 99;
report("object", p.x === 99);

const arr = [1, 2];
const arr2 = arr;
arr2[0] = 99;
report("array", arr[0] === 99);

const map = new Map<string, number>([["a", 1]]);
const map2 = map;
map2.set("a", 99);
report("Map", map.get("a") === 99);

// Identity: === on objects asks "the same object?"
const r = new Point(99);
console.log("p === q:", p === q, " (same object)");
console.log("p === r:", p === r, " (same x, different object)");
const a1 = [1, 2];
const a2 = [1, 2];
console.log("a1 === a2:", a1 === a2, " (equal contents, different objects)");
console.log("Object.is(p, q):", Object.is(p, q));
