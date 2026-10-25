const { x, y } = { x: 3, y: -2 };
console.log(`x = ${x}, y = ${y}`);

for (const [i, c] of [..."abc"].entries()) {
  console.log(`${i}: ${c}`);
}

const kv = "mode=fast".split("=");
if (kv.length === 2) {
  const [key, value] = kv;
  console.log(`${key} -> ${value}`);
}

// an empty slot skips an item
const [first, , , last] = [10, 20, 30, 40];
console.log(`first = ${first}, last = ${last}`);
