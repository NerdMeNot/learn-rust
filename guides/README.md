# guides

The projects behind the guide pages (Start here, the standard library tour, Build with
Rust). The prose lives in `website/src/content/docs/<track>/<id>.mdx`; everything a page
shows as code or terminal output comes from here.

```
guides/<id>/
  guide.yaml      the sessions the page shows, and its gotcha
  rust/           the Rust project the page is about (a complete, buildable crate)
  go/             the same thing in Go, where the page compares
  gotcha/         files swapped into the project to show the gotcha
  snippet-*/      single files swapped in for one example
```

`bun run extract` (in `website/`) runs every session in the runner image and writes
`website/src/data/guides/<id>.json`. A session that fails when it should succeed, or
succeeds when it should fail (`expect: fail`), is reported as a problem.

## Rules for a page

- **Every file is complete.** A page may show an excerpt, but the file in the project
  builds. A `Cargo.toml` has its `[package]` section.
- **Every session runs.** If a page shows `cargo new hello`, the session runs it. If it
  starts a server and calls it with `curl`, the server step is `background: true` with
  a `ready:` pattern.
- **Dependencies are pinned.** The first run writes `Cargo.lock` / `go.sum` next to the
  project; commit them.
- **Commands are what a reader types.** Use `show:` only when the real command needs
  something the reader would not type (a flag that hides noise, a wait).
