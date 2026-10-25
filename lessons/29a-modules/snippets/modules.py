import subprocess
import sys
import tempfile
from pathlib import Path

root = Path(tempfile.mkdtemp())
(root / "shop.py").write_text("RATE = 20\n_hidden = 'internal'\n\n\ndef total(q):\n    return q * 10\n")
(root / "shop_all.py").write_text("__all__ = ['total']\nRATE = 20\n\n\ndef total(q):\n    return q * 10\n")
(root / "extra.py").write_text("print('extra.py ran')\n")
# Two files that import each other, once with `from ... import` and once with `import`.
(root / "ping.py").write_text("from pong import g\n\n\ndef f():\n    return g()\n")
(root / "pong.py").write_text("from ping import f\n\n\ndef g():\n    return 1\n")
(root / "ping2.py").write_text("import pong2\n\n\ndef f():\n    return pong2.g()\n")
(root / "pong2.py").write_text("import ping2\n\n\ndef g():\n    return 1\n")
# A package: __init__.py re-exports a name from a file inside it.
(root / "shopkg").mkdir()
(root / "shopkg" / "__init__.py").write_text("from .rates import rate\n")
(root / "shopkg" / "rates.py").write_text("def rate():\n    return 20\n")

SHOW = "print(sorted(n for n in dir() if n in ('RATE', 'total', '_hidden')))"


def run(label, code):
    print(label)
    out = subprocess.run([sys.executable, "-c", code], cwd=root, capture_output=True, text=True)
    last = (out.stdout + out.stderr).strip().splitlines()[-1]
    # The error text also names a temporary folder; keep the part that is the same every time.
    print(last.split(" (consider")[0])


run("1. reach a _private name through the module", "import shop; print(shop._hidden)")
run("2. star-import: which names arrive", "from shop import *; " + SHOW)
run("3. star-import from a module with __all__", "from shop_all import *; " + SHOW)
run("4. import runs the file", "import extra")
run("5. name a module-level name that __all__ left out", "from shop_all import RATE; print(RATE)")
run("6. a cycle with `from ... import`", "import ping")
run("7. the same cycle with `import`", "import ping2; print(ping2.f())")
run("8. a package re-exports a name in __init__.py", "from shopkg import rate; print(rate())")
