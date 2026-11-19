"""Tell search engines which pages changed, with IndexNow (Bing, Yandex, Seznam, Naver...).

    python3 scripts/indexnow.py prepare dist   # before deploying: write dist/indexnow.json
    python3 scripts/indexnow.py submit dist    # after deploying: announce what changed

IndexNow asks for URLs "as soon as the content is added, updated, or deleted", not the
whole site on every deploy (https://www.indexnow.org/documentation). So each deploy
publishes a manifest of page-content hashes at /indexnow.json; the next deploy reads the
live one, compares, and submits only pages that are new, changed or gone. With no live
manifest (the first deploy), every page is new.

A page's hash covers its <main> element, so a rebuilt stylesheet or script name does not
count as a change. Ownership is proven by the key file at the site root (public/<key>.txt,
copied to dist/ by the build).

Never fails the deploy: a search engine being unreachable is not a reason to stop.
"""

import glob
import hashlib
import json
import os
import re
import sys
import urllib.request

SITE = 'https://learningrust.nerdmenot.in'
ENDPOINT = 'https://api.indexnow.org/indexnow'
# Cloudflare refuses Python's default User-Agent (403), so requests name themselves.
UA = {'User-Agent': 'learningrust-deploy (IndexNow)'}
CHANGES = os.path.join(os.environ.get('RUNNER_TEMP', '/tmp'), 'indexnow-changes.json')


def pages(dist):
    out = {}
    for path in glob.glob(os.path.join(dist, '**', 'index.html'), recursive=True):
        rel = os.path.relpath(os.path.dirname(path), dist).replace('\\', '/')
        if rel.startswith('pagefind') or rel == '404':
            continue
        html = open(path, encoding='utf-8').read()
        if re.search(r'<meta name="robots" content="[^"]*noindex', html):
            continue
        m = re.search(r'<main[\s>].*?</main>', html, re.S) or re.search(r'<body[\s>].*?</body>', html, re.S)
        url = SITE + ('/' if rel == '.' else f'/{rel}/')
        out[url] = hashlib.sha256((m.group(0) if m else html).encode()).hexdigest()[:16]
    return dict(sorted(out.items()))


def live_manifest():
    try:
        with urllib.request.urlopen(urllib.request.Request(f'{SITE}/indexnow.json', headers=UA), timeout=20) as r:
            return json.load(r)
    except Exception as e:  # first deploy, or the site is down: treat every page as new
        print(f'no live manifest ({e}); every page counts as new')
        return {}


def prepare(dist):
    new = pages(dist)
    old = live_manifest()
    changed = [u for u, h in new.items() if old.get(u) != h] + [u for u in old if u not in new]
    with open(os.path.join(dist, 'indexnow.json'), 'w') as f:
        json.dump(new, f, indent=0)
    with open(CHANGES, 'w') as f:
        json.dump(changed, f)
    print(f'{len(new)} pages, {len(changed)} new, changed or removed')
    for u in changed[:20]:
        print('  ', u)


def submit(dist):
    changed = json.load(open(CHANGES)) if os.path.exists(CHANGES) else []
    if not changed:
        print('nothing changed; nothing to submit')
        return
    keys = [os.path.basename(p)[:-4] for p in glob.glob(os.path.join(dist, '*.txt')) if re.fullmatch(r'[0-9a-f]{32}', os.path.basename(p)[:-4])]
    if len(keys) != 1:
        print(f'expected one IndexNow key file in dist/, found {len(keys)}; not submitting')
        return
    body = json.dumps({'host': SITE.split('//')[1], 'key': keys[0], 'keyLocation': f'{SITE}/{keys[0]}.txt', 'urlList': changed[:10000]}).encode()
    req = urllib.request.Request(ENDPOINT, data=body, method='POST', headers={'Content-Type': 'application/json; charset=utf-8', **UA})
    try:
        with urllib.request.urlopen(req, timeout=30) as r:
            # 200: received. 202: received, key check pending (normal for a new key).
            print(f'IndexNow: {r.status} for {len(changed)} URLs')
    except urllib.error.HTTPError as e:
        # 400 bad format, 403 key not valid, 422 URL not on this host, 429 too many requests.
        print(f'::warning::IndexNow answered {e.code}: {e.read().decode(errors="replace")[:200]}')
    except Exception as e:
        print(f'::warning::IndexNow not reached: {e}')


if __name__ == '__main__':
    if sys.argv[1:2] == ['prepare']:
        prepare(sys.argv[2] if len(sys.argv) > 2 else 'dist')
    elif sys.argv[1:2] == ['submit']:
        submit(sys.argv[2] if len(sys.argv) > 2 else 'dist')
    else:
        sys.exit(__doc__)
