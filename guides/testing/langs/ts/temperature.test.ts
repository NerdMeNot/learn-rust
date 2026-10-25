import { test } from "node:test";
import assert from "node:assert/strict";

function toFahrenheit(celsius: number): number {
  return (celsius * 9) / 5 + 32;
}

test("boiling point", () => {
  assert.equal(toFahrenheit(100), 212);
});

test("freezing point", () => {
  assert.equal(toFahrenheit(0), 32);
});
