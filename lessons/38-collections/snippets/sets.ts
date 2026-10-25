const a = new Set(["ana", "bo", "cy", "dee"]);
const b = new Set(["cy", "dee", "eli"]);
// Set methods for union and friends arrived in newer engines: this ran on Node 24
console.log("union:", [...a.union(b)]);
console.log("intersection:", [...a.intersection(b)]);
console.log("difference:", [...a.difference(b)]);

console.log("new Set keeps first-seen order:", [...new Set([3, 1, 3, 2, 1])]);
console.log("a Set compares object keys by identity:", new Set([[1, 2], [1, 2]]).size);

// A Map visits keys in the order they were inserted
const m = new Map<string, number>();
m.set("zoe", 1);
m.set("ana", 2);
m.set("mia", 3);
console.log("Map order:", [...m.keys()]);
console.log("sorted keys:", [...m.keys()].sort());

// No heap, no deque, no sorted map in the language: an array is the queue
const queue = ["a", "b", "c"];
queue.push("d");
console.log("shift:", queue.shift(), "left:", queue);
