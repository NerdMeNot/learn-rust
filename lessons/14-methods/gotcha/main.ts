class Counter {
  count = 0;
  bump(): void {
    this.count += 1;
  }
}

const c = new Counter();
c.bump();
console.log(c.count);
