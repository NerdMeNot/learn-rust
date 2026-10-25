function largest<T>(list: T[]): T {
  let best = list[0];
  for (const item of list) {
    if (item > best) {
      best = item;
    }
  }
  return best;
}

const numbers = [3, 9, 4];
const words = ["pear", "fig", "apple"];
console.log("largest number:", largest(numbers));
console.log("largest word:", largest(words));
