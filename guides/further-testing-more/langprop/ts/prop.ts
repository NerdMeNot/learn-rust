import fc from "fast-check";

// JavaScript numbers are doubles, so `| 0` wraps a sum to a 32-bit int by hand.
function averageNaive(xs: number[]): number {
  let total = 0;
  for (const x of xs) total = (total + x) | 0;
  return Math.trunc(total / xs.length);
}

const property = fc.property(
  fc.array(fc.integer({ min: -(2 ** 31), max: 2 ** 31 - 1 }), { minLength: 1, maxLength: 7 }),
  (xs) => {
    const avg = averageNaive(xs);
    return avg >= Math.min(...xs) && avg <= Math.max(...xs);
  },
);

// A fixed seed makes the run repeatable.
fc.assert(property, { seed: 42 });
