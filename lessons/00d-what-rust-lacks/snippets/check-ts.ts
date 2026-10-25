function area(width: number, height: number = 1): number {
  return width * height;
}

function parse(s: string): number {
  const n = Number.parseInt(s, 10);
  if (Number.isNaN(n)) {
    throw new Error("not a number: " + s);
  }
  return n + 1;
}

function check(ok: boolean, what: string): void {
  if (!ok) {
    throw new Error(what);
  }
}

type User = { name: string } | null;

function nameOf(u: User): string | undefined {
  return u?.name;
}

// ++ and --, also inside expressions, and --x decrements
const a = [10, 20, 30];
let i = 0;
const first = a[i++];
const second = a[++i];
check(first === 10 && second === 30 && i === 2, "inc");
let j = 5;
const pre = --j;
check(pre === 4 && j === 4, "--x decrements");

// the ternary operator
const label = i > 1 ? "big" : "small";
check(label === "big", "ternary");

// null and undefined
const missing: string | null = null;
let notSet: number | undefined;
check(missing === null && notSet === undefined, "null");

// optional chaining and ??
check(nameOf(null) === undefined && nameOf({ name: "Ada" }) === "Ada", "optional chaining");
const maybe: number | null = null;
check((maybe ?? 5) === 5, "coalesce");

// === and a == that coerces
const loose: any = 1;
check(loose == "1" && loose !== "1", "loose equality");

// .length on an array
check(a.length === 3, "length");

// truthiness
const empty: string = "";
const zero: number = 0;
const list: number[] = [];
const zeroText: string = "0";
check(!empty && !zero && !missing && !notSet, "falsy");
check(!!list && !!zeroText, "truthy");

// optional parameters, an options object, and a closure
function scale(x: number, factor?: number): number {
  return x * (factor ?? 1);
}
check(scale(2) === 2 && scale(2, 3) === 6, "optional parameter");
check(area(3) === 3 && area(3, 4) === 12, "default");
function box({ width, height = 1 }: { width: number; height?: number }): number {
  return width * height;
}
check(box({ width: 3, height: 4 }) === 12 && box({ width: 3 }) === 3, "options");
const inc = (x: number) => x + 1;
check(inc(1) === 2, "arrow");

// one number type
check(1 + 2.5 === 3.5 && 1 === 1.0, "number");

// exceptions
let caught = false;
try {
  parse("x");
} catch (e) {
  caught = true;
}
check(caught && parse("41") === 42, "catch");

// indexing a string gives a one-character string
check("héllo"[0] === "h" && "héllo"[1] === "é", "index");

// switch falls through without break
let out = "";
const pick: number = 1;
switch (pick) {
  case 1:
    out += "a";
  case 2:
    out += "b";
    break;
  default:
    out += "c";
}
check(out === "ab", "fallthrough");

// a missing semicolon is repaired at the end of the line
let asi = 1
asi += 1
check(asi === 2, "asi");

// a module-level variable that any function may change
let visits = 0;
function visit(): void {
  visits++;
}
visit();
check(visits === 1, "global");

console.log("typescript: all checks passed");
