import wordstats

print(wordstats.add(2, 3))
print(wordstats.shout("héllo, wörld"))
print(wordstats.word_counts("the cat saw the other cat and The dog"))
print(wordstats.total([1.5, 2.5, 3.0]))
print(wordstats.total([1, 2, 3]))
print(wordstats.add.__doc__)
print(wordstats.word_counts("")) 
print(type(wordstats.word_counts("a")))
