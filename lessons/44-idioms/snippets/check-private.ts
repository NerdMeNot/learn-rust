class Port {
  private constructor(readonly number: number) {}
  static parse(n: number): Port {
    if (n < 1 || n > 65535) throw new Error("out of range");
    return new Port(n);
  }
}

// `private` is a compile-time check only: a cast gets past it, and plain JavaScript never saw it.
const bad = new (Port as any)(0);
console.log("built Port with number", bad.number);
