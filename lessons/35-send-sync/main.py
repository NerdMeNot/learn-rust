import json
from concurrent.futures import ThreadPoolExecutor

words = ["alpha", "beta"]

with ThreadPoolExecutor() as pool:
    # hand the list to a pool thread and get it back as the result
    back = pool.submit(lambda: words).result()
    print("moved there and back:", json.dumps(back))

    # two threads read the same list at once; nothing checks that this is safe
    a = pool.submit(lambda: len(words[0]))
    b = pool.submit(lambda: len(words[1]))
    print(f"lengths read in parallel: {a.result()} and {b.result()}")
