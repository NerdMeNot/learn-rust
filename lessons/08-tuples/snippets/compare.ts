// A TypeScript tuple is an array, and < on arrays compares them as text.
const a: [number, number] = [1, 10];
const b: [number, number] = [1, 9];
console.log("[1, 10] < [1, 9]:", a < b);
