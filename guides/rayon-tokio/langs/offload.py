import asyncio
from concurrent.futures import ProcessPoolExecutor


def work() -> int:
    return sum(n % 7 for n in range(1_000_000))


async def main():
    loop = asyncio.get_running_loop()
    with ProcessPoolExecutor(max_workers=2) as pool:
        print("process pool:", await loop.run_in_executor(pool, work))
    print("to_thread:", await asyncio.to_thread(work))


if __name__ == "__main__":
    asyncio.run(main())
