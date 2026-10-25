function must(ok: boolean, what: string): void {
  if (!ok) throw new Error(what);
}

let hits = 0;
outer: for (let a = 0; a < 3; a++) {
  for (let b = 0; b < 3; b++) {
    if (b === 1) continue outer;
    if (a === 2) break outer;
    hits++;
  }
}
must(hits === 2, "labels");

let sum = 0;
for (const x of [1, 2, 3]) sum += x;
must(sum === 6, "for...of");

let n = 0;
while (true) {
  n += 1;
  if (n === 3) break;
}
must(n === 3, "while (true)");

// for...of: changing the variable lasts one pass (like Rust)
const seenOf: number[] = [];
for (let x of [0, 1, 2, 3, 4]) {
  if (x === 1) x += 2;
  seenOf.push(x);
}
must(JSON.stringify(seenOf) === "[0,3,2,3,4]", "for...of copy");

// three-part for: the body's change carries into the next pass
const seenFor: number[] = [];
for (let i = 0; i < 5; i++) {
  if (i === 1) i += 2;
  seenFor.push(i);
}
must(JSON.stringify(seenFor) === "[0,3,4]", "for counter");

// continue in a for runs the step
let steps = 0;
for (let i = 0; i < 3; i++) {
  if (i === 1) continue;
  steps++;
}
must(steps === 2, "continue runs i++");

let d = 0;
do { d += 5; } while (d < 3);
must(d === 5, "do-while runs once");
console.log("ok");
