function must(ok: boolean, what: string): void {
  if (!ok) throw new Error(what);
}
const same = (a: unknown[], b: unknown[]) => JSON.stringify(a) === JSON.stringify(b);

const a = [3, 1, 2];
a.push(4);
must(a.pop() === 4, "pop");
must([].pop() === undefined, "pop empty");

const removed = a.splice(1, 1);
must(removed[0] === 1 && same(a, [3, 2]), "splice remove");
a.splice(1, 0, 5);
must(same(a, [3, 5, 2]), "splice insert");
must(a.at(-1) === 2 && a.length === 3, "at length");
must(a.includes(5) && a.indexOf(2) === 2 && a.indexOf(99) === -1, "includes indexOf");

const part = a.slice(1, 3);
part[0] = 99;
must(same(a, [3, 5, 2]), "slice copies");
must(same([1].concat([2, 3]), [1, 2, 3]) && same([...[1], ...[2]], [1, 2]), "concat");

must(same([10, 9, 1].sort(), [1, 10, 9]), "default sort is by string");
must(same([10, 9, 1].sort((x, y) => x - y), [1, 9, 10]), "numeric sort");
must(same(new Array(3).fill(0), [0, 0, 0]), "fill");
a.length = 1;
must(same(a, [3]), "set length truncates");
const base = [3, 1, 2];
const joined = base.concat([9]);
must(same(base, [3, 1, 2]) && same(joined, [3, 1, 2, 9]), "concat returns a new array");
must(base.at(10) === undefined && base.at(1) === 1, "at");
console.log("all checks passed");
