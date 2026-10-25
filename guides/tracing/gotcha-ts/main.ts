class Span implements Disposable {
  name: string;

  constructor(name: string) {
    this.name = name;
    console.log("enter", name);
  }

  [Symbol.dispose]() {
    console.log("exit", this.name);
  }
}

{
  using span = new Span("request");
  console.log("listed notes"); // [Symbol.dispose] runs when the block ends
}
new Span("forgotten"); // nothing ever calls [Symbol.dispose]
console.log("after");
