import tempfile
from pathlib import Path

path = Path(tempfile.gettempdir()) / "rbc-hello.txt"
path.write_text("hello")

with open(path) as f:
    text = f.read()
print(text)
path.unlink()
