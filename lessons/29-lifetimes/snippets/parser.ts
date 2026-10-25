class Parser {
  constructor(private rest: string) {}

  nextWord(): string | undefined {
    const trimmed = this.rest.trimStart();
    if (trimmed === "") return undefined;
    const space = trimmed.indexOf(" ");
    this.rest = space < 0 ? "" : trimmed.slice(space + 1);
    return space < 0 ? trimmed : trimmed.slice(0, space);
  }
}

const parser = new Parser("lifetimes are regions");
const a = parser.nextWord();
const b = parser.nextWord(); // a is still usable: no annotation needed
console.log("a:", a);
console.log("b:", b);
