import os
import tempfile
from pathlib import Path

try:
    Path("/no/such/file.txt").read_text()
except OSError as e:
    print(type(e).__name__ + ":", e)

path = Path(tempfile.gettempdir()) / "lr-fio-py.txt"
path.write_bytes(b"bad \xff here\n")
try:
    path.read_text()
except UnicodeDecodeError as e:
    print(type(e).__name__ + ":", e, "| an OSError:", isinstance(e, OSError))

# text mode translates line endings on the way in; Rust's read_to_string does not
path.write_bytes(b"one\r\ntwo\r\n")
with open(path, "rb") as f:
    print("bytes:", f.read())
with open(path) as f:
    print("for line in f:", list(f))  # each line keeps its \n
with open(path, newline="") as f:
    print("newline='':", repr(f.read()))
print("suffix of a.tar.gz:", Path("a.tar.gz").suffix)  # Rust's extension() has no dot


def write_calls():
    with open("/proc/self/io") as f:
        return int(next(l for l in f if l.startswith("syscw:")).split()[1])


# open() buffers; a Rust File does not
with open(path, "w") as f:
    before = write_calls()
    for i in range(1000):
        f.write(f"line {i}\n")
    during = write_calls() - before
print("1000 f.write calls:", during, "write calls before close,", write_calls() - before, "after")
os.remove(path)

print("Path('/data') / '/etc/hosts':", Path("/data") / "/etc/hosts")
