// A class has no == by value, no ordering and no usable Map key: write each by hand.
class Version {
  constructor(
    readonly major: number,
    readonly minor: number,
  ) {}

  // toString is the user-facing text.
  toString(): string {
    return `v${this.major}.${this.minor}`;
  }

  equals(other: Version): boolean {
    return this.major === other.major && this.minor === other.minor;
  }

  compareTo(other: Version): number {
    return this.major - other.major || this.minor - other.minor;
  }

  // A Map compares object keys by identity, so key on a string built from the fields.
  key(): string {
    return `${this.major}.${this.minor}`;
  }
}

const a = new Version(1, 10);
const b = new Version(1, 9);
console.log("display:", String(a));
console.log("a == b:", a.equals(b));
console.log("a > b:", a.compareTo(b) > 0);

const all = [a, b, new Version(0, 0)].sort((x, y) => x.compareTo(y));
console.log("sorted:", all.map(String).join(" "));

const seen = new Set<string>();
for (const v of [a, b, new Version(0, 0), a]) {
  seen.add(v.key());
}
console.log("distinct:", seen.size);
