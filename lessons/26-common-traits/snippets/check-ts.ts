import assert from "node:assert";

assert.strictEqual(parseInt("4x"), 4);
assert.ok(Number.isNaN(Number("4x")));
assert.strictEqual(Number("42"), 42);

class Version {
  constructor(
    readonly major: number,
    readonly minor: number,
  ) {}
}
assert.notStrictEqual(new Version(1, 2), new Version(1, 2));
assert.deepStrictEqual(new Version(1, 2), new Version(1, 2));
console.log("all checks passed");
