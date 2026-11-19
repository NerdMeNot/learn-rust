// Every compiler error code the site shows, where it shows it, and the worked examples an
// error page can be built from. Read from the extract data only, so it cannot drift from
// the real runs: a new gotcha appears on the error index, and on its code's page, by itself.
//
// Used by /errors/ (the index) and /errors/<code>/ (one page per code that has a note with
// a `name`, see error-notes.ts).
import { getCollection } from 'astro:content'
import { ERROR_NOTES } from './error-notes'

export type Hit = { href: string; label: string; message: string }
/** A gotcha (Rust beside the other languages, the error, the fix), as Gotcha.astro takes it. */
export type GotchaExample = { kind: 'gotcha'; href: string; label: string; g: any }
/** A program that fails to compile, with the error it gets. */
export type PlainExample = { kind: 'plain'; href: string; label: string; code: string; error: string }
export type Example = GotchaExample | PlainExample
export type ErrorEntry = { code: string; hits: Hit[]; examples: Example[] }

const lessons = import.meta.glob<{ default: any }>('../data/lessons/*.json', { eager: true })
import order from '../data/lessons.json'
const guides = import.meta.glob<{ default: any }>('../data/guides/*.json', { eager: true })

const CODE_LINE = /^error\[(E\d{4})\]: (.*)$/gm
const codesIn = (text: string | undefined) => [...(text ?? '').matchAll(CODE_LINE)].map(m => ({ code: m[1], message: m[2] }))

/** The URL of a code's own page, or undefined when it has none (no `name` in its note). */
export const errorPage = (code: string | undefined) =>
  code && ERROR_NOTES[code]?.name ? `/errors/${code.toLowerCase()}/` : undefined

let cached: Promise<Map<string, ErrorEntry>> | undefined
export function errorIndex(): Promise<Map<string, ErrorEntry>> {
  return (cached ??= build())
}

async function build(): Promise<Map<string, ErrorEntry>> {
  const docs = await getCollection('docs')
  const index = new Map<string, ErrorEntry>()
  const entry = (code: string) => {
    let e = index.get(code)
    if (!e) index.set(code, (e = { code, hits: [], examples: [] }))
    return e
  }
  const scan = (text: string | undefined, href: string, label: string) => {
    for (const { code, message } of codesIn(text)) {
      const e = entry(code)
      // One entry per page, whichever part of the page shows it.
      if (!e.hits.some(h => h.href.split('#')[0] === href.split('#')[0])) e.hits.push({ href, label, message })
    }
  }
  // A program counts as an example of the first error it gets: that is the one to read.
  const example = (text: string | undefined, ex: Omit<PlainExample, 'kind' | 'error'> | Omit<GotchaExample, 'kind'>) => {
    const first = codesIn(text)[0]
    if (!first) return
    entry(first.code).examples.push('g' in ex ? { kind: 'gotcha', ...ex } : { kind: 'plain', error: text!, ...ex })
  }

  // Lessons in course order, so a code's first example and its list of pages follow the path a
  // reader takes (Ownership's E0382 before Borrowing's).
  for (const { id } of order) {
    const l = lessons[`../data/lessons/${id}.json`].default
    const href = `/learn/${l.id}/`
    if (l.gotcha) {
      scan(l.gotcha.error, `${href}#gotcha`, l.title)
      example(l.gotcha.error, { href: `${href}#gotcha`, label: l.title, g: l.gotcha })
    }
    for (const s of Object.values<any>(l.snippets ?? {})) {
      scan(s.error, href, l.title)
      if (s.lang === 'rust' && s.code) example(s.error, { href, label: l.title, code: s.code })
    }
    for (const q of l.faq ?? []) {
      scan(q.bad?.error, href, l.title)
      if (q.bad?.code) example(q.bad.error, { href: `${href}#questions`, label: l.title, code: q.bad.code })
    }
  }

  const guideRoute = (id: string) => {
    // A guide's id is its page's last segment (app-errors: build/app-errors), or its track and
    // name joined by a dash (further-performance: further/performance; project-todo: projects/todo).
    const hit = docs.find(d => d.id.endsWith(`/${id}`) || d.id === id || d.id === id.replace('-', '/') || d.id === id.replace(/^project-/, 'projects/'))
      ?? docs.find(d => d.id === 'std/tour' && id === 'std-tour')
    return hit ? `/${hit.id}/` : undefined
  }
  for (const path of Object.keys(guides).sort()) {
    if (path.endsWith('.runs.json')) continue
    const g = guides[path].default
    const href = guideRoute(g.id)
    if (!href) continue
    const title = docs.find(d => `/${d.id}/` === href)?.data.title ?? g.id
    for (const steps of Object.values<any[]>(g.sessions ?? {})) for (const s of steps) scan(s.out, href, title)
    scan(g.gotcha?.error, href, title)
    if (g.gotcha && (g.gotcha.kind ?? 'compile') === 'compile') example(g.gotcha.error, { href, label: title, g: g.gotcha })
  }

  // Gotchas first (they carry the other languages and the fix), then the shortest programs.
  for (const e of index.values()) {
    e.examples.sort((a, b) => (a.kind === b.kind ? (a.kind === 'plain' ? a.code.length - (b as PlainExample).code.length : 0) : a.kind === 'gotcha' ? -1 : 1))
  }
  return index
}
