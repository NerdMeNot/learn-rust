// Learn Rust (even with all your past baggage) — learningrust.nerdmenot.in
//
// Same split as the assay and decoy sites: Starlight owns `src/content/docs/**` for
// what it does well (sidebar, search, keyboard nav, heading anchors, a11y), and the
// landing page is a plain Astro route with its own layout.
import { defineConfig } from 'astro/config'
import starlight from '@astrojs/starlight'
import { readFileSync, existsSync } from 'node:fs'

// Lessons are pages built from src/data (see src/pages/learn/[id].astro), so the
// sidebar for them is built here from the index the extract script writes.
const lessons = JSON.parse(readFileSync(new URL('./src/data/lessons.json', import.meta.url), 'utf8'))
const PARTS = [
  ['syntax', 'Syntax'], ['basics', 'Basics'], ['ownership', 'Ownership'], ['data', 'Data'],
  ['collections', 'Working with collections'], ['abstractions', 'Abstractions'],
  ['smart-pointers', 'Smart pointers'], ['concurrency', 'Concurrency'],
]
// Pages that are written in batches: a sidebar entry appears once its file exists.
const docPages = (dir, ids) => ids.filter(id => existsSync(new URL(`./src/content/docs/${dir}/${id}.mdx`, import.meta.url))).map(id => ({ slug: `${dir}/${id}` }))
const lessonLinks = part =>
  lessons.filter(l => l.part === part).map(l => ({ label: `${l.order}. ${l.title}`, link: `/learn/${l.id}/` }))

export default defineConfig({
  site: 'https://learningrust.nerdmenot.in',
  // Lesson pages import lessons/*/lesson.mdx from outside website/.
  vite: { server: { fs: { allow: ['..'] } } },
  integrations: [
    starlight({
      title: 'Learn Rust',
      description:
        'Rust for people who already know another language. Every lesson puts a Rust ' +
        'program next to the same program in Go, Kotlin, Python, TypeScript, Java, C#, ' +
        'C or C++, then shows the habit from that language that Rust refuses, and why.',
            favicon: '/icon.svg',
      customCss: ['./src/styles/fonts.css', './src/styles/theme.css', './src/styles/lesson.css', './src/styles/motion.css', './src/styles/print.css'],
      components: {
        Head: './src/components/Head.astro',
        PageTitle: './src/components/PageTitle.astro',
        ThemeSelect: './src/components/ThemeSelect.astro',
        Header: './src/components/Header.astro',
        Footer: './src/components/Footer.astro',
      },
      // A light code theme in light mode, dark in dark: the lessons put several code
      // panels side by side, and a page of dark slabs on a light page is hard to read.
      expressiveCode: {
        themes: ['github-dark', 'github-light'],
        styleOverrides: {
          borderRadius: '8px',
          codeFontFamily: 'var(--lr-mono)',
        },
      },
      // TODO: add `social` and `editLink` once the GitHub repo exists.
      lastUpdated: true,
      pagination: true,
      head: [
        // The same card for every page unless a page names its own: shared links get a picture.
        { tag: 'meta', attrs: { property: 'og:image', content: 'https://learningrust.nerdmenot.in/social-card.png' } },
        { tag: 'meta', attrs: { property: 'og:image:width', content: '1200' } },
        { tag: 'meta', attrs: { property: 'og:image:height', content: '630' } },
        { tag: 'meta', attrs: { name: 'twitter:image', content: 'https://learningrust.nerdmenot.in/social-card.png' } },
        // Fonts are served from /fonts (src/styles/fonts.css). The three faces every page
        // paints with are preloaded so a page change never swaps them in late.
        ...['atkinson-hyperlegible-normal-400', 'bricolage-grotesque-normal-600', 'fredoka-normal-600'].map(f => ({
          tag: 'link',
          attrs: { rel: 'preload', as: 'font', type: 'font/woff2', href: `/fonts/${f}.woff2`, crossorigin: true },
        })),
      ],
      sidebar: [
        {
          label: 'Start here',
          collapsed: true,
          items: [
            { slug: 'start/why-rust' }, { slug: 'start/how-rust-thinks' }, { slug: 'start/first-project' },
            { slug: 'start/using-crates' }, { slug: 'start/files-and-modules' }, { slug: 'start/several-programs' },
            { label: 'Reading compiler errors', link: '/errors/' },
            { slug: 'start/glossary' },
          ],
        },
        {
          label: 'Learn Rust',
          items: PARTS.map(([id, label]) => ({
            label,
            // Only the part you are in is open; Starlight opens a collapsed group that holds the current page.
            collapsed: true,
            // The guided program closes the Data part: it uses what those lessons teach.
            items: [...lessonLinks(id), ...(id === 'data' ? [{ label: 'Project: a word counter', link: '/start/word-frequency/' }] : [])],
          })).filter(g => g.items.length > 0),
        },
        { label: 'Standard library', collapsed: true, items: [{ slug: 'std/tour' }, ...lessonLinks('std')] },
        {
          label: 'Build with Rust',
          collapsed: true,
          items: ['crates-map', 'serde', 'tokio', 'rayon', 'rayon-tokio', 'axum', 'grpc', 'sqlx', 'clap', 'app-errors', 'tracing', 'testing', 'datafusion'].map(s => ({ slug: `build/${s}` })),
        },
        {
          label: 'Going further',
          collapsed: true,
          items: [...lessonLinks('further'), ...docPages('further', ['project-structure', 'testing-more', 'performance'])],
        },
        { label: 'Projects', collapsed: true, items: docPages('projects', ['todo', 'money', 'notes-api', 'indexer', 'shop']) },
        { label: 'Cheat sheets', collapsed: true, items: docPages('cheatsheets', ['syntax']) },
        { label: 'Rust for your language', collapsed: true, items: docPages('for', ['go', 'java', 'kotlin', 'csharp', 'python', 'typescript', 'c-cpp']) },
        { label: 'Use Rust from…', collapsed: true, items: docPages('interop', ['python', 'node', 'c-cpp', 'wasm']) },
        {
          label: 'Checkpoints',
          collapsed: true,
          items: docPages('checkpoints', ['syntax', 'basics', 'ownership', 'data', 'collections', 'abstractions', 'smart-pointers', 'concurrency', 'std']),
        },
      ],
    }),
  ],
})
