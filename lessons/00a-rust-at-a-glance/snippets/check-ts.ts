function must(ok: boolean, what: string): void {
  if (!ok) throw new Error(what);
}

function add(a: number, b: number): number {
  return a + b;
}

function parse(s: string): number {
  const n = Number(s);
  if (Number.isNaN(n)) throw new Error("not a number");
  return n;
}

let x = 5;
x = 6;
const y = 5;
must(x === 6 && y === 5, "variables");
must(add(2, 3) === 5, "function");

let count = 0;
for (let i = 0; i < 3; i++) {
  count += i;
}
must(count === 3, "for");

let joined = "";
for (const w of ["a", "b"]) {
  joined += w;
}
must(joined === "ab", "for of");

const n: number = 1;
let s: string;
switch (n) {
  case 1:
    s = "one";
    break;
  default:
    s = "many";
}
must(s === "one", "switch");

const inc = (v: number) => v + 1;
must(inc(1) === 2, "arrow");

const name = "Ana";
const age = 30;
console.log(`${name} is ${age}`);

let threw = false;
try {
  parse("x");
} catch {
  threw = true;
}
must(threw, "throw leaves the function");
console.log("all typescript cells compile and hold");
