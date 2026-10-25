const a = [1, 2, 3];
const b = a;   // b and a now point at the SAME array. Nothing moves.
console.log(b);

const x = 5;
const y = x;   // numbers are copied
console.log(`${x} ${y}`);
