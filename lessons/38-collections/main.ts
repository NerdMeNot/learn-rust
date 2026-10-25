const text = "the cat saw the dog and the cat ran";

// Map: count words. It keeps insertion order, so sort for a ranking.
const counts = new Map<string, number>();
for (const w of text.split(" ")) {
  counts.set(w, (counts.get(w) ?? 0) + 1);
}
const pairs = [...counts].sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]));
console.log("top:", pairs.slice(0, 3));

// no sorted map in std: sort the entries yourself
const byName = new Map([["zoe", 31], ["ana", 30], ["mia", 28]]);
console.log("sorted:", new Map([...byName].sort((a, b) => a[0].localeCompare(b[0]))));

// Set: unique items, fast "is it in there?"
const seen = new Set(text.split(" "));
console.log(`${seen.size} unique words, has dog: ${seen.has("dog")}`);

// an array as a queue: shift() takes from the front, but it moves every item
const queue = ["a", "b"];
queue.push("c");
const first = queue.shift();
console.log(`served ${first}, waiting`, queue);

// no priority queue in std: sort a copy (with a number compare, not the default string one)
const heap = [5, 1, 8, 3].sort((a, b) => a - b);
console.log(`smallest first: ${heap.join(" ")}`);
