const ages = [30, 40, 50];

// A copy: `age` holds its own number.
for (let age of ages) {
  age += 1;
  console.log("the copy is now", age);
}
console.log("ages after the copy loop:    ", ages);

// The elements themselves: assign into the array.
for (let i = 0; i < ages.length; i++) {
  ages[i] += 1;
}
console.log("ages after the in-place loop:", ages);
