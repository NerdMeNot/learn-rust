// One line per row of the page's table: row number, then the value.
function row(n: number, v: unknown) {
  console.log(String(n).padStart(2), v);
}

const n = 3;
const x: number | undefined = undefined;
const u: { name: string } | undefined = { name: "ana" };
const xs = [1, 2, 3, 4];
const ys = [5, 6];
const s = "héllo🙂";
const num = "12";
const m = new Map<string, number>();
const p = { x: 1, y: 2 };

row(1, `${n} items`);
row(2, x ?? 5);
row(3, u?.name);
row(4, xs.map((x) => x * 2));
row(5, xs.filter((x) => x % 2 === 0));
row(6, xs.reduce((a, b) => a + b, 0));
row(7, xs.find((x) => x > 2));
row(8, xs.includes(3));
row(9, xs.slice(1, 3));
row(10, [...xs, ...ys]);
row(11, [10, 9, 1].sort());
const a = [1, 2];
const b = [1, 2];
row(12, a === b);
row(13, [s.length, [...s].length]);
m.set("a", 1);
row(14, m.get("b") ?? 0);
const pairs: [number, number][] = [];
for (const [i, x] of xs.entries()) pairs.push([i, x]);
row(15, pairs);
row(16, [parseInt(num, 10), parseInt("x", 10)]);
const { x: px, y: py } = p;
row(17, [px, py]);
const double = (x: number) => x * 2;
row(18, double(21));
row(19, Array.from({ length: 3 }, (_, i) => i * i));
row(20, xs.every((x) => x > 0));
