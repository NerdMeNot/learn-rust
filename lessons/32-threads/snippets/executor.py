from concurrent.futures import ThreadPoolExecutor


def parse(text):
    return int(text)


with ThreadPoolExecutor() as pool:
    good = pool.submit(parse, "17")
    bad = pool.submit(parse, "seventeen")
    print("good.result():", good.result())
    try:
        bad.result()  # the worker's exception is raised here
    except ValueError as e:
        print("bad.result() raised ValueError:", e)
