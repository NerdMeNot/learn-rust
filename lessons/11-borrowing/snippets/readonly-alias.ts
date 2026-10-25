const nums: number[] = [1, 2, 3];
const view: readonly number[] = nums; // view can't write...
nums.push(4); // ...but another name for the same array can
console.log("view:", view);
