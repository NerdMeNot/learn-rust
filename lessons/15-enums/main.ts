// a discriminated union: `kind` says which variant it is
type Shape =
  | { kind: "circle"; r: number }
  | { kind: "rect"; w: number; h: number };

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

const shapes: Shape[] = [{ kind: "circle", r: 1.0 }, { kind: "rect", w: 2.0, h: 3.0 }];
for (const s of shapes) {
  console.log(area(s));
}
