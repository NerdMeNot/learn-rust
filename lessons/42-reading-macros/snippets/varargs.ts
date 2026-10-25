function sum(...xs: number[]): number {
    return xs.reduce((a, b) => a + b, 0);
}

console.log("sum() =", sum());
console.log("sum(1, 2, 3, 4) =", sum(1, 2, 3, 4));
