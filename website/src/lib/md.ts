// Lesson prose is Markdown: inline for one-line fields (intro, notes, verdict text),
// block for answers that hold lists. Links that leave the site open in a new tab.
import { Marked } from 'marked'

const marked = new Marked({
  gfm: true,
  renderer: {
    link({ href, text }) {
      const ext = /^https?:/.test(href)
      return `<a href="${href}"${ext ? ' target="_blank" rel="noopener"' : ''}>${text}</a>`
    },
  },
})

export const inline = (s: string | undefined) => (s ? (marked.parseInline(s) as string) : '')
export const block = (s: string | undefined) => (s ? (marked.parse(s) as string) : '')
export const plain = (s: string | undefined) => (s ?? '').replace(/`([^`]+)`/g, '$1').replace(/\*\*?([^*]+)\*\*?/g, '$1').replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')
