// Object and Map literals are part of the language: no macro needed
const ages = new Map([
    ["Ada", 36],
    ["Linus", 28],
    ["Grace", 45],
]);

for (const name of [...ages.keys()].sort()) {
    console.log(`${name}: ${ages.get(name)}`);
}
console.log(`entries: ${ages.size}`);
