import asyncio
import multiprocessing as mp
from concurrent.futures import ThreadPoolExecutor
import time


def child(q) -> None:
    data = q.get()
    data.append(99)  # changes the child's copy only


async def aio() -> None:
    q: asyncio.Queue[int] = asyncio.Queue()
    q.put_nowait(1)
    print("asyncio.Queue.get:", await q.get())


if __name__ == "__main__":
    # multiprocessing.Queue pickles what you put: the receiver gets a copy
    q = mp.Queue()
    data = [1, 2]
    q.put(data)
    p = mp.Process(target=child, args=(q,))
    p.start()
    p.join()
    print("multiprocessing.Queue: the sender's list is still", data)

    # ThreadPoolExecutor.map returns results in input order, whichever finishes first
    def slow_first(x: int) -> int:
        time.sleep(0.05 if x == 3 else 0)
        return x * x

    with ThreadPoolExecutor() as pool:
        print("ThreadPoolExecutor.map:", list(pool.map(slow_first, [3, 1, 2])))

    asyncio.run(aio())
