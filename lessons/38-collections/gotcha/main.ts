const counts: Record<string, number> = {};
counts["cat"] = 0;
for (const w of "cat cat".split(" ")) {
  counts[w] += 1;
}
console.log(counts);
