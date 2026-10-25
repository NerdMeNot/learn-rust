import subprocess

# timeout=: when it passes, subprocess.run kills the child and raises
try:
    subprocess.run(["sleep", "60"], timeout=0.2)
except subprocess.TimeoutExpired as e:
    print("TimeoutExpired:", e)

# a child ended by a signal has a negative returncode
print("kill -9 returncode:", subprocess.run(["sh", "-c", "kill -9 $$"]).returncode)
