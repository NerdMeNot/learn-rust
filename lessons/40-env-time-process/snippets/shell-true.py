import subprocess

user = "world; echo injected"
# shell=True hands the whole string to /bin/sh: the semicolon starts a second command
subprocess.run(f"echo hello {user}", shell=True)
# a list of arguments never reaches a shell
subprocess.run(["echo", "hello", user])
