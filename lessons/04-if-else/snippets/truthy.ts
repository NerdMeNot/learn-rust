function orDefault(x: number, d: number): number {
  return x || d;
}
function nullishDefault(x: number | null, d: number): number {
  return x ?? d;
}
function describe(items: number[]): string {
  return items ? "an empty array is true" : "false";
}

console.log("0 || 5:", orDefault(0, 5));
console.log("0 ?? 5:", nullishDefault(0, 5));
console.log("null ?? 5:", nullishDefault(null, 5));
const p = [1, 2];
const q = [1, 2];
console.log("p === q for two equal arrays:", p === q);
console.log("[]:", describe([]));
