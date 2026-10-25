/**
 * Runs every program in lessons/ and writes what the site shows.
 *
 *     bun run extract            # run what changed, reuse the rest from the cache
 *     bun run extract --fresh    # ignore the cache and run everything again
 *     bun run extract --check    # fail if any claim a lesson makes is false
 *
 * Nothing on the site is typed by hand. A lesson folder holds prose (lesson.yaml) and
 * source files. This script compiles and runs each file inside the runner image
 * (tools/runner), with every toolchain pinned, and writes the outputs and compiler
 * errors to src/data/. It also checks every claim a lesson makes: that the gotcha
 * really fails to compile, that its fix really runs, that "crashes in C" really
 * crashes. A false claim fails the extract.
 *
 * Results are cached in src/data/runs.json by a hash of (image, language, mode, code),
 * so only changed programs run again. The cache is committed: a fresh clone builds the
 * site without Docker.
 */

import { $ } from 'bun'
import { createHash } from 'node:crypto'
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { loadGuides, writeGuide } from './guides.ts'
import { LANG, OTHERS, type Lang, type Mode, type Status, type Verdict } from '../src/lib/langs.ts'

const HERE = import.meta.dir
const REPO = join(HERE, '..', '..')
const LESSONS = join(REPO, 'lessons')
const DATA = join(HERE, '..', 'src', 'data')
const CACHE_FILE = join(DATA, 'runs.json')
const IMAGE = 'learnrust-runner:dev'
const RUNNER_DIR = join(REPO, 'tools', 'runner')
// run.py is mounted rather than baked in, so changing how a language runs does not
// need an image rebuild. Its hash is part of every cache key either way.
// seccomp=unconfined lets ThreadSanitizer switch off address randomisation (setarch -R).
// The volumes keep downloaded crates and build output between runs.
// linux/arm64, always: some outputs depend on the CPU (C's `char` is unsigned on ARM
// and signed on x86), so every run, local or CI, must be on the same architecture.
const DOCKER_ARGS = [
  '--platform', 'linux/arm64',
  '--security-opt', 'seccomp=unconfined',
  '-v', `${join(RUNNER_DIR, 'run.py')}:/opt/runner/run.py:ro`,
  '-v', 'learnrust-cargo-registry:/opt/cargo/registry',
  '-v', 'learnrust-target:/tmp/target',
]

const argv = process.argv.slice(2)
const args = new Set(argv)
const FRESH = args.has('--fresh')
// --only lessons | --only guide:<id> : run and write just that part. Guides keep their
// cache in their own file, so runs for different guides can happen at the same time.
const ONLY = argv.includes('--only') ? argv[argv.indexOf('--only') + 1] : undefined
const doLessons = !ONLY || ONLY === 'lessons'
const guideFilter = (id: string) => !ONLY || ONLY === `guide:${id}`

// The cache key includes a hash of the runner itself, so changing a toolchain version
// or how a language is compiled re-runs everything that language touches.
// Comments are left out of the hash, so explaining the Dockerfile better does not re-run
// six hundred programs.
const dockerfile = readFileSync(join(RUNNER_DIR, 'Dockerfile'), 'utf8').split('\n').filter(l => !/^\s*#/.test(l)).join('\n')
// Lines of run.py tagged `hash-exempt` add a capability without touching how any existing program
// runs (today: Kotlin programs that import kotlinx.coroutines), so they stay out of the hash.
const runPy = readFileSync(join(RUNNER_DIR, 'run.py'), 'utf8').split('\n').filter(l => !l.includes('hash-exempt')).join('\n')
const RUNNER_HASH = sha(dockerfile + runPy)

function sha(s: string) {
  return createHash('sha256').update(s).digest('hex').slice(0, 16)
}

// ---------------------------------------------------------------- reading lessons

type Job = { key: string; lang: Lang; mode?: Mode; code: string; hash: string }
type Result = {
  key: string; compiled: boolean; compile_out: string; exit?: number | null; out?: string
  timed_out?: boolean; stable?: boolean; out_2?: string; runner_error?: string; seconds?: number
}

const FILE: Record<Lang, string> = {
  rust: 'main.rs', go: 'main.go', kotlin: 'Main.kt', python: 'main.py', typescript: 'main.ts',
  java: 'Main.java', csharp: 'Program.cs', c: 'main.c', cpp: 'main.cpp',
}

const read = (p: string) => (existsSync(p) ? readFileSync(p, 'utf8') : undefined)

const jobs: Job[] = []
function job(key: string, lang: Lang, code: string, mode?: Mode): Job {
  const j = { key, lang, mode, code, hash: sha([RUNNER_HASH, lang, mode ?? '', code].join('\0')) }
  jobs.push(j)
  return j
}

/** Words of prose in an MDX file: code fences, tags, table rules and URLs don't count. */
function proseWords(mdx: string) {
  const text = mdx
    .replace(/```[\s\S]*?```/g, ' ')
    .replace(/<Memory[\s\S]*?\/>/g, ' ')
    .replace(/<[^>]+>/g, ' ')
    .replace(/\]\([^)]*\)/g, ' ')
    .replace(/^\|?[-| :]+\|?$/gm, ' ')
    .replace(/[|#*`>_-]/g, ' ')
  return text.split(/\s+/).filter(w => /\w/.test(w)).length
}
const WORD_LIMIT = 4000

const SNIPPET_EXT: Record<string, Lang> = { go: 'go', kt: 'kotlin', py: 'python', ts: 'typescript', java: 'java', cs: 'csharp', c: 'c', cpp: 'cpp' }
/** The language of a snippet file, from its extension (Rust snippets are handled separately). */
function snippetLang(file: string): Lang | undefined {
  return SNIPPET_EXT[file.split('.').pop() ?? '']
}
/** The markers between the name and the extension: `stale.crash.asan.cpp` -> ['crash', 'asan']. */
function snippetTokens(file: string): string[] {
  return file.split('.').slice(1, -1)
}
/** `race`, `asan` or `tsan` among the markers: run with the race detector or a sanitizer. */
function snippetMode(file: string): Mode | undefined {
  return snippetTokens(file).find((t): t is Mode => t === 'race' || t === 'asan' || t === 'tsan')
}

const lessonDirs = readdirSync(LESSONS).filter(d => /^\d\d[a-z]?-/.test(d)).sort()
const lessons = lessonDirs.map((dir, rank) => {
  const base = join(LESSONS, dir)
  const doc: any = Bun.YAML.parse(readFileSync(join(base, 'lesson.yaml'), 'utf8'))
  // Lesson numbers come from position in the folder order, not from a number in the
  // YAML, so folders can leave gaps for lessons not written yet without the site showing them.
  doc.order = rank + 1
  const id = dir.replace(/^\d\d[a-z]?-/, '')
  const program: Partial<Record<Lang, Job>> = {}
  for (const lang of ['rust', ...OTHERS] as Lang[]) {
    const code = read(join(base, FILE[lang]))
    if (code) program[lang] = job(`${id}/program/${lang}`, lang, code)
  }
  let gotcha: any
  if (doc.gotcha) {
    const g = join(base, 'gotcha')
    const langs: Partial<Record<Lang, Job>> = {}
    for (const lang of OTHERS) {
      const code = read(join(g, FILE[lang]))
      if (code) langs[lang] = job(`${id}/gotcha/${lang}`, lang, code, doc.gotcha.langs?.[lang]?.mode)
    }
    const proof = read(join(g, 'proof.go'))
    gotcha = {
      rust: job(`${id}/gotcha/rust`, 'rust', read(join(g, 'main.rs'))!),
      fixed: job(`${id}/gotcha/fixed`, 'rust', read(join(g, 'fixed.rs'))!),
      proof: proof ? job(`${id}/gotcha/proof`, 'go', proof) : undefined,
      langs,
    }
  }
  const faq = (doc.faq ?? []).map((f: any, i: number) => {
    const d = join(base, `faq-${i + 1}`)
    const bad = read(join(d, 'wont-compile.rs'))
    const good = read(join(d, 'main.rs'))
    return {
      bad: bad ? job(`${id}/faq-${i + 1}/bad`, 'rust', bad) : undefined,
      good: good ? job(`${id}/faq-${i + 1}/good`, 'rust', good) : undefined,
    }
  })
  // Layers 2 and 3 (lesson.mdx): every snippet and exercise is a real program.
  //   snippets/x.rs        must compile and run
  //   snippets/x.fail.rs   must fail to compile (the error is what the page shows)
  //   snippets/x.panic.rs  must compile, then stop with a panic or non-zero exit
  //   exercises/x.rs + exercises/x.solution.rs
  //                        the starter must compile and fail when run (todo!() or an
  //                        assert); the solution must pass the same checks
  const rsFiles = (sub: string) => (existsSync(join(base, sub)) ? readdirSync(join(base, sub)).filter(f => f.endsWith('.rs')).sort() : [])
  // Snippets in other languages: snippets/<name>[.fail|.crash][.race|.asan|.tsan].<ext>
  // (go, kt, py, ts, java, cs, c, cpp). Java files hold `public class Main`, C# top-level
  // statements, Kotlin a `fun main()`, exactly like the layer-1 programs.
  const otherSnippets = existsSync(join(base, 'snippets')) ? readdirSync(join(base, 'snippets')).filter(f => snippetLang(f) && !f.endsWith('.rs')).sort() : []
  const snippets = Object.fromEntries([
    ...rsFiles('snippets').map(f => [f, job(`${id}/snippets/${f}`, 'rust', read(join(base, 'snippets', f))!)] as const),
    ...otherSnippets.map(f => [f, job(`${id}/snippets/${f}`, snippetLang(f)!, read(join(base, 'snippets', f))!, snippetMode(f))] as const),
  ])
  const exercises = Object.fromEntries(rsFiles('exercises').filter(f => !f.endsWith('.solution.rs')).map(f => {
    const name = f.replace(/\.rs$/, '')
    const sol = read(join(base, 'exercises', `${name}.solution.rs`))
    return [name, {
      starter: job(`${id}/exercises/${name}`, 'rust', read(join(base, 'exercises', f))!),
      solution: sol ? job(`${id}/exercises/${name}.solution`, 'rust', sol) : undefined,
    }]
  }))
  const mdx = existsSync(join(base, 'lesson.mdx'))
  return { id, dir, doc, program, gotcha, faq, snippets, exercises, mdx, words: mdx ? proseWords(read(join(base, 'lesson.mdx'))!) : 0 }
})

const guides = loadGuides(join(REPO, 'guides'), s => sha(RUNNER_HASH + s), guideFilter)
if (!doLessons) jobs.length = 0
const sessionJobs = guides.flatMap(g => Object.values(g.jobs))

// ---------------------------------------------------------------- running

const readJson = (p: string) => (!FRESH && existsSync(p) ? JSON.parse(readFileSync(p, 'utf8')) : {})
const guideCacheFile = (id: string) => join(DATA, 'guides', `${id}.runs.json`)
const cache: Record<string, Result> = doLessons ? readJson(CACHE_FILE) : {}
const guideCache: Record<string, any> = {}
for (const g of guides) Object.assign(guideCache, readJson(guideCacheFile(g.id)))
const sortObj = (o: object) => JSON.stringify(Object.fromEntries(Object.entries(o).sort()), null, 1) + '\n'
function saveCaches() {
  mkdirSync(join(DATA, 'guides'), { recursive: true })
  if (doLessons) writeFileSync(CACHE_FILE, sortObj(cache))
  for (const g of guides) {
    const keys = new Set(Object.values(g.jobs).map(j => j.key))
    writeFileSync(guideCacheFile(g.id), sortObj(Object.fromEntries(Object.entries(guideCache).filter(([k]) => keys.has(k)))))
  }
}
const todo = [...new Map(jobs.filter(j => !cache[j.hash]).map(j => [j.hash, j])).values()]
const sessionTodo = [...new Map(sessionJobs.filter(j => !guideCache[j.key]).map(j => [j.key, j])).values()]

if (todo.length || sessionTodo.length) {
  const docker = await $`docker image inspect ${IMAGE}`.quiet().nothrow()
  if (docker.exitCode !== 0) {
    console.error(`${todo.length} programs need running, but the runner image is missing.`)
    console.error(`Build it:  docker build -t ${IMAGE} tools/runner`)
    process.exit(1)
  }
  console.log(`running ${todo.length} programs and ${sessionTodo.length} sessions (${jobs.length - todo.length + sessionJobs.length - sessionTodo.length} cached)`)
  // Languages in separate batches keep each container's compiler caches warm, and
  // let two batches run at once.
  const byLang = Object.groupBy(todo, j => j.lang)
  const run = async (batch: object[]) => {
    const p = Bun.spawn(['docker', 'run', '-i', '--rm', ...DOCKER_ARGS, IMAGE], {
      stdin: new Blob([JSON.stringify(batch)]), stdout: 'pipe', stderr: 'inherit',
    })
    const text = await new Response(p.stdout).text()
    if ((await p.exited) !== 0) throw new Error('runner failed')
    for (const r of JSON.parse(text) as any[]) (r.steps ? guideCache : cache)[r.key] = r
    // Save as we go: a later batch failing should not throw away finished work.
    saveCaches()
  }
  const batches: object[][] = Object.values(byLang).map(list => list!.map(j => ({ key: j.hash, lang: j.lang, mode: j.mode, code: j.code })))
  // Guide sessions build real crates (DataFusion's link step alone needs gigabytes), so
  // they run on their own first, never beside another batch.
  if (sessionTodo.length) await run(sessionTodo.map(({ dir, ...j }) => j))
  const queue = [...batches]
  await Promise.all([0, 1].map(async () => { while (queue.length) await run(queue.shift()!) }))
} else {
  console.log(`all ${jobs.length} programs cached`)
}

// Drop entries nothing refers to any more, so the committed cache does not grow forever.
const live = new Set(jobs.map(j => j.hash))
if (doLessons) for (const k of Object.keys(cache)) if (!live.has(k)) delete cache[k]
saveCaches()

// ---------------------------------------------------------------- cleaning output

/** rustc's diagnostics as a reader sees them, without Cargo's summary lines.
 *  Warnings are dropped on purpose: a lesson's compile error is about the one rule it teaches, and an
 *  unrelated warning beside it (an unused variable in a deliberately broken program) would be noise.
 *  A page that wants a warning shown runs a program that compiles, where it is printed with the output. */
function rustDiagnostics(out: string, { warnings = false } = {}) {
  const blocks = out.replace(/\n+$/, '').split(/\n\n(?=(?:error|warning)(?:\[E\d+\])?:)/)
  return blocks
    .filter(b => !/^error: could not compile/.test(b) && !/^warning: `[^`]+` \(.*\) generated/.test(b))
    .filter(b => warnings || !b.startsWith('warning'))
    .map(b => b.replace(/\n(error: could not compile|warning: `[^`]+` \(.*\) generated|For more information about).*$/s, ''))
    .map(trimStd)
    .join('\n\n')
}

const STD_MARK = '  ... (notes pointing into the standard library trimmed)'
const LIB_MARK = '  ... (notes pointing into library source code trimmed)'
// Paths into a crate's own source as Cargo downloaded it, e.g. tokio's or axum's.
const LIB_PATH = /\/registry\/src\//

/**
 * Newer rustc versions point notes into the standard library's own source
 * (`--> /rustc/<hash>/library/core/...`). Useful to a compiler hacker, noise to someone
 * learning, and the paths change with every release. Drop those notes, and say so.
 * The rule is mechanical: a help/note section that points into /rustc/ goes, and in
 * the main message, a /rustc/ span goes with the `= note:` attached to it.
 */
function trimStd(block: string): string {
  // rustc writes very long type names to a file and says so; the file is gone by the
  // time anyone reads the page.
  block = block.split('\n').filter(l => !/= note: (the full name for the type has been written to|consider using `--verbose` to print the full type name)/.test(l)).join('\n')
  const inStd = (l: string) => l.includes('/rustc/')
  const inLib = (l: string) => LIB_PATH.test(l)
  const parts: string[][] = [[]]
  for (const l of block.split('\n')) {
    if (/^(help|note):/.test(l)) parts.push([l])
    else parts.at(-1)!.push(l)
  }
  let std = false
  let lib = false
  const kept = parts.filter((p, i) => {
    if (i > 0 && p.some(inStd)) { std = true; return false }
    if (i > 0 && p.some(inLib)) { lib = true; return false }
    return true
  })
  const main: string[] = []
  let skipping = false
  for (const l of kept[0]) {
    if (/^\s*(-->|:::) /.test(l) && (inStd(l) || inLib(l))) { skipping = true; if (inStd(l)) std = true; else lib = true; continue }
    if (skipping && (/^\s*\|\s*$/.test(l) || /^\s*= note: /.test(l))) continue
    skipping = false
    main.push(l)
  }
  kept[0] = main
  // Lines that are only a gutter `|` carry nothing once what followed them is gone.
  const text = kept.map(p => p.join('\n')).join('\n').replace(/(\n\s*\|\s*)+$/, '')
  return lib ? `${text}\n${LIB_MARK}` : std ? `${text}\n${STD_MARK}` : text
}

/** Addresses, PIDs and thread ids change on every run; the reader cares about the shape. */
function normalise(out: string) {
  return out
    .replace(/0x[0-9a-f]{6,16}/g, m => '0x' + '·'.repeat(Math.min(12, m.length - 2)))
    .replace(/==\d+==/g, '==1==')
    // Rust 1.9x panics name the OS thread id: thread 'main' (402) panicked
    .replace(/(thread '[^']*') \(\d+\)/g, '$1 (1)')
    .replace(/\b(pid|tid)=\d+/g, '$1=1')
    .replace(/\/tmp\/job\/\w+\//g, '')
    .replace(/ \(BuildId: [0-9a-f]+\)/g, '')
}

/** Same output on every run, once addresses and process ids are set aside? */
const stable = (r: Result) => r.stable !== false || normalise(r.out ?? '') === normalise(r.out_2 ?? '')

// ---------------------------------------------------------------- checking claims

const problems: string[] = []
const notes: string[] = []
const R = (j: Job) => cache[j.hash]

function ran(j: Job, what: string) {
  const r = R(j)
  if (r.runner_error) problems.push(`${j.key}: runner error ${r.runner_error}`)
  else if (!r.compiled) problems.push(`${j.key}: ${what} does not compile:\n${indent(r.compile_out)}`)
  else if (r.timed_out) problems.push(`${j.key}: ${what} timed out`)
  else if (r.exit !== 0) problems.push(`${j.key}: ${what} exited with ${r.exit}:\n${indent(r.out ?? '')}`)
  if (!stable(r)) notes.push(`${j.key}: output differs between runs`)
  if (j.lang === 'rust' && r.compiled && /^warning/m.test(r.compile_out)) notes.push(`${j.key}: compiles with warnings:\n${indent(rustDiagnostics(r.compile_out, { warnings: true }))}`)
  return r
}
const indent = (s: string) => s.replace(/\n+$/, '').split('\n').map(l => '      ' + l).join('\n')

function checkStatus(j: Job, status: Status) {
  const r = R(j)
  if (r.runner_error) return problems.push(`${j.key}: runner error ${r.runner_error}`)
  const out = r.out ?? ''
  const ok = (() => {
    switch (status) {
      case 'refuses': return !r.compiled || (j.lang === 'python' && r.exit !== 0)
      case 'runs':
      case 'wrong': return r.compiled && r.exit === 0
      case 'crash': return r.compiled && (r.exit !== 0 || /Sanitizer|Exception|panic|Traceback|Segmentation/.test(out))
      case 'race': return r.compiled && (j.lang === 'go' && j.mode === 'race' ? /DATA RACE/.test(out) : j.mode === 'tsan' ? /ThreadSanitizer/.test(out) : true)
      case 'none': return false
    }
  })()
  if (!ok) problems.push(`${j.key}: lesson says "${status}", but it ${r.compiled ? `ran (exit ${r.exit}):\n${indent(out)}` : `did not compile:\n${indent(r.compile_out)}`}`)
}

function display(j: Job) {
  const r = R(j)
  if (!r.compiled) {
    const err = j.lang === 'rust' ? rustDiagnostics(r.compile_out) : r.compile_out.replace(/\n+$/, '')
    return { code: j.code, error: normalise(err) }
  }
  // What the compiler printed on the way to a successful build: gcc's -Wall warnings,
  // C#'s nullable and switch warnings. A reader running the command sees them, and
  // several gotchas are about exactly that warning.
  const warnings = j.lang !== 'rust' && r.compile_out.trim() ? normalise(r.compile_out.replace(/\n+$/, '')) : undefined
  return { code: j.code, warnings, out: normalise(r.out ?? ''), varies: !stable(r) || undefined }
}

// ---------------------------------------------------------------- writing

const outDir = join(DATA, 'lessons')
mkdirSync(outDir, { recursive: true })
const index: object[] = []
const wordCounts: string[] = []

for (const l of doLessons ? lessons : []) {
  const program: Record<string, unknown> = {}
  for (const [lang, j] of Object.entries(l.program) as [Lang, Job][]) {
    ran(j, 'lesson program')
    program[lang] = display(j)
  }

  let gotcha: any
  if (l.gotcha) {
    const g = l.doc.gotcha
    const rr = R(l.gotcha.rust)
    if (rr.compiled) problems.push(`${l.gotcha.rust.key}: the gotcha is supposed to fail to compile, but it compiles`)
    ran(l.gotcha.fixed, 'the fix')
    const langs: Record<string, unknown> = {}
    for (const lang of OTHERS) {
      const meta = g.langs?.[lang]
      const j = l.gotcha.langs[lang]
      if (!meta && !j) continue
      const status: Status = meta?.status ?? 'runs'
      if (j) checkStatus(j, status)
      else if (status !== 'none' && status !== 'refuses') problems.push(`${l.id}/gotcha/${lang}: status "${status}" but no ${FILE[lang]}`)
      langs[lang] = { status, mode: meta?.mode, says: meta?.says, ...(j ? display(j) : {}) }
    }
    const proof = l.gotcha.proof ? (ran(l.gotcha.proof, 'the Go proof'), display(l.gotcha.proof)) : undefined
    const err = rustDiagnostics(rr.compile_out)
    gotcha = {
      ...g,
      code: l.gotcha.rust.code,
      error: err,
      error_code: err.match(/^error\[(E\d+)\]/)?.[1],
      fixed: display(l.gotcha.fixed),
      proof,
      langs,
    }
  }

  const faq = (l.doc.faq ?? []).map((f: any, i: number) => {
    const jobsFor = l.faq[i]
    if (jobsFor.bad && R(jobsFor.bad).compiled) problems.push(`${jobsFor.bad.key}: supposed to fail to compile, but it compiles`)
    if (jobsFor.good) ran(jobsFor.good, 'the FAQ program')
    return { ...f, bad: jobsFor.bad && display(jobsFor.bad), good: jobsFor.good && display(jobsFor.good) }
  })

  const snippets: Record<string, unknown> = {}
  for (const [f, j] of Object.entries(l.snippets)) {
    const r = R(j)
    const tokens = snippetTokens(f)
    const wantsFail = tokens.includes('fail')
    const wantsCrash = tokens.includes('crash') || tokens.includes('panic')
    // A dynamic language has no compile step: a ".fail" Python snippet is one that raises.
    const failsAtRun = wantsFail && j.lang === 'python'
    const sanitizerHit = /Sanitizer|DATA RACE|panicked|Exception|Traceback|Unhandled|Error/.test(r.out ?? '')
    if (wantsFail && !failsAtRun) {
      if (r.compiled) problems.push(`${j.key}: named .fail, but it compiles`)
    } else if (wantsCrash || failsAtRun) {
      if (!r.compiled) problems.push(`${j.key}: named .${failsAtRun ? 'fail' : 'crash/.panic'}, but it does not compile:\n${indent(r.compile_out)}`)
      else if (r.exit === 0 && !sanitizerHit) problems.push(`${j.key}: named .${failsAtRun ? 'fail' : 'crash/.panic'}, but it exits 0 with nothing wrong in its output`)
    } else if (!/^check-/.test(f)) ran(j, 'snippet')
    else ran(j, 'check')
    if (j.lang === 'rust' && r.compiled && /^warning/m.test(r.compile_out) && !wantsFail) notes.push(`${j.key}: compiles with warnings`)
    snippets[f] = { ...display(j), lang: j.lang, mode: j.mode, exit: r.exit }
  }

  const exercises: Record<string, unknown> = {}
  for (const [name, ex] of Object.entries(l.exercises)) {
    const st = R(ex.starter)
    if (!st.compiled) problems.push(`${ex.starter.key}: the starter must compile (use todo!() for the missing parts):\n${indent(st.compile_out)}`)
    else if (st.exit === 0) problems.push(`${ex.starter.key}: the starter already passes; it should fail until the reader fills it in`)
    if (!ex.solution) problems.push(`${ex.starter.key}: no ${name}.solution.rs`)
    else {
      ran(ex.solution, 'the solution')
      // The checks in main() must be the same in both, so a solution cannot pass by
      // weakening them.
      const tail = (code: string) => code.slice(code.indexOf('fn main('))
      if (tail(ex.starter.code) !== tail(ex.solution.code)) problems.push(`${ex.starter.key}: main() differs between the starter and the solution`)
    }
    exercises[name] = { starter: ex.starter.code, solution: ex.solution && display(ex.solution) }
  }

  if (l.words > WORD_LIMIT) notes.push(`${l.id}: lesson.mdx is ${l.words} words (over ${WORD_LIMIT}); move reference material into tables or cut`)
  if (l.words) wordCounts.push(`${l.id}: ${l.words}`)

  const data = { id: l.id, ...l.doc, program, gotcha, faq, snippets, exercises, mdx: l.mdx || undefined }
  delete (data as any).langs
  writeFileSync(join(outDir, `${l.id}.json`), JSON.stringify(data, null, 1) + '\n')
  index.push({ id: l.id, dir: l.dir, title: l.doc.title, part: l.doc.part, order: l.doc.order, verdict: (l.doc.gotcha?.verdict as Verdict) ?? null, mdx: l.mdx })
}

for (const g of guides) writeGuide(g, guideCache, join(DATA, 'guides'), problems, rustDiagnostics)

// Every error code that appears on the site needs a plain-language note on the error index.
{
  const known = new Set([...readFileSync(join(HERE, '..', 'src', 'lib', 'error-notes.ts'), 'utf8').matchAll(/^\s{2}(E\d{4}):/gm)].map(m => m[1]))
  const shown = new Set<string>()
  const scanCodes = (t?: string) => { for (const m of (t ?? '').matchAll(/^error\[(E\d{4})\]/gm)) shown.add(m[1]) }
  for (const l of lessons) {
    scanCodes(R(l.gotcha?.rust ?? ({} as Job))?.compile_out)
  }
  for (const f of readdirSync(outDir).filter(f => f.endsWith('.json'))) {
    const d = JSON.parse(readFileSync(join(outDir, f), 'utf8'))
    scanCodes(d.gotcha?.error); for (const s of Object.values<any>(d.snippets ?? {})) scanCodes(s.error)
    for (const q of d.faq ?? []) scanCodes(q.bad?.error)
  }
  const gdir = join(DATA, 'guides')
  if (existsSync(gdir)) for (const f of readdirSync(gdir).filter(f => f.endsWith('.json') && !f.endsWith('.runs.json'))) {
    const d = JSON.parse(readFileSync(join(gdir, f), 'utf8'))
    for (const steps of Object.values<any[]>(d.sessions ?? {})) for (const st of steps) scanCodes(st.out)
    scanCodes(d.gotcha?.error)
  }
  const missing = [...shown].filter(c => !known.has(c)).sort()
  if (missing.length) notes.push(`error index: no note yet for ${missing.join(', ')} (add to website/src/lib/error-notes.ts)`)
}

if (doLessons) writeFileSync(join(DATA, 'lessons.json'), JSON.stringify(index, null, 1) + '\n')

if (todo.length || !existsSync(join(DATA, 'versions.json'))) {
  const v = await $`docker run --rm ${IMAGE} versions`.nothrow().text()
  if (v.trim()) writeFileSync(join(DATA, 'versions.json'), v.trim() + '\n')
}

// ---------------------------------------------------------------- report

const REPORT = join(DATA, ONLY ? `extract-report.${ONLY.replace(':', '-')}.txt` : 'extract-report.txt')
if (wordCounts.length) console.log(`words (lesson.mdx): ${wordCounts.join(', ')}`)
writeFileSync(REPORT,
  `problems: ${problems.length}\n\n${problems.join('\n\n')}\n\nnotes: ${notes.length}\n\n${notes.join('\n\n')}\n`)
console.log(`${doLessons ? lessons.length : 0} lessons, ${guides.length} guides, ${jobs.length} programs, ${sessionJobs.length} sessions, ${problems.length} problems, ${notes.length} notes`)
console.log(`details: ${REPORT.slice(REPO.length + 1)}`)
if (problems.length && args.has('--check')) process.exit(1)
