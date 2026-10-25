type Shape =
  | { kind: "circle"; r: number }
  | { kind: "square"; side: number }
  | { kind: "triangle" };

function describe(s: Shape): void {
  switch (s.kind) {
    case "circle":
      console.log("circle", 3 * s.r * s.r);
      break;
    case "square":
      console.log("square", s.side * s.side);
      break;
  }
}

describe({ kind: "circle", r: 1 });
describe({ kind: "square", side: 2 });
describe({ kind: "triangle" });
console.log("done");
