UNITS = {"s": 1, "m": 60, "h": 3600}


def seconds(text):
    """Read text like "1h30m" or "90s" and return the number of seconds."""
    if not text:
        raise ValueError("empty")
    total = 0
    digits = ""
    for c in text:
        if c.isdigit():
            digits += c
            continue
        if c not in UNITS:
            raise ValueError(f"unknown unit {c!r}")
        if not digits:
            raise ValueError("no number")
        total += int(digits) * UNITS[c]
        digits = ""
    if digits:
        raise ValueError("missing unit")
    return total
