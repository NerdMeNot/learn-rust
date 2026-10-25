const withVar: (() => number)[] = [];
for (var i = 0; i < 3; i++) withVar.push(() => i);
console.log("var:", withVar.map((f) => f()));

const withLet: (() => number)[] = [];
for (let j = 0; j < 3; j++) withLet.push(() => j);
console.log("let:", withLet.map((f) => f()));

let x = 1;
const show = () => console.log("x:", x);
x = 2;
show();
