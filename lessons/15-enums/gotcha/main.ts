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
    default: {
      const unreachable: never = s; // fails to compile if a case is missing
      return unreachable;
    }
  }
}

console.log(area({ kind: "triangle", b: 2.0, h: 3.0 }));
