"""Fail if any internal link or #anchor in the built site points nowhere.

    python3 scripts/check-links.py dist

Adapted from the assay site's check. A heading gets reworded far more often than a page
gets deleted, so anchors are checked too.
"""

import os
import re
import sys

root = sys.argv[1] if len(sys.argv) > 1 else 'dist'
pages, ids = set(), {}
for d, _, fs in os.walk(root):
    for f in fs:
        if f != 'index.html':
            continue
        rel = os.path.relpath(d, root).replace('\\', '/')
        slug = '/' if rel == '.' else '/' + rel.strip('/') + '/'
        pages.add(slug)
        with open(os.path.join(d, f), encoding='utf-8') as fh:
            ids[slug] = set(re.findall(r'id="([^"]+)"', fh.read()))

bad = []
skip_ext = ('.svg', '.png', '.xml', '.js', '.css', '.ico', '.txt', '.json', '.webmanifest', '.woff2')
for d, _, fs in os.walk(root):
    for f in fs:
        if not f.endswith('.html'):
            continue
        here = os.path.join(d, f)
        with open(here, encoding='utf-8') as fh:
            src = fh.read()
        for href in re.findall(r'href="(/[^"?]*)"', src):
            if href.startswith(('/_', '//')) or href.endswith(skip_ext):
                continue
            page, _, anchor = href.partition('#')
            h = page if page.endswith('/') else page + '/'
            if h not in pages:
                bad.append((here, href, 'no such page'))
            elif anchor and anchor not in ids[h]:
                bad.append((here, href, 'no such anchor'))

for src, h, why in sorted(set(bad)):
    print(f'BROKEN {h} in {src} ({why})')
print(f'checked {len(pages)} pages, {len(set(bad))} broken links')
sys.exit(1 if bad else 0)
