class Version {
  constructor(
    readonly major: number,
    readonly minor: number,
  ) {}
}

const a = new Version(1, 2);
const b = new Version(1, 2);
console.log("a === b:", a === b);
console.log("same fields:", a.major === b.major && a.minor === b.minor);

const notes = new Map<Version, string>([[a, "stable"]]);
console.log("Map.get(a):", notes.get(a));
console.log("Map.get(b):", notes.get(b));

// Printing a class instance shows its fields, like {:?}; String(a) does not.
console.log(a);
console.log(String(a));
