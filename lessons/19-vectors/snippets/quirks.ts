const v: number[] = [10, 20, 30];

console.log("v[10]:", v[10]);
console.log("v.at(-1):", v.at(-1));

const nums = [10, 9, 1];
nums.sort();
console.log("[10, 9, 1].sort():", nums);

const holes = [10, 20, 30];
holes[6] = 70;
console.log("holes:", holes, "length", holes.length);
console.log("3 in holes:", 3 in holes);
