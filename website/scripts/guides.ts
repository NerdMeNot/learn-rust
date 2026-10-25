/**
 * The guide half of the extract: guides/<id>/guide.yaml describes real projects and
 * the shell sessions a page shows. This turns them into runner jobs, then checks and
 * writes src/data/guides/<id>.json.
 *
 * guide.yaml:
 *
 *   name: notes-api                 # the project's name, used in paths Cargo prints
 *   sessions:
 *     run:
 *       dir: rust                   # a folder in guides/<id>/, copied as the project
 *       overlay:                    # optional: swap files in before running
 *         src/main.rs: gotcha/main.rs
 *       local_target: true          # build into ./target/ (default: a shared cache folder)
 *       steps:
 *         - cmd: cargo run --bin notes-api
 *           background: true        # keep running; wait for `ready`
 *           ready: listening on
 *         - cmd: curl -s localhost:3000/notes
 *           expect: fail            # ok (default) | fail | any
 *           raw: true               # keep Cargo's "Compiling ..." lines
 *           show: "$ cargo run"     # what the page prints as the command
 *   gotcha:                         # optional; same prose fields as a lesson's
 *     kind: compile                 # compile | runtime | cargo
 *     code: { file: gotcha/main.rs, name: src/main.rs }
 *     session: gotcha               # its last step is the error (or, for runtime, the output)
 *     fixed: { file: rust/src/main.rs, name: src/main.rs, session: run }
 *     langs: { go: { status: runs, says: ..., file: go/main.go, session: go } }
 */
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync, mkdirSync } from 'node:fs'
import { join, relative } from 'node:path'

const SKIP = new Set(['target', 'node_modules', '.DS_Store'])

// Sorted: the order a directory lists its files in depends on the filesystem, and the order
// goes into the cache key, so an unsorted walk gives macOS and Linux (CI) different keys.
function walk(dir: string, base = dir, out: Record<string, string> = {}) {
  for (const name of readdirSync(dir).sort()) {
    if (SKIP.has(name)) continue
    const p = join(dir, name)
    if (statSync(p).isDirectory()) walk(p, base, out)
    else out[relative(base, p)] = readFileSync(p, 'utf8')
  }
  return out
}

export type SessionJob = { key: string; kind: 'session'; project: string; files: Record<string, string>; steps: any[]; collect: string[] }

export function loadGuides(root: string, hash: (s: string) => string, only: (id: string) => boolean = () => true) {
  if (!existsSync(root)) return []
  // Filter before reading anything, so a page someone is half-way through editing
  // cannot break a run for a different page.
  return readdirSync(root).filter(d => only(d) && existsSync(join(root, d, 'guide.yaml'))).sort().map(id => {
    const base = join(root, id)
    const doc: any = Bun.YAML.parse(readFileSync(join(base, 'guide.yaml'), 'utf8'))
    const dirs = readdirSync(base).sort().filter(d => statSync(join(base, d)).isDirectory() && !SKIP.has(d))
    const files = Object.fromEntries(dirs.map(d => [d, walk(join(base, d))]))
    const jobs: Record<string, SessionJob & { dir: string }> = {}
    for (const [name, s] of Object.entries<any>(doc.sessions ?? {})) {
      const projectFiles = { ...files[s.dir] }
      for (const [to, from] of Object.entries<string>(s.overlay ?? {})) {
        const text = readFileSync(join(base, from), 'utf8')
        projectFiles[to] = text
      }
      const steps = s.steps.map((st: any) => ({ cmd: st.cmd, cwd: st.cwd, background: st.background, ready: st.ready, raw: st.raw, timeout: st.timeout }))
      const body = { kind: 'session' as const, project: `${id}-${s.dir}`, files: projectFiles, steps, collect: ['Cargo.lock', 'go.sum'], local_target: s.local_target || undefined }
      jobs[name] = { ...body, dir: s.dir, key: hash(JSON.stringify(body)) }
    }
    return { id, base, doc, files, jobs }
  })
}

export function writeGuide(g: ReturnType<typeof loadGuides>[number], results: Record<string, any>, outDir: string, problems: string[], diagnostics: (s: string) => string) {
  const name = g.doc.name ?? g.id
  const clean = (s: string) => s.replace(/\/tmp\/job\/session/g, `/home/you/${name}`).replace(/\/tmp\/target\/[\w-]+/g, 'target')
  const sessions: Record<string, unknown[]> = {}
  for (const [sname, job] of Object.entries(g.jobs)) {
    const r = results[job.key]
    if (!r || r.runner_error) { problems.push(`guide ${g.id}/${sname}: ${r?.runner_error ?? 'not run'}`); continue }
    const spec = g.doc.sessions[sname].steps
    sessions[sname] = r.steps.map((st: any, i: number) => {
      const expect = spec[i].expect ?? 'ok'
      const failed = st.exit !== 0 && st.exit !== null
      if (!spec[i].background && expect === 'ok' && (failed || st.timed_out)) problems.push(`guide ${g.id}/${sname} step ${i + 1} (${st.cmd}) failed (exit ${st.exit}):\n${st.out}`)
      if (expect === 'fail' && !failed) problems.push(`guide ${g.id}/${sname} step ${i + 1} (${st.cmd}) was supposed to fail, but exited 0`)
      // A failed cargo build reads like a lesson's error: rustc's diagnostics, without
      // Cargo's "could not compile" summary.
      const out = failed && /^cargo (build|run|check|test|clippy)\b/.test(st.cmd) && /^(error|warning)/m.test(st.out) ? diagnostics(st.out) + '\n' : st.out
      // `varies: true` on a step in guide.yaml marks output that changes between runs: the page says so.
      return { show: spec[i].show ?? `$ ${st.cmd}`, out: clean(out), exit: st.exit, error: failed || undefined, varies: spec[i].varies || undefined }
    })
    // Pin what the first run resolved, so later runs build against the same versions.
    for (const [path, text] of Object.entries<string>(r.collected ?? {})) {
      const dest = join(g.base, job.dir, path)
      if (!existsSync(dest)) writeFileSync(dest, text)
    }
  }

  let gotcha: any
  if (g.doc.gotcha) {
    const t = g.doc.gotcha
    const last = (s?: string) => (s && sessions[s] ? (sessions[s].at(-1) as any) : undefined)
    const src = (f?: { file: string }) => (f ? readFileSync(join(g.base, f.file), 'utf8') : undefined)
    const ev = last(t.session)
    if (t.kind !== 'runtime' && ev && !ev.error) problems.push(`guide ${g.id}: the gotcha is supposed to fail, but its session succeeded`)
    gotcha = {
      ...t,
      code: src(t.code), code_name: t.code?.name, code_lang: t.code_lang ?? t.code?.lang,
      error: t.kind === 'runtime' || !ev ? undefined : t.kind === 'compile' ? diagnostics(ev.out) : ev.out.replace(/\n+$/, ''),
      error_cmd: ev?.show,
      error_code: ev?.out?.match(/^error\[(E\d+)\]/m)?.[1],
      out: t.kind === 'runtime' ? ev?.out : undefined,
      fixed: { code: src(t.fixed), name: t.fixed?.name, lang: t.fixed?.lang ?? (t.fixed?.name?.endsWith('.toml') ? 'toml' : undefined), out: last(t.fixed?.session)?.out, cmd: last(t.fixed?.session)?.show },
      langs: Object.fromEntries(Object.entries<any>(t.langs ?? {}).map(([k, v]) => [k, {
        status: v.status, says: v.says, mode: v.mode,
        code: v.file ? readFileSync(join(g.base, v.file), 'utf8') : undefined,
        out: last(v.session)?.out,
        varies: last(v.session)?.varies,
      }])),
    }
  }
  mkdirSync(outDir, { recursive: true })
  writeFileSync(join(outDir, `${g.id}.json`), JSON.stringify({ id: g.id, files: g.files, sessions, gotcha }, null, 1) + '\n')
}
