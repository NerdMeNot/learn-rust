from wordcount import word_count


def test_counts_ignoring_case():
    counts = word_count("the cat and The hat")
    assert counts["the"] == 2
    assert counts["cat"] == 1
