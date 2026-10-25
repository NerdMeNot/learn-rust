import asyncio


async def hello():
    print("hello")


async def main():
    hello()
    print("after the call")
    await hello()


asyncio.run(main())
