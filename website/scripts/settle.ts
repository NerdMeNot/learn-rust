/**
 * Keeping the committed output when a re-run differs only by noise.
 *
 * Freshness (.github/workflows/freshness.yml) re-runs every program and fails if any shown
 * output changed. Many outputs change on every run without anything having happened: a
 * thread id in a panic, how long a test took, the order parallel tests finish in, a temp
 * folder's name, and everything in a step the page already marks as varying (timings,
 * benchmarks, data races). If those reached the diff, the job would fail every week and
 * nobody would read it.
 *
 * So when the extract writes a lesson's or a guide's JSON, it settles each output against
 * the one already on disk: a step marked `varies` keeps its committed run (the page says
 * the output is one run of many, and its exit code too while it still passes or fails the
 * same way), and any other output keeps its committed text if the two
 * are the same once the noise below is set aside. What is left in the diff is a real
 * change: a new compiler message, a crate that behaves differently, a tool's new version.
 *
 * None of this touches the checks: whether a program compiles, fails, crashes or passes
 * its tests is decided from the new run, before anything is settled.
 */

/** What the page shows: addresses, process and thread ids set to fixed values. */
export function normalise(out: string) {
  return out
    .replace(/0x[0-9a-f]{6,16}/g, m => '0x' + '·'.repeat(Math.min(12, m.length - 2)))
    .replace(/==\d+==/g, '==1==')
    // Rust 1.9x panics name the OS thread id: thread 'main' (402) panicked
    .replace(/(thread '[^']*') \(\d+\)/g, '$1 (1)')
    .replace(/\b(pid|tid)=\d+/g, '$1=1')
    .replace(/\/tmp\/job\/\w+\//g, '')
    .replace(/ \(BuildId: [0-9a-f]+\)/g, '')
}

/** Output with what changes from run to run removed, for comparing two runs. */
export function quiet(out: string) {
  return normalise(out)
    // Durations: "finished in 0.01s", "0.371546ms", Go's "ok pkg 0.003s", "Time Elapsed 00:00:02.21".
    .replace(/\b\d{2}:\d{2}:\d{2}\.\d+\b/g, 'T')
    .replace(/\b\d+(?:\.\d+)?\s?(?:ns|µs|us|ms|s)\b/g, 'T')
    // Node's test runner: "duration_ms 43.47771".
    .replace(/\bduration_ms [\d.]+/g, 'duration_ms T')
    // Temp folders named at random: /tmp/.tmpZJyTDF/
    .replace(/\/tmp\/\.tmp\w+/g, '/tmp/.tmp')
    .split('\n')
    // Upgrade notices depend on the day, not on the program: pip's "[notice] A new release of
    // pip is available", wasm-pack's "There's a newer version of wasm-pack available".
    .filter(l => !/^\[notice\] /.test(l) && !/There's a newer version of [\w-]+ available/.test(l))
    // Parallel tests, threads and hash maps print in an order that changes between runs.
    .sort()
    .join('\n')
}

const OUTPUT_KEYS = new Set(['out', 'error', 'warnings'])

/**
 * `fresh`, with each output field replaced by the one in `committed` when the two differ only
 * by noise (or the step is marked as varying). Walks objects and arrays in step.
 */
export function settle(committed: unknown, fresh: unknown): unknown {
  if (Array.isArray(fresh)) return Array.isArray(committed) ? fresh.map((v, i) => settle(committed[i], v)) : fresh
  if (!fresh || typeof fresh !== 'object' || !committed || typeof committed !== 'object') return fresh
  const c = committed as Record<string, unknown>
  const f = fresh as Record<string, unknown>
  // Once a run has been seen to vary, it varies: a later pair of runs that happened to agree
  // (two hash maps that came out in the same order) does not make it stable.
  const varies = f.varies === true || c.varies === true
  const out: Record<string, unknown> = {}
  for (const [k, v] of Object.entries(f)) {
    const old = c[k]
    if (OUTPUT_KEYS.has(k) && typeof v === 'string' && typeof old === 'string') out[k] = varies || quiet(old) === quiet(v) ? old : v
    // A varying step may fail with a different code (a pipe's SIGPIPE gives 141 instead of 1).
    // Whether it passed or failed is what the page claims, and that was checked on the new run.
    else if (k === 'exit' && varies && typeof v === 'number' && typeof old === 'number' && (v === 0) === (old === 0)) out[k] = old
    else out[k] = settle(old, v)
  }
  if (varies) out.varies = true
  return out
}
