// every object is already a shared reference
const log: string[] = [];
const other = log;

log.push("from log");
other.push("from other");

console.log("log:", log.join(", "));
console.log("same list:", log === other);
