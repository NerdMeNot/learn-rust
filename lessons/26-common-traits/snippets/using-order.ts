class Noisy {
  constructor(readonly name: string) {}

  [Symbol.dispose]() {
    console.log(`  dispose ${this.name}`);
  }
}

{
  using a = new Noisy("a");
  using b = new Noisy("b");
  console.log("  end of block");
}
