// T can be a number or a string: both can be compared with < and >
function largest<T extends number | string>(items: readonly T[]): T {
  let best = items[0];
  for (const item of items) {
    if (item > best) {
      best = item;
    }
  }
  return best;
}

console.log(largest([3, 9, 2]));
console.log(largest(["kiwi", "apple", "pear"]));
