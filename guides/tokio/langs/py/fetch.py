import asyncio


async def fetch(name, ms):
    try:
        await asyncio.sleep(ms / 1000)
    except asyncio.CancelledError:
        print(f"({name} was cancelled)")
        raise
    return f"{name} done after {ms}ms"


async def main():
    users = asyncio.create_task(fetch("users", 30))
    orders = asyncio.create_task(fetch("orders", 10))
    print(await users)
    print(await orders)

    left, right = await asyncio.gather(fetch("left", 20), fetch("right", 20))
    print(left, "/", right)

    try:
        print(await asyncio.wait_for(fetch("slow", 500), timeout=0.05))
    except TimeoutError:
        print("slow call timed out")


asyncio.run(main())
