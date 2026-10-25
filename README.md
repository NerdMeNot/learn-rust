# Learn Rust (even with all your past baggage)

A Rust course for people who already know another language, at
[learningrust.nerdmenot.in](https://learningrust.nerdmenot.in). Each lesson puts a Rust
program beside the same program in Go, Kotlin, Python, TypeScript, Java, C#, C or C++,
then shows a habit from that language that Rust refuses, the error it gives, and why.

## The one rule

**Nothing on the site is typed by hand.** Every program, output and compiler error comes
from compiling and running a real file in this repo, inside one pinned Docker image
(`tools/runner`) that holds all nine toolchains. Every claim a lesson makes ("this fails
to compile", "this crashes in C", "this has a data race in Go") is checked against
that run, and the build fails if one is false.

## What is in the repo

```
lessons/NN-<id>/    one lesson: lesson.yaml (the prose) + the programs, in every language
                    main.rs, main.go, Main.kt, main.py, main.ts, Main.java, Program.cs,
                    main.c, main.cpp; gotcha/ (the code Rust refuses, and fixed.rs);
                    faq-N/ (a question's programs)
guides/<id>/        the projects and terminal sessions behind the guide pages
                    (Start here, the standard library tour, Build with Rust)
tools/runner/       the Docker image every program runs in, and run.py
website/            the site: Astro + Starlight, deployed to Cloudflare Pages
```

## Working on it

Tool versions are pinned in `.prototools` (Node, bun, Rust) and `rust-toolchain.toml`.
With [proto](https://moonrepo.dev/proto): `proto install`. You also need Docker.

```sh
docker build -t learnrust-runner:dev tools/runner   # once, and after changing it

cd website
bun install
bun run extract                 # run what changed, check every claim
bun run extract --only lessons  # or --only guide:<id>
bun run dev                     # http://localhost:4321
bun run build && bun run check-links
```

`bun run extract` caches results by a hash of the program and the runner, in
`website/src/data/runs.json` and `website/src/data/guides/<id>.runs.json`. Those files are
committed, so the site builds without Docker. `--fresh` ignores the cache;
`--check` exits non-zero if any claim is false. The report of what it found is
`website/src/data/extract-report*.txt`.

### Editing a lesson

Change the prose in `lesson.yaml` or the code beside it, run `bun run extract --only
lessons`, and read the report. If you change what a program does, check that the prose
around it still describes the output: the extract checks statuses, not sentences.

### Editing a guide page

The prose is `website/src/content/docs/<track>/<id>.mdx`; the code and the commands it
shows are in `guides/<id>/`. See `guides/README.md`.

### Upgrading a toolchain

Bump the version in `tools/runner/Dockerfile` (and for Rust, also `rust-toolchain.toml`
and `.prototools`), rebuild the image, run `bun run extract`, and review the diff of
`website/src/data/`. Compiler messages change between releases; that diff is the review.

## Known limits

Decisions, so nobody has to rediscover them:

- **Not in the runner image:** Rust ThreadSanitizer, clippy inside the extract, Java
  `--enable-preview` and C++ TBB (`std::execution::par`). Pages that would need them say
  "not run" and link the docs; adding any of them changes the image hash and re-runs everything.
- **Not run, and labelled on the pages:** `go get`, `cargo add`, `rustup target add` and the
  debuggers (`dlv`, `gdb`, `lldb`); Maven, Gradle and JUnit; browsers (WebAssembly); Docker
  (the notes-api Dockerfile); Ctrl-C in the indexer. They need the network or tools the image lacks.
- **Needs network at extract time** (labelled in the step): the DataFusion guide's Python
  comparison, the interop guides' crates and npm packages. `guides/interop-node` names the image's
  `tsc` rather than pinning `typescript`.
- **Compile errors show the error only.** Warnings are dropped from failing runs (see
  `rustDiagnostics` in `website/scripts/extract.ts`); a program that compiles shows its warnings.
- **Lines of `tools/runner/run.py` tagged `hash-exempt`** add a capability (today: Kotlin programs that
  import `kotlinx.coroutines`) without touching how an existing program runs, so they stay out of the
  runner hash and do not re-run the cache. Anything else in `run.py` or the Dockerfile changes every key.

## CI

- **site.yml**: on every push. Re-checks every claim from the committed results (no
  Docker), checks the generated data is committed, builds, checks links, and deploys
  `main` to Cloudflare Pages. Needs the `CLOUDFLARE_API_TOKEN` and
  `CLOUDFLARE_ACCOUNT_ID` secrets.
- **freshness.yml**: weekly and on runner changes. Rebuilds the image, re-runs every
  program from scratch on an ARM runner, and fails if any output changed.

Outputs are made on linux/arm64. A few C details (whether `char` is signed) differ by
CPU, so local runs and CI both use ARM.
