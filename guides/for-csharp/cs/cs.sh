#!/bin/bash
# Build and run one C# file the way the lessons do: inside the image's restored console
# project (net10.0, nullable and implicit usings on). Prints the compiler's warnings and
# errors for Program.cs, then runs the program.
d=$(mktemp -d) && cp -r /opt/templates/csharp "$d/app" && cp "$1" "$d/app/Program.cs" && cd "$d/app" || exit 2
if ! dotnet build --no-restore -v q -nologo -clp:NoSummary > build.log 2>&1; then
  grep -E '^Program\.cs\([0-9]+,[0-9]+\): (error|warning)' <(sed "s#$d/app/##" build.log) | sed -E 's# \[[^]]*\]$##' | sort -u
  exit 1
fi
grep -E '^Program\.cs\([0-9]+,[0-9]+\): warning' <(sed "s#$d/app/##" build.log) | sed -E 's# \[[^]]*\]$##' | sort -u
dotnet bin/Debug/net10.0/Program.dll
