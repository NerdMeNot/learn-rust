// The data a lesson page renders from: src/data/lessons/<id>.json, written by
// `bun run extract`. Components inside lesson.mdx find their lesson through
// Astro.locals.lessonId, which the page sets before rendering the MDX.
const all = import.meta.glob<{ default: any }>('../../data/lessons/*.json', { eager: true })

export function lessonData(id: string | undefined) {
  if (!id) throw new Error('lesson component used outside a lesson page')
  const l = all[`../../data/lessons/${id}.json`]?.default
  if (!l) throw new Error(`no data for lesson "${id}": run \`bun run extract\``)
  return l
}

/** A Rust Playground link that opens this code, ready to run and edit. */
export const playground = (code: string) =>
  `https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=${encodeURIComponent(code)}`
