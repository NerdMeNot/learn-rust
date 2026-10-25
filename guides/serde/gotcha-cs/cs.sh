#!/bin/bash
# build one C# file the way the lessons do (the image's restored project), but keep
# the build's own output (timing lines) off the page unless the build fails
set -e
rm -rf /tmp/cs && cp -r /opt/templates/csharp /tmp/cs && cp "$1" /tmp/cs/Program.cs
cd /tmp/cs
dotnet build --no-restore -v q -nologo -clp:NoSummary > /tmp/cs-build.log 2>&1 || { cat /tmp/cs-build.log; exit 1; }
dotnet bin/Debug/net10.0/Program.dll
