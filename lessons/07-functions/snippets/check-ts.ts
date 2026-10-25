function must(ok: boolean, what: string): void {
  if (!ok) throw new Error(what);
}

function useIt(): number {
  return arrow(1);
}

// A function declaration is hoisted, so it can be called before its line.
must(hoisted() === 5, "hoisting");
function hoisted(): number {
  return 5;
}

// An arrow function stored in a const is not.
let threw = false;
try {
  useIt();
} catch (e) {
  threw = e instanceof ReferenceError;
}
const arrow = (n: number): number => n + 1;
must(threw, "const arrow used before its line throws");
must(useIt() === 2, "works after");

function totalWithTax(price: number): number {
  const rate = 20;
  function tax(p: number): number {
    return (p * rate) / 100; // a nested function may use `rate`
  }
  return price + tax(price);
}
must(totalWithTax(50) === 60, "nested function captures");

function area(w: number, h: number = 1): number {
  return w * h;
}
function total(...xs: number[]): number {
  return xs.reduce((a, b) => a + b, 0);
}
must(area(3) === 3 && area(3, 4) === 12, "default");
must(total(1, 2, 3) === 6, "rest");
function noReturn(): void {}
must((noReturn() as unknown) === undefined, "no return gives undefined");
function size(x: string): number;
function size(x: number[]): number;
function size(x: string | number[]): number {
  return x.length;
}
must(size("abc") === 3 && size([1, 2]) === 2, "overload signatures");

function connect({ retries = 3 }: { retries?: number }): number {
  return retries;
}
must(connect({}) === 3 && connect({ retries: 5 }) === 5, "options object");

function fill(xs: number[]): void {
  xs.push(1);
}
const list: number[] = [];
fill(list);
must(list.length === 1, "the caller sees the callee's change");
console.log("all checks passed");
