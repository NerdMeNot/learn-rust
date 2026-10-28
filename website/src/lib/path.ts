// The order a newcomer reads the site in. The home page's path, its "Start here" link and the
// "where next" lines at the end of the first pages all read from here, so they cannot
// disagree. The sidebar's "Start here" group opens with the same three pages.
export const START = [
  { href: '/start/why-rust/', note: 'The sourced case for and against. Skip it if you have already decided.' },
  { href: '/start/how-rust-thinks/', note: 'The mindset behind the rules, before any syntax.' },
  { href: '/start/first-project/', note: 'Install Rust, make a project, build it and run it.' },
] as const
