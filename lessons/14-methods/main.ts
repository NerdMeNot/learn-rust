class Counter {
  private count = 0;

  get(): number { // only reads
    return this.count;
  }

  bump(): void { // changes the object
    this.count += 1;
  }
}

const c = new Counter();
c.bump();
c.bump();
console.log(c.get());
