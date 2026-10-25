#!/bin/bash
# build one C# file the way the lessons do: inside the image's restored project
set -e
rm -rf /tmp/cs && cp -r /opt/templates/csharp /tmp/cs && cp "$1" /tmp/cs/Program.cs
cd /tmp/cs && dotnet build --no-restore -v q -nologo -clp:NoSummary
dotnet bin/Debug/net10.0/Program.dll
