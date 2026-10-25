console.log('"a,b,".split(","):', "a,b,".split(","));
console.log('"".split(","):', "".split(","));
console.log('"k,v,w".split(",", 2):', "k,v,w".split(",", 2));
console.log('"".trim().split(/\\s+/):', "".trim().split(/\s+/));

for (const text of ["42", "+42", " 42", "1_000", "42abc", ""]) {
  console.log(`parseInt(${JSON.stringify(text)}):`, parseInt(text, 10), `Number(...):`, Number(text));
}

console.log('"a-a-a".replace("-", "+"):', "a-a-a".replace("-", "+"));
console.log('"a-a-a".replaceAll("-", "+"):', "a-a-a".replaceAll("-", "+"));
// \s counts U+FEFF as white space and not U+0085, the opposite of Rust's split_whitespace.
for (const [name, sep] of [["U+0085", "\u0085"], ["U+FEFF", "\ufeff"]]) {
  console.log(`"a" + ${name} + "b" split on /\\s+/:`, ("a" + sep + "b").split(/\s+/).length, "pieces");
}
console.log('"straße".toUpperCase():', "straße".toUpperCase());
console.log('"İ".toLowerCase().length:', "İ".toLowerCase().length);
