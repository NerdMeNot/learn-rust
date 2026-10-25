enum Color {
  Red,
  Green,
  Blue,
}

const n: number = 42;
const c: Color = n; // compiles: any number is accepted
console.log("Color.Green:", Color.Green, "name of 1:", Color[1]);
console.log("a Color holding 42:", c, "name:", Color[c]);

// the usual alternative: a union of string literals
type Dir = "north" | "south";
function flip(d: Dir): Dir {
  return d === "north" ? "south" : "north";
}
console.log("flip:", flip("north"));
