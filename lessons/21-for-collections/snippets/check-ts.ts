// Backs the TypeScript cells of the tables in lesson.mdx. Never shown.
const v = ["ana", "bo", "cy"];
const entries = [...v.entries()].map(([i, x]) => `${i}${x}`).join(",");
if (entries !== "0ana,1bo,2cy") throw new Error(entries);
if ([...v].reverse().join(",") !== "cy,bo,ana") throw new Error("reverse");
if (v.filter((_, i) => i % 2 === 0).join(",") !== "ana,cy") throw new Error("every other");
let seen = "";
v.forEach((x) => {
  seen += x;
});
if (seen !== "anabocy") throw new Error(seen);
// removing the current item skips the next one; appending is seen by the loop
const removing = [1, 2, 3, 4];
const sawRemove: number[] = [];
for (const x of removing) {
  sawRemove.push(x);
  if (x === 2) removing.splice(removing.indexOf(2), 1);
}
if (JSON.stringify(removing) !== "[1,3,4]" || JSON.stringify(sawRemove) !== "[1,2,4]") throw new Error("remove");
const appending = [1, 2, 3, 4];
const sawAppend: number[] = [];
for (const x of appending) {
  sawAppend.push(x);
  if (x === 2) appending.push(99);
}
if (JSON.stringify(sawAppend) !== "[1,2,3,4,99]") throw new Error("append");
// a string loops over code points; a Map keeps insertion order
if ([..."h\u00e9llo\u{1F600}"].length !== 6) throw new Error("string");
if ([...new Map([["b", 1], ["a", 2], ["c", 3]]).keys()].join("") !== "bac") throw new Error("map");
console.log("ok");
