const owned = "héllo";          // strings can't be changed in place
const view = owned.slice(0, 1); // slice makes a new string

let s = "go";
s += " + rust";                 // makes a new string and reassigns s

console.log(`${owned} ${view} ${s}`);
const bytes = new TextEncoder().encode(owned).length; // UTF-8 bytes
const chars = [...owned].length;                      // real characters
console.log(`bytes=${bytes} chars=${chars}`);

let line = "";
for (const c of owned) { // for...of walks characters, not UTF-16 units
  line += `[${c}]`;
}
console.log(line);
