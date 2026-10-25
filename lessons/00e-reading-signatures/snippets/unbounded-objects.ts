function largest<T>(list: T[]): T {
  let best = list[0];
  for (const item of list) {
    if (item > best) {
      best = item;
    }
  }
  return best;
}

// compiles under `strict`: both sides are the same type, and that is all it asks
console.log("largest:", largest([{ a: 1 }, { a: 2 }]));
