# lessons

One folder per lesson: `lessons/NN-<id>/`, shown at `/learn/<id>/`.

Every program, output and compiler error the site shows comes from compiling and
running a file in here, never from typing. `cd website && bun run extract --only lessons`
runs them all in the runner image and checks every claim. It must report `0 problems`.

## The three layers

A lesson reads top to bottom in three layers. A reader can stop after any of them.

| Layer | What it gives the reader | Where it lives |
|---|---|---|
| **1. The quick comparison** | Rust next to the reader's language, what's different, and the gotcha. Two minutes. | `lesson.yaml` + `main.rs`, `main.go`, … + `gotcha/` + `faq-N/` |
| **2. The toolkit** | Everything they'll use day to day, each with a real program and its real output. | `lesson.mdx` + `snippets/` |
| **3. Going deeper** | How it works underneath, compared with their language, plus the types and cases they'll meet later. | `lesson.mdx` + `snippets/` |
| **Practice** | Two or three exercises, then where to read more. | `lesson.mdx` + `exercises/` |

`18-strings/` is the reference lesson. Read it before writing another.

## Files

```
lessons/NN-<id>/
  lesson.yaml              layer 1 prose (title, intro, coming_from, notes, faq, gotcha)
  main.rs, main.go, ...    layer 1 programs: Rust and the 8 other languages
  gotcha/                  the code Rust refuses, fixed.rs, and the other languages' versions
  lesson.mdx               layers 2 and 3, and practice
  snippets/<name>.rs       a program the page shows. Must compile and run.
  snippets/<name>.fail.rs  must FAIL to compile; the page shows the compiler error
  snippets/<name>.panic.rs must compile and then panic (or exit non-zero)
  snippets/<name>.<ext>    the same, in another language (go, kt, py, ts, java, cs, c, cpp), run in the runner
                           image and shown with its real output: how a claim about another language
                           gets proven. Java files hold `public class Main`, Kotlin a `fun main()`,
                           C# top-level statements. Name the variants:
                             name.fail.go     must not compile; name.crash.java must compile and fail
                             name.race.go     run with the race detector; name.asan.c / name.tsan.c
                                              run under AddressSanitizer / ThreadSanitizer
                           Markers combine in any order: name.crash.asan.cpp = under ASan, and it must fail.
                           (a Python ".fail" snippet is one that raises). Show it with
                           <Snippet file="stale-pointer.go" />.
  snippets/check-*.rs      run by the extract but never shown: asserts that back the "example -> result"
                           cells of a table, so a table cannot be typed by hand
  exercises/<name>.rs      a starter: compiles, but fails when run. Use todo!("a short hint"):
                           the hint is what the reader sees when they run it first
  exercises/<name>.solution.rs   passes. main() must be identical to the starter's.
```

## In lesson.mdx

No imports needed. These components are available:

- `<Snippet file="splitting.rs" />` shows a program from `snippets/` with its real output.
  `.fail.rs` shows the error, `.panic.rs` the panic. Optional: `title`, `mark={[3, 4]}` to
  highlight lines, `hideOutput`.
- `<Memory stack={[…]} heap={[…]} caption="…" title="…" />` draws variables on the
  stack, data on the heap, and arrows. See the comment at the top of
  `website/src/components/lesson/Memory.astro`. Back every diagram with a snippet that
  prints the facts it shows (as `snippets/memory.rs` does for strings).
- `<Exercise name="initials" title="Initials">what to do</Exercise>`.
- `<ForLang only="go">…</ForLang>` (or `only="c cpp"`) shows a passage only to readers coming from those
  languages. Use it for mappings ("strings.Cut is split_once") and for details only one
  background needs, so other readers are not slowed down. Optional `title`.
- `<Aside type="tip" title="…">…</Aside>` for a side note. (`:::tip` does not work here.)
- Mark each layer with `<p class="lr-layer">The toolkit</p>`, `Going deeper`, `Practice`.
  Then use `##` headings; they appear in "On this page".

## Writing rules

- **Real output only.** If the prose says what something prints, the snippet beside it
  prints exactly that. Re-read the output in `website/src/data/lessons/<id>.json` after
  every change.
- **Label printed values** so a reader can match output to code without counting lines:
  `println!("find('/'): {:?}", path.find('/'))`, not `println!("{:?}", path.find('/'))`.
- **One idea per snippet.** Group related methods (searching, splitting); don't make one
  program that does everything.
- **Plain words, short sentences.** Explain a term the first time it appears. Say what a
  thing is for, not only what it is.
- **Speak to the reader's background.** Compare with Go, Java/Kotlin/C#, Python,
  TypeScript and C/C++ where it helps. Be fair to their language.
- **Be honest.** No invented numbers ("90% of the time"). Say when Rust is awkward,
  and what people do about it.
- **Link, don't copy.** End with "Read more": the exact std docs pages, the Rust Book
  chapter, Rust by Example. Check every link opens.
- **Exercises teach one more thing.** Each should need a method from the toolkit, and the
  checks in `main` should include the case people get wrong (non-English text, an empty
  input).
- Every `.rs` file is `rustfmt --edition 2024` clean and has no compiler warnings
  (exercise starters may warn about unused parameters).

## Layout and colour (decided 2026-10-24)

- Code panels are **stacked** on every lesson, on every screen size: Rust first, then the reader's language, in a 60rem column that keeps the table of contents. There is **no side-by-side mode** (tried 2026-10-24 and removed on the owner's say-so). Never add a layout that splits panels into columns.
- Palette: Ferris orange on rust-lang.org slate (`#2a3439`) over warm paper (`#f4f1ea`) in light mode. Text and fills use `#c93c00` in light mode and `#ff6b35` in dark; Ferris's own `#f74c00` fails contrast as text (3.5:1). Tokens live in `website/src/styles/theme.css`. The header (`Header.astro`) and the three-way theme control (`ThemeToggle.astro`) follow the decoy, assay and catena sites; the "Learn Rust" wordmark is set in Fredoka 600 with "Rust" in the accent colour (headings keep Bricolage Grotesque). The mark is the claw (generated in Recraft, chosen by the user; the traced original is `archive/claw-logo-original.svg`): `website/public/icon.svg` is the favicon and home-page tile, `website/src/assets/logo-{light,dark}.svg` are the header logos. Keep the footer line "Not affiliated with the Rust Project or Foundation" (trademark policy).
