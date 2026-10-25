import asyncio


async def fetch_name():
    print("  fetch_name body runs")
    return "ferris"


async def leaf():
    pass  # finishes at once, so awaiting it does not suspend main


async def main():
    print("calling fetch_name()")
    coroutine = fetch_name()
    print("called it:", type(coroutine).__name__, "- the body has not run")
    print("name:", await coroutine)

    # create_task hands the coroutine to the event loop, which starts it when main next suspends
    task = asyncio.create_task(fetch_name())
    print("task created; still has not run")
    await leaf()
    print("after awaiting leaf(), which never waits: done =", task.done())
    await asyncio.sleep(0)
    print("after asyncio.sleep(0), which does suspend: done =", task.done())
    await task

    # a coroutine that is never awaited is reported when it is garbage collected
    fetch_name()
    print("called without await")


asyncio.run(main())
