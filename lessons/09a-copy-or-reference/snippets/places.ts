interface Person {
  name: string;
  age: number;
}

const ages = [30, 40];
for (let age of ages) {
  age += 1; // changes the loop's own variable
}
console.log("ages after for...of age += 1:", ages);

const people: Person[] = [
  { name: "Ada", age: 36 },
  { name: "Linus", age: 54 },
];
for (const p of people) {
  p.age += 1; // const stops p being reassigned, not the object being changed
}
console.log("people after p.age += 1:", JSON.stringify(people));

// slice() builds a new array
const nums = [1, 2, 3];
const part = nums.slice(0, 2);
part[0] = 99;
console.log("nums after part = nums.slice(0, 2); part[0] = 99:", nums);

// Copies are shallow: spread copies the top level only
const team = { name: "red", members: ["ana"] };
const shallow = { ...team };
const deep = structuredClone(team);
team.members[0] = "zed";
console.log("shallow.members:", shallow.members, " deep.members:", deep.members);

function addItem(items: string[]): void {
  items.push("x"); // changes the caller's array
}
function replaceItems(items: string[]): void {
  items = ["y"]; // rebinds the parameter only
}
const mine: string[] = [];
addItem(mine);
replaceItems(mine);
console.log("after addItem and replaceItems:", mine);

// Closures capture the variable. `var` has one per function, `let` one per iteration.
const withVar: (() => number)[] = [];
for (var i = 0; i < 3; i++) {
  withVar.push(() => i);
}
const withLet: (() => number)[] = [];
for (let j = 0; j < 3; j++) {
  withLet.push(() => j);
}
console.log("var:", withVar.map((f) => f()), " let:", withLet.map((f) => f()));
