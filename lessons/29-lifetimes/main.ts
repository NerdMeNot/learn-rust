// Nothing to annotate: the garbage collector keeps a string alive while it is reachable.
function longest(a: string, b: string): string {
  return a.length >= b.length ? a : b;
}

class Excerpt {
  constructor(readonly part: string) {}
}

const novel = "Call me Ishmael. Some years ago...";
const first = novel.split(".")[0];
const excerpt = new Excerpt(first);
const l = longest(excerpt.part, "short");
console.log("longest:", l);
console.log("excerpt length:", excerpt.part.length);
