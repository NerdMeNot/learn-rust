// a tuple type: an array with a fixed length and a type per slot
function minMax(xs: readonly number[]): [number, number] {
  let lo = xs[0];
  let hi = xs[0];
  for (const x of xs) {
    if (x < lo) lo = x;
    if (x > hi) hi = x;
  }
  return [lo, hi];
}

const [lo, hi] = minMax([4, 9, 1, 7]);
console.log(`${lo} ${hi}`);

const [, onlyHi] = minMax([3, 5]); // leave a slot empty to skip it
console.log(onlyHi);
