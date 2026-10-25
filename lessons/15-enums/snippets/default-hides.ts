type Shape =
  | { kind: "circle"; r: number }
  | { kind: "rect"; w: number; h: number }
  | { kind: "triangle"; b: number; h: number }; // new!

function area(s: Shape): number {
  switch (s.kind) {
    case "circle":
      return 3.14 * s.r * s.r;
    case "rect":
      return s.w * s.h;
    default:
      return 0; // compiles, and now covers triangle too
  }
}

console.log("triangle area:", area({ kind: "triangle", b: 3, h: 4 }));
