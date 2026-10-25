import asyncio


async def task():
    print("task body runs")


print("start")
coroutine = task()  # a coroutine object: the body has not run
print("task created")
asyncio.run(coroutine)
print("end")
