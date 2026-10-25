class Counter {
  hits = 0;
}

const a = new Counter();
const b = a;

a.hits += 1;
b.hits += 1;
console.log("hits:", a.hits);
