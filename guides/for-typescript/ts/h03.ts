const nums: number[] = [1, 2, 3];
const view: readonly number[] = nums; // read-only through this name only
nums.push(4);
console.log(view.length);
