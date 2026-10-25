const names = ["ana", "bo"];

for (const i in names) {
  console.log(`for...in: ${JSON.stringify(i)} is a ${typeof i}`);
}
for (const [i, n] of names.entries()) {
  console.log(`entries(): ${JSON.stringify(i)} is a ${typeof i}, item ${n}`);
}

const nums = [1, 2, 3];
for (let n of nums) {
  n *= 10; // changes the local n only
}
console.log("after n *= 10:", nums);
