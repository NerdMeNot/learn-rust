import wordstats

try:
    wordstats.count_words("   ")
except wordstats.EmptyText as e:
    print(type(e).__name__ + ":", e)
    print("is a ValueError:", isinstance(e, ValueError))

print(wordstats.count_words("one two three"))
