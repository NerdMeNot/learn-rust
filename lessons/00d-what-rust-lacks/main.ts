// What other languages write with ?:, i++, null and switch, written in TypeScript.
function size(n: number): string {
  switch (n) {
    case 1:
    case 2:
      return "small";
    case 3:
      return "medium";
    default:
      return "large";
  }
}

function firstEven(items: number[]): number | undefined {
  for (const x of items) {
    if (x % 2 === 0) return x;
  }
  return undefined;
}

let total = 0;
for (let i = 0; i < 4; i++) {
  total += i;
}
const label = total > 5 ? "big" : "small";
console.log(`total: ${total} (${label})`);
console.log(`size: ${size(2)} ${size(9)}`);
const even = firstEven([3, 5, 8]);
if (even !== undefined) {
  console.log(`first even: ${even}`);
} else {
  console.log("no even number");
}
console.log(`none: ${firstEven([1, 3]) ?? -1}`);
const wide = total * 1_000_000_000;
console.log(`wide: ${wide}`);
