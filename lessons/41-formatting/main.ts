class Money {
  constructor(public cents: number) {}

  // toString: how Money looks inside `${}`
  toString(): string {
    return `$${Math.trunc(this.cents / 100)}.${String(this.cents % 100).padStart(2, "0")}`;
  }
}

interface Item {
  name: string;
  qty: number;
}

// no format mini-language: padStart and padEnd do the work, centering is by hand
function center(s: string, width: number): string {
  const left = Math.floor((width - s.length) / 2);
  return s.padStart(s.length + left).padEnd(width);
}

const n = 42;
const s = String(n);
console.log(`[${s.padStart(6)}] [${s.padEnd(6)}] [${center(s, 6)}] [${s.padStart(6, "0")}]`);
console.log(`${(3.14159).toFixed(2)} ${(1234.5).toExponential()} 0x${n.toString(16)} 0b${n.toString(2)}`);

const price = new Money(1999);
console.log(`total: ${price}`);

const label = `${price} each`; // a template literal builds a string
console.log(`${label} (${label.length} chars)`);

const item: Item = { name: "pen", qty: 3 };
console.log(`${item.qty} x ${item.name}`);
console.log(item);                         // console.log shows the object's fields
console.log(JSON.stringify(item, null, 2)); // pretty, as JSON
