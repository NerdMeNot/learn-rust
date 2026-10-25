#!/bin/bash
# build one C# file the way the lessons do (inside the image's restored project), and run it
rm -rf /tmp/cs && cp -r /opt/templates/csharp /tmp/cs && cp "$1" /tmp/cs/Program.cs
cd /tmp/cs
if ! out=$(dotnet build --no-restore -v q -nologo -clp:NoSummary 2>&1); then
  echo "$out" | grep ' error ' | sed -e 's# \[/tmp/cs/[^]]*\]##' -e 's#/tmp/cs/##' | sort -u
  exit 1
fi
dotnet bin/Debug/net10.0/Program.dll
