// 1, 2, 3 (the end is not included)
for (let i = 1; i < 4; i++) {
  console.log(`for ${i}`);
}

let n = 3;
while (n > 0) {
  n -= 1;
}

// break can't hand back a value, so set a variable before breaking
let tries = 0;
let found: number;
while (true) {
  tries += 1;
  if (tries === 3) {
    found = tries * 10;
    break;
  }
}
console.log(`n=${n} found=${found}`);
