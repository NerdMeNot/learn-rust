type Shape =
  | { kind: "circle"; r: number }
  | { kind: "rect"; w: number; h: number };

function area(s: Shape): number {
  switch (s.kind) {
    case "circle":
      return 3 * s.r * s.r;
    case "rect":
      return s.w * s.h;
  }
}

console.log(area({ kind: "circle", r: 2 }), area({ kind: "rect", w: 2, h: 5 }));
