#!/bin/bash
# The runner image has Python but no pip, so fetch pip, then install the pinned
# packages into a scratch folder (nothing is installed system-wide).
set -e
cd "$(dirname "$0")"
{
  curl -sS https://bootstrap.pypa.io/get-pip.py -o /tmp/get-pip.py
  python3 /tmp/get-pip.py -q --break-system-packages --target /tmp/pylibs
  PYTHONPATH=/tmp/pylibs python3 -m pip install -q --target /tmp/pylibs -r requirements.txt
} > /tmp/pip.log 2>&1 || { cat /tmp/pip.log; exit 1; }
echo installed
