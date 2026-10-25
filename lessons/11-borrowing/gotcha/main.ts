function total(xs: number[]): number {
  let sum = 0;
  for (const x of xs) {
    sum += x;
  }
  return sum;
}

const nums = [1, 2, 3];
const t = total(nums);
console.log(t, nums);
