// Backs the TypeScript cells of the table in lesson.mdx. Never shown.
class Stack<T> {
  items: T[] = [];
  push(item: T): void {
    this.items.push(item);
  }
}

interface HasLength {
  length: number;
}

const mixed: (number | string)[] = [1, "a"];

function longest<T extends HasLength>(a: T, b: T): T {
  return a.length >= b.length ? a : b;
}

const s = new Stack<number>();
s.push(1);
console.log(longest("ab", "abc"), longest([1], [1, 2]), s.items);
console.log(mixed.length);
console.log("ok");
