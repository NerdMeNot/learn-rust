import sys

DEFAULT_TOP = 10
USAGE = "usage: wordcount [FILE] [--top N]"


class UsageError(Exception):
    pass


def words(text):
    cleaned = "".join(c if c.isalpha() or c == "'" else " " for c in text)
    stripped = (w.strip("'") for w in cleaned.split())
    return [w for w in stripped if w]


def count_words(text):
    counts = {}
    for word in words(text):
        word = word.lower()
        counts[word] = counts.get(word, 0) + 1
    return counts


def top_n(counts, n):
    ranked = sorted(counts.items(), key=lambda item: (-item[1], item[0]))
    return ranked[:n]


def parse_args(args):
    path, top = None, DEFAULT_TOP
    i = 0
    while i < len(args):
        arg = args[i]
        i += 1
        if arg == "--top":
            if i == len(args):
                raise UsageError("--top needs a number")
            value = args[i]
            i += 1
            try:
                top = int(value)
            except ValueError:
                raise UsageError(f"invalid value {value!r} for --top")
            if top < 1:
                raise UsageError("--top must be at least 1")
        elif arg.startswith("-") and arg != "-":
            raise UsageError(f"unknown option {arg}")
        elif path is not None:
            raise UsageError("only one input file is allowed")
        elif arg != "-":
            path = arg
    return path, top


def read_input(path):
    if path is None:
        return sys.stdin.read()
    with open(path, encoding="utf-8") as f:
        return f.read()


def main():
    try:
        path, top = parse_args(sys.argv[1:])
    except UsageError as e:
        print(f"wordcount: {e}\n{USAGE}", file=sys.stderr)
        return 2
    try:
        text = read_input(path)
    except (OSError, UnicodeDecodeError) as e:
        print(f"wordcount: cannot read {path or 'standard input'}: {e}", file=sys.stderr)
        return 1

    counts = count_words(text)
    rows = top_n(counts, top)
    print(f"{sum(counts.values())} words, {len(counts)} distinct")
    width = max((len(word) for word, _ in rows), default=0)
    for rank, (word, count) in enumerate(rows, 1):
        print(f"{rank:>3}  {word:<{width}}  {count:>5}")
    return 0


sys.exit(main())
