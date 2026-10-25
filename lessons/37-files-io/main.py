import tempfile
from pathlib import Path

path = Path(tempfile.gettempdir()) / "rbc-notes.txt"

# write a whole file at once (creates it, or empties it first)
path.write_text("milk\neggs\n")

# open for appending and add a line
with open(path, "a") as f:
    f.write("bread\n")

# read the whole file into one str
text = path.read_text()
print(f"{len(text)} chars")

# read line by line, without loading the whole file
with open(path) as f:  # `with` closes the file at the end of the block
    for i, line in enumerate(f, start=1):
        print(f"{i}: {line.rstrip()}")

print(f"file name: {path.name!r}")

# a missing file raises an exception you can catch
try:
    Path("/no/such/file.txt").read_text()
except FileNotFoundError as e:
    print(f"error: {e.strerror} (errno {e.errno})")

path.unlink()
