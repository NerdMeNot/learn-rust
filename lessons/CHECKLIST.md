# Lesson checklist

Run through this before a lesson goes to review, and again before it is called done.
It comes from what the four pilot pages' reviews found. Most items cost one sentence to fix.

## Order and scope
- [ ] **Taught before used.** Every term a reader meets (heap, closure, borrow, trait, `Copy`…) is
      either explained on this page, explained on an earlier lesson in the course order, or linked.
      If it can't be, say so in one line and link forward.
- [ ] **One idea explained once.** If a topic needs more than a paragraph (for example `&&` in
      iterators), it lives in one section and the rest link to it.
- [ ] **Length.** The toolkit shows the everyday core; reference material goes in tables.
      Target about 1,500 words for Basics, 2,500–3,500 for the rest. The extract warns above 4,000.

## The reader's language
- [ ] **A `<ForLang>` mapping table for each background that matters** (Go, Java/Kotlin/C#,
      Python/TypeScript, C/C++): "what you write today → in Rust". These were the most valued
      part of the pilot.
- [ ] **No explanation written for one language is shown to everyone.** If it names Go's `**int`,
      Java's `Optional` or C++'s `std::move`, it belongs in `<ForLang>` or in that language's
      `coming_from` line.
- [ ] **Each language is described fairly.** Check the details a native would check: Go's GC
      pauses are short, Kotlin already has null safety, TypeScript under `strict` checks a lot,
      Python's `b = a` sharing is documented, not a bug. When unsure, run it.
- [ ] **Gotcha verdict matches the reasoning.** "Fine elsewhere. Rust can't prove it's safe." only
      when it really is a safety proof. A convention or ambiguity is `rust-rule`.

## Truth
- [ ] **Every claim about output, a compiler message, size or behaviour is backed by a program
      on the page**, or by a source link you opened. Not "I remember that…".
- [ ] **The platform is named.** Outputs come from Linux on ARM64 (Rust 1.99.1, GCC 14, etc.).
      Anything that depends on the machine (`char` signedness, thread ids, addresses) says so.
- [ ] **No hype and no invented numbers** ("90% of the time", "10x faster").
- [ ] **Words that overclaim are checked:** "always", "never", "only", "every", "free",
      "zero-cost", "safe". Qualify or prove.
- [ ] **Re-read the generated output** in `website/src/data/lessons/<id>.json` after every
      change, and compare it with the sentence beside it.

## Craft
- [ ] **Outputs are labelled**: `println!("find('/'): {:?}", …)`, so a reader can match output to code.
- [ ] **Each diagram is backed by a program** that prints the same facts (`size_of`, `as_ptr`,
      counts). Cells are written in full; the `note`, `links` and `gone` options are used.
- [ ] **Each snippet shows one idea.** Failing code is a `.fail.rs` (the real error is shown),
      a panic is a `.panic.rs`.
- [ ] **Exercises** (2–3): starters compile and stop with `todo!("a hint")`; `main` is identical in
      starter and solution; the checks include the case people get wrong (empty input,
      non-English text); a plain loop is a fine answer if iterators haven't been taught yet.
- [ ] **"Read more"** links to the exact std doc pages, the Rust Book chapter and Rust by Example.
      Every link returns 200.
- [ ] **Plain words, short sentences.** Explain a term the first time. Say what a thing is
      *for*, not only what it is.
- [ ] `rustfmt --edition 2024` clean, no compiler warnings (exercise starters may warn about
      unused parameters).

## Review loop
1. Write the lesson (one agent per lesson folder).
2. `cd website && bun run extract --only lessons`: 0 problems.
3. Fact-check agent: opens every linked source, runs every disputed claim.
4. Two reader reviews, chosen for the topic (Go and Java for traits; C++ and Python for lifetimes).
5. Revise.
6. Read the built page and screenshot it as 2–3 languages.

**Only the person integrating runs `astro build`.** Parallel builds share `.astro/.prerender`
and corrupt each other. Writers run the extract and read the JSON.
