// readonly: total promises not to change the array
function total(xs: readonly number[]): number {
  let sum = 0;
  for (const x of xs) {
    sum += x;
  }
  return sum;
}

const nums = [1, 2, 3];
const t = total(nums); // total gets a reference to the same array
console.log(t, nums);
