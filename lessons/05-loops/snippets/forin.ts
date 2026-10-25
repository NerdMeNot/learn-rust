const xs = ["a", "b"];
for (const k in xs) {
  console.log(`for...in key: ${JSON.stringify(k)}, type ${typeof k}`);
}
for (const x of xs) {
  console.log(`for...of value: ${x}`);
}
