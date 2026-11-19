# tally

A library that parses order files (`item,qty,unit_cents`) and totals them,
plus a small command-line program that uses it.

## Before you say you are done

All three must pass. Run them in this order:

    cargo fmt --check
    cargo clippy --all-targets -- -D warnings
    cargo test

## Toolchain

Rust 1.99, edition 2024 (`rust-version` in Cargo.toml). Do not use anything newer.
Dependency versions are pinned in Cargo.lock: anyhow 1.0.104, thiserror 2.0.21.
Read the docs for those versions, e.g. https://docs.rs/anyhow/1.0.104/anyhow/

## Where things live

- `src/lib.rs`: the library: `Order`, `ParseError`, `parse`, `total_cents`, unit tests.
- `src/main.rs`: the binary. Reads a file, prints the total. No logic of its own.
- `tests/`: integration tests, through the public API only.

## Rules

- Errors: `thiserror` in the library (`ParseError`), `anyhow` with `.context(...)` in the binary.
- No `unwrap` or `expect` outside tests and `main`. Return an error instead.
- No `unsafe`. If you think you need it, stop and ask. Any `unsafe` block needs
  a `// SAFETY:` comment saying why it is sound, and a test that exercises it.
- No new dependencies without asking first. Say which crate, which version, and why.
- Keep the public API stable: do not rename or change the signature of a `pub` item
  unless asked.
- Never weaken, skip or delete a test to make it pass. If a test looks wrong, say so.
- Do not silence a warning with `#[allow(...)]` unless asked. Fix the cause.
- Prefer borrowing to cloning. If you add a `.clone()`, say in one line why
  the copy is needed.

## How to report back

- If a command fails, paste its full output. Do not summarise compiler errors.
- If you changed who owns a value (a parameter from `T` to `&T`, a new `clone`,
  an `Rc` or `Arc`), say which and why.
- List the commands you ran and whether each passed.
