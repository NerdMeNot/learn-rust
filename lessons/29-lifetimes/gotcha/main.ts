function longest(a: string, b: string): string {
  return a.length >= b.length ? a : b;
}

// join builds a new string at run time, not a constant.
function join(...parts: string[]): string {
  return parts.join(" ");
}

const s1 = join("a", "long", "first", "string");
let result: string;
let other: string;
{
  const s2 = join("short", "string");
  result = longest(s1, s2);
  const s3 = join("an", "even", "longer", "second", "string");
  other = longest(s1, s3);
}
console.log("longest:", result);
console.log("with a longer s3:", other);
