import { test } from "node:test";
import assert from "node:assert/strict";
import { seconds } from "./parse.ts";

const valid: [string, number][] = [
  ["90s", 90],
  ["2m", 120],
  ["1h30m", 5400],
  ["1h1m1s", 3661],
];
for (const [text, want] of valid) {
  test(`parses ${text}`, () => assert.equal(seconds(text), want));
}

for (const text of ["", "15", "m", "5x"]) {
  test(`rejects ${JSON.stringify(text)}`, () => assert.throws(() => seconds(text)));
}
