const n = 7;
if (n % 2 === 0) {
  console.log(`${n} is even`);
} else {
  console.log(`${n} is odd`);
}

// `if` is a statement in TypeScript; the ternary gives back a value
const size = n > 5 ? "big" : "small";
console.log(size);
