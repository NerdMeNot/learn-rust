import os
import subprocess
import sys
import time

# argv: the first one is the script itself
print(f"{len(sys.argv) - 1} arg(s) after the program name")

# environ.get returns None (or your default) when the variable isn't set
port = os.environ.get("RBC_PORT", "8080")
print(f"port {port}")
missing = os.environ.get("RBC_MISSING")
print(f"RBC_MISSING={missing}" if missing is not None else "RBC_MISSING: not set")

# monotonic clock for measuring, in nanoseconds
start = time.monotonic_ns()
time.sleep(0.020)
took_ms = (time.monotonic_ns() - start) / 1_000_000
print(f"slept at least 20ms: {took_ms >= 20}")

# run another program and capture what it prints
out = subprocess.run(["echo", "hello from echo"], capture_output=True, text=True)
print(out.stdout, end="")
print(f"exit ok: {out.returncode == 0}")
