import asyncio

log = []


async def heartbeat():
    for i in range(1, 4):
        log.append(f"tick {i}")
        await asyncio.sleep(0)  # let the event loop run something else


async def main():
    hb = asyncio.create_task(heartbeat())
    await asyncio.sleep(0)  # let the heartbeat start

    # CPU-bound work, called straight from a coroutine
    total = sum(n % 7 for n in range(1_000_000))
    log.append(f"sum done, total {total}")

    await hb
    print("\n".join(log))


asyncio.run(main())
