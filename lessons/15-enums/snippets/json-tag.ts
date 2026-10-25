type Shape =
  | { kind: "circle"; r: number }
  | { kind: "rect"; w: number; h: number };

function area(s: Shape): number {
  switch (s.kind) {
    case "circle":
      return 3.14 * s.r * s.r;
    case "rect":
      return s.w * s.h;
  }
}

// the tag says "rect", but the fields were never checked
const s = JSON.parse('{"kind": "rect", "w": 2}') as Shape;
console.log("area:", area(s));
