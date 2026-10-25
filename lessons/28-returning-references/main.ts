// returns a new string
function makeGreeting(name: string): string {
  return `hello ${name}`;
}

// returns part of the input (also a new string: JS strings are immutable)
function firstWord(text: string): string {
  return text.split(" ")[0] ?? "";
}

const g = makeGreeting("rust");
const w = firstWord(g);
console.log(`${g} / ${w}`);
