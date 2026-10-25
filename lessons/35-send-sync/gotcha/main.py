import json
import threading


class Job:
    def __init__(self, name, log):
        self.name = name
        self.log = log


def run(job):
    job.log.append(job.name)


log = []
job = Job("resize", log)
t = threading.Thread(target=run, args=(job,))
t.start()
t.join()
print("log:", json.dumps(log))
