import argparse
import re
import sys
from collections import Counter

parser = argparse.ArgumentParser(prog="wordcount")
parser.add_argument("file", nargs="?", default="-")
parser.add_argument("--top", type=int, default=10)
args = parser.parse_args()

text = sys.stdin.read() if args.file == "-" else open(args.file, encoding="utf-8").read()
counts = Counter(w.lower() for w in re.findall(r"[^\W\d_']+(?:'+[^\W\d_']+)*", text))
rows = sorted(counts.items(), key=lambda kv: (-kv[1], kv[0]))[: args.top]
width = max((len(w) for w, _ in rows), default=0)
print(f"{sum(counts.values())} words, {len(counts)} distinct")
for rank, (word, n) in enumerate(rows, 1):
    print(f"{rank:>3}  {word:<{width}}  {n:>5}")
