const v: number[] = [];
v.push(10);
v.push(20);

const w = [1, 2, 3];
w.push(4);           // const stops reassigning w, not changing it

const part = w.slice(1, 3); // slice makes a copy, not a view
console.log(v, w, part, `len=${w.length}`);
