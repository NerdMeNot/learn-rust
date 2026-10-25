type Circle = { kind: "circle"; r: number };
type Rect = { kind: "rect"; w: number; h: number };
type Shape = Circle | Rect;

// each member is a type of its own, so a function can ask for just one
function radius(c: Circle): number {
  return c.r;
}

const shapes: Shape[] = [{ kind: "circle", r: 1 }, { kind: "rect", w: 2, h: 3 }];
console.log("radius:", radius({ kind: "circle", r: 1 }), "shapes:", shapes.length);
