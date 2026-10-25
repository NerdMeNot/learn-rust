import subprocess
import sys

code = "assert 1 + 1 == 3\nprint('assert did not stop the program')"
result = subprocess.run([sys.executable, "-O", "-c", code], capture_output=True, text=True)
print(result.stdout, end="")
