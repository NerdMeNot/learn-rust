// How long a page takes to read, to the nearest few minutes. It counts prose at 200 words a
// minute and code at 25 lines a minute, and skips imports, tags and markup. A <Snippet> or
// <Session> pulls its code from a file, so each one counts as a fixed block of lines. This
// is an estimate, which is why the pages say "about".
const WORDS_PER_MIN = 200
const CODE_LINES_PER_MIN = 25
const LINES_PER_EMBED = 10

export function minutes(text: string, extraCodeLines = 0): number {
  const fences = [...text.matchAll(/^```[^\n]*\n([\s\S]*?)^```/gm)]
  const codeLines = fences.reduce((n, m) => n + m[1].split('\n').length, 0)
  const embeds = (text.match(/<(Snippet|Session|Files|Exercise)\b/g) ?? []).length
  const prose = text
    .replace(/^```[\s\S]*?^```/gm, ' ')
    .replace(/^import .*$/gm, ' ')
    .replace(/<[^>]+>/g, ' ')
    .replace(/[`*_#>|\[\]()]/g, ' ')
  const words = prose.split(/\s+/).filter(w => /[A-Za-z0-9]/.test(w)).length
  return words / WORDS_PER_MIN + (codeLines + embeds * LINES_PER_EMBED + extraCodeLines) / CODE_LINES_PER_MIN
}

/** "5 min", "15 min": whole minutes under 10, then the nearest 5. */
export function approx(m: number): string {
  const r = m < 10 ? Math.max(1, Math.round(m)) : Math.round(m / 5) * 5
  return `${r} min`
}
