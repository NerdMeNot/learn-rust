const ages = new Map<string, number>();
ages.set("ana", 30);
ages.set("bo", 25);

// get returns number | undefined
const ana = ages.get("ana");
if (ana !== undefined) {
  console.log(`ana is ${ana}`);
} else {
  console.log("no ana");
}

// a default on purpose: 0 when the key is missing
const zed = ages.get("zed") ?? 0;
console.log(`zed is ${zed}`);

// count words: read the old count (or 0) and store the new one
const counts = new Map<string, number>();
for (const word of "a b a c a".split(" ")) {
  counts.set(word, (counts.get(word) ?? 0) + 1);
}
console.log(`a appears ${counts.get("a")} times`);
