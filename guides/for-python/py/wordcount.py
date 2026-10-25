from collections import Counter


def word_count(text: str) -> Counter[str]:
    """Count how often each word appears, ignoring case."""
    return Counter(word.lower() for word in text.split())


if __name__ == "__main__":
    for word, n in sorted(word_count("the cat and The hat").items()):
        print(f"{word}: {n}")
