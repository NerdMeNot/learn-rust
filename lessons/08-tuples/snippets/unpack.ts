const t: [number, number, number, number] = [1, 2, 3, 4];
const [first, ...rest] = t;
console.log("first:", first, "rest:", rest);
