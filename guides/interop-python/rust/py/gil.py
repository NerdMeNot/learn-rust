import threading
import time

import wordstats


def helper():
    time.sleep(0.1)
    wordstats.set_flag()


def run(wait):
    wordstats.reset_flag()
    t = threading.Thread(target=helper)
    t.start()
    got_it = wait(1000)
    t.join()
    return got_it


print("wait_holding_gil:", run(wordstats.wait_holding_gil))
print("wait_detached:   ", run(wordstats.wait_detached))
