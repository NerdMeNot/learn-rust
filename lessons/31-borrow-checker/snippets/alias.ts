const a: number[] = [1, 2, 3];
const b = a;
b.push(4);
console.log("a:", a);
console.log("same object:", a === b);

const c = [...a];
c.push(5);
console.log("a:", a, "c:", c);
