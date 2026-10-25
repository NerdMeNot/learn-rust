const keys = ["b", "2", "a", "1"];

const obj: Record<string, number> = {};
const map = new Map<string, number>();
for (const k of keys) {
  obj[k] = 0;
  map.set(k, 0);
}
console.log("object keys:", Object.keys(obj));
console.log("Map keys:", [...map.keys()]);

const id = { n: 1 };
const byObject = new Map([[id, "found"]]);
console.log("same object:", byObject.get(id));
console.log("equal-looking object:", byObject.get({ n: 1 }));
