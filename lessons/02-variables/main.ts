const mascot = "gopher"; // const: cannot be reassigned
let count = 1;         // let: can change
count += 1;

// TypeScript has no shadowing in the same scope, so use a new name
const scaled = count * 10;
console.log(`${mascot} ${scaled}`);
