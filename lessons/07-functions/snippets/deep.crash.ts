function depth(n: number): number {
  return n === 0 ? 0 : 1 + depth(n - 1);
}

console.log("depth(500):", depth(500));
console.log("depth(100_000_000):", depth(100_000_000));
