// Greet someone, then add up 1 to 4.
function greet(name: string): string {
  return `Hello, ${name}!`;
}

function add(a: number, b: number): number {
  return a + b;
}

console.log(greet("Ferris"));
let total = 0;
for (let n = 1; n <= 4; n++) {
  total = add(total, n);
}
console.log(`total: ${total}`);
