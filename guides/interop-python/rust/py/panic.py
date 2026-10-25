import wordstats

try:
    wordstats.explode("seven")
except Exception:
    print("caught by except Exception")
except BaseException as e:
    print("only BaseException caught it:", type(e).__name__)
    print(type(e).__mro__)
    print(e)
