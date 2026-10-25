const nums = [1, 2, 3, 4, 5, 6];

// each step builds a new array right away (not lazy like Rust)
const evensSquared = nums
  .filter((n) => n % 2 === 0)
  .map((n) => n * n);

const total = nums.reduce((sum, n) => sum + n, 0);
console.log(evensSquared, total);
