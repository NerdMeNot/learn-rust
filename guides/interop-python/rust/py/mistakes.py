import wordstats


def attempt(label, call):
    try:
        print(label, "->", call())
    except Exception as e:
        print(label, "->", type(e).__name__ + ":", e)


attempt("add(2**63, 1)", lambda: wordstats.add(2**63, 1))
attempt("add('1', 2)", lambda: wordstats.add("1", 2))
attempt("total('123')", lambda: wordstats.total("123"))
attempt("total([1, 'x'])", lambda: wordstats.total([1, "x"]))
attempt("shout('\\ud800')", lambda: wordstats.shout("\ud800"))
attempt("parse_pair('3, 4')", lambda: wordstats.parse_pair("3, 4"))
attempt("parse_pair('34')", lambda: wordstats.parse_pair("34"))
attempt("parse_pair('3,x')", lambda: wordstats.parse_pair("3,x"))
