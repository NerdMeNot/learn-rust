function must(ok: boolean, what: string): void {
  if (!ok) throw new Error(what);
}

function add(a: number, b: number): number {
  return a + b;
}

function ident<T>(x: T): T {
  return x;
}

const count = 5;
let changing = 5;
changing += count;
must(changing === 10, "const let");

const pair: [number, string] = [1, "two"];
const [a, b] = pair;
const p = { x: 3, y: 4 };
const { x, y } = p;
must(a === 1 && b === "two" && x + y === 7, "destructuring");

const inc = (v: number) => v + 1;
must(inc(1) === 2 && add(1, 2) === 3, "arrow");

const unknownValue: unknown = "5";
const asserted = unknownValue as number; // a compile-time assertion: nothing is converted
must(typeof asserted === "string", "as does not convert");

must(ident<number>(5) === 5, "explicit type argument");
const m = new Map<string, number>();
must(m.size === 0, "Map");

const arr: number[] = [];
arr.push(1); // a TypeScript const stops reassignment, not changes inside
must(arr.length === 1 && parseInt("42", 10) === 42 && Number("42") === 42, "const contents, parse");
interface Point {
  x: number;
  y: number;
}
const pt: Point = { x: 1, y: 2 };
must(pt.x + pt.y === 3, "interface");

const raw = String.raw`C:\temp`;
must(raw.length === 7, "raw string");
must(10_000 === 10000 && 0xff === 255 && 0b101 === 5 && 0o17 === 15, "literals");
const nums: number[] = [1, 2];
const nums2: Array<number> = [1, 2];
must(nums.length === nums2.length, "arrays");
