import os
import subprocess
import sys
import time
from datetime import timedelta

print("len(sys.argv):", len(sys.argv), "(the script name is argv[0])")

# a list of arguments: no shell; a failing program is a return code unless check=True
out = subprocess.run(["echo", "a  b", "$HOME", "one; echo two"], capture_output=True, text=True)
print("output:", repr(out.stdout))
print("false returncode:", subprocess.run(["false"]).returncode)
try:
    subprocess.run(["false"], check=True)
except subprocess.CalledProcessError as e:
    print("check=True raises:", e)

# input= is written to the child's stdin and the pipe is closed for you
sorted_out = subprocess.run(["sort"], input="pear\napple\n", capture_output=True, text=True)
print("sort printed:", repr(sorted_out.stdout))

# a missing variable
try:
    os.environ["RBC_MISSING"]
except KeyError as e:
    print("os.environ[...] raises KeyError:", e)
print("os.environ.get:", os.environ.get("RBC_MISSING"))

# monotonic vs wall clock; a timedelta is signed
start = time.monotonic()
time.sleep(0.02)
print("monotonic elapsed >= 20ms:", time.monotonic() - start >= 0.02)
print("1s - 2s:", timedelta(seconds=1) - timedelta(seconds=2))

# sys.exit raises SystemExit, so finally and with blocks still run; os._exit does not
for call in ["sys.exit(0)", "os._exit(0)"]:
    code = f"import os, sys\ntry:\n    print('calling {call}')\n    {call}\nfinally:\n    print('finally ran')"
    print(subprocess.run([sys.executable, "-c", code], capture_output=True, text=True).stdout, end="")
