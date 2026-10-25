type Point = { x: number; y?: number };

// rename, default value and rest, all in the pattern
function describe({ x: px, y = 0, ...rest }: Point & { label?: string }): string {
  return `px=${px} y=${y} rest=${JSON.stringify(rest)}`;
}

console.log(describe({ x: 1 }));
console.log(describe({ x: 1, y: 2, label: "p" }));

// an array destructures by position, and a missing item is undefined, not an error
const items: string[] = ["only"];
const [first, second] = items;
console.log(`first=${first} second=${second}`);

// destructuring gives the same values second names: the source stays whole
const pair = [1, 2];
const [m, n] = pair;
console.log(`pair is still ${pair} (m=${m}, n=${n})`);
