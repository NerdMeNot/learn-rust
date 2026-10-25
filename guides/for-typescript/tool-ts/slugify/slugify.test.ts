import { test } from "node:test";
import assert from "node:assert/strict";
import { slugify } from "./slugify.ts";

test("joins words", () => {
  assert.equal(slugify("Rust for TypeScript developers"), "rust-for-typescript-developers");
});

test("drops punctuation", () => {
  assert.equal(slugify("  Hello,   World!  "), "hello-world");
});
