function add<T extends number>(a: T, b: T): T {
  return a + b;
}

console.log(add(2, 3));
console.log(add(1.5, 2.25));
