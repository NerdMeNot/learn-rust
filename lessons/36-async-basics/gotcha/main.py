import asyncio


async def load_count():
    return 3


async def report():
    n = load_count()
    return n + 1


print("count:", asyncio.run(report()))
