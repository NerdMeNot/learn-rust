const names = ["ana", "bo"];

for (const n of names) {
  console.log(`hi ${n}`);
}

const nums = [1, 2, 3];
for (let i = 0; i < nums.length; i++) { // n holds a copy of each number, so assign through the index
  nums[i] *= 10;
}

for (const [i, n] of nums.entries()) { // with the position
  console.log(`${i}: ${n}`);
}
console.log(`${names.length} names,`, nums);
