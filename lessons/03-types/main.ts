const a: number = 7;      // every ordinary number is a 64-bit float
const b: bigint = 10n;    // bigint: whole numbers of any size
const price: number = 2.5;
const ok: boolean = true;

const total = BigInt(a) + b; // number and bigint don't mix: convert first
const cost = a * price;
console.log(`${total} ${cost} ${ok}`);
