from concurrent.futures import ThreadPoolExecutor

names = ["a", "b", "c"]


def work() -> int:
    for n in names:
        print(f"worker sees {n}")
    return len(names)  # a thread can hand back a value through a future


with ThreadPoolExecutor() as pool:
    future = pool.submit(work)
    count = future.result()  # wait for the thread
print(f"worker counted {count}")
