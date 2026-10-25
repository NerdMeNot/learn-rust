const line = "  name=Ana, age=30, city=Lisbon  ";

const clean = line.trim(); // a new string: strings never change in place
console.log(`[${clean}]`);

// split on ',', trim each piece, split once on '='
for (const field of clean.split(",")) {
  const piece = field.trim();
  const at = piece.indexOf("=");
  if (at !== -1) {
    console.log(`${piece.slice(0, at).padStart(5)} -> ${piece.slice(at + 1)}`);
  }
}

// text to number: Number gives NaN on bad input instead of throwing
let age = Number("30");
if (Number.isNaN(age)) age = 0;
console.log(`next year: ${age + 1}`);

console.log(clean.toUpperCase().replaceAll(", ", " | "));

console.log(`starts with name: ${clean.startsWith("name")}`);
console.log(`position of age: ${clean.indexOf("age")}`); // -1 when missing

console.log(["build", "with", "rust"].join("-"));

// += on a string is fast enough in JS engines; no builder needed
let out = "";
for (let i = 1; i <= 3; i++) {
  out += `${i};`;
}
console.log(out);
