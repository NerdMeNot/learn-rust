import { readFileSync as readSync } from "node:fs";

export function largest<T extends number | string>(list: T[]): T {
  let best = list[0];
  for (const item of list) {
    if (item > best) best = item;
  }
  return best;
}

function apply(f: (x: number) => number, x: number): number {
  return f(x);
}

function find(m: Map<string, string>, k: string): string | undefined {
  return m.get(k);
}

function read(path: string): string {
  return readSync(path, "utf8");
}

function sumAll(xs: readonly number[]): number {
  let s = 0;
  for (const x of xs) s += x;
  return s;
}

function pushOne(xs: number[]): void {
  xs.push(1);
}

function must(ok: boolean, what: string): void {
  if (!ok) throw new Error(what);
}

must(largest([3, 9, 4]) === 9, "largest");
must(apply((x) => x + 1, 1) === 2, "apply");
must(find(new Map(), "a") === undefined, "find");
try {
  read("/nope");
  must(false, "read");
} catch (e) {
  // nothing in the signature said this could throw
}
const v = [2];
pushOne(v);
must(sumAll(v) === 3, "sum");
console.log("typescript rows ok");
