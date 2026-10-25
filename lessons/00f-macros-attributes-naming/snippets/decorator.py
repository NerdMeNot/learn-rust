def shout(func):
    print("decorating", func.__name__)

    def wrapper():
        print("before")
        func()
        print("after")

    return wrapper


@shout
def greet():
    print("hello")


greet()
