// Guide pages read their projects and sessions from src/data/guides/<id>.json, which
// `bun run extract` writes from guides/<id>/. Components only ever see real files and
// real output.
const all = import.meta.glob<{ default: GuideData }>('../../data/guides/*.json', { eager: true })

export type Step = { show: string; out: string; exit: number | null; error?: boolean; varies?: boolean }
export type GuideData = {
  id: string
  files: Record<string, Record<string, string>> // dir -> path -> text
  sessions: Record<string, Step[]>
  gotcha?: any
}

// A guide whose sessions have not been run yet still builds, with a visible marker in
// place of each missing output, so one unfinished page does not block the rest of the
// site. `bun run extract --check` is what refuses to ship it.
const EMPTY: GuideData = { id: '', files: {}, sessions: {} }
export const MISSING = '⚠ not run yet: bun run extract'

export function guide(id: string): GuideData {
  return all[`../../data/guides/${id}.json`]?.default ?? EMPTY
}

export function file(id: string, dir: string, path: string): string {
  return guide(id).files[dir]?.[path] ?? `// ${MISSING} (guides/${id}/${dir}/${path})`
}

export function session(id: string, name: string): Step[] {
  return guide(id).sessions[name] ?? [{ show: `# session ${name}`, out: MISSING, exit: null }]
}

const EXT: Record<string, string> = { rs: 'rust', toml: 'toml', go: 'go', mod: 'go', proto: 'protobuf', sql: 'sql', json: 'json', md: 'md', kt: 'kotlin', kts: 'kotlin', py: 'python', ts: 'typescript', js: 'javascript', mjs: 'javascript', sh: 'sh', cs: 'csharp', java: 'java', c: 'c', h: 'c', cpp: 'cpp', hpp: 'cpp', yaml: 'yaml', yml: 'yaml', xml: 'xml', csproj: 'xml', lock: 'toml' }
export const langOf = (path: string) => EXT[path.split('.').pop() ?? ''] ?? 'text'
