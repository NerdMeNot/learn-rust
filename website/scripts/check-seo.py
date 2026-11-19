"""Fail if a built page is missing what search engines and link previews read.

    python3 scripts/check-seo.py dist

Every indexable page needs a <title>, a canonical URL and a meta description. Titles must be
unique, and a description must fit the roughly 160 characters a search result shows (a
lesson's falls back to its intro, so a long intro needs a `description:` in lesson.yaml).
"""

import html
import os
import re
import sys

root = sys.argv[1] if len(sys.argv) > 1 else 'dist'
MIN, MAX = 50, 160
bad, titles = [], {}
for d, _, fs in os.walk(root):
    if os.path.relpath(d, root).startswith('pagefind'):
        continue
    for f in fs:
        if not f.endswith('.html'):
            continue
        path = os.path.relpath(os.path.join(d, f), root)
        with open(os.path.join(d, f), encoding='utf-8') as fh:
            head = fh.read().split('</head>')[0]
        if re.search(r'<meta name="robots" content="[^"]*noindex', head):
            continue
        title = re.search(r'<title>(.*?)</title>', head, re.S)
        desc = re.search(r'<meta name="description" content="([^"]*)"', head)
        if not title:
            bad.append(f'{path}: no <title>')
        else:
            titles.setdefault(html.unescape(title.group(1)), []).append(path)
        if 'rel="canonical"' not in head:
            bad.append(f'{path}: no canonical link')
        if not desc:
            bad.append(f'{path}: no meta description')
        else:
            n = len(html.unescape(desc.group(1)))
            if not MIN <= n <= MAX:
                bad.append(f'{path}: description is {n} characters (want {MIN}-{MAX})')
for t, paths in titles.items():
    if len(paths) > 1:
        bad.append(f'title "{t}" is on {len(paths)} pages: {", ".join(paths)}')

if bad:
    print('\n'.join(bad))
    sys.exit(1)
print(f'SEO basics OK on {sum(len(p) for p in titles.values())} pages')
