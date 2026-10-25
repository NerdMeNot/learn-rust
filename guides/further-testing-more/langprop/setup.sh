#!/bin/bash
# The runner image has Python and Node but no pip and no node_modules. Fetch pip, install the
# pinned hypothesis into a scratch folder, and install the pinned fast-check next to the
# TypeScript test. Nothing is installed system-wide. Needs network.
set -e
cd "$(dirname "$0")"
{
  curl -sS https://bootstrap.pypa.io/get-pip.py -o /tmp/get-pip.py
  python3 /tmp/get-pip.py -q --break-system-packages --target /tmp/pylibs
  PYTHONPATH=/tmp/pylibs python3 -m pip install -q --target /tmp/pylibs -r requirements.txt
  (cd ts && npm install --no-audit --no-fund --silent)
} > /tmp/setup.log 2>&1 || { cat /tmp/setup.log; exit 1; }
echo installed
