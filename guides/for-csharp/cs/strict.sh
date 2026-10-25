#!/bin/bash
# Like cs.sh, but the csproj also has <WarningsAsErrors>nullable</WarningsAsErrors>.
d=$(mktemp -d) && cp -r /opt/templates/csharp "$d/app" && cp "$1" "$d/app/Program.cs" && cd "$d/app" || exit 2
sed -i 's#<Nullable>enable</Nullable>#<Nullable>enable</Nullable><WarningsAsErrors>nullable</WarningsAsErrors>#' ./*.csproj
if ! dotnet build --no-restore -v q -nologo -clp:NoSummary > build.log 2>&1; then
  grep -E '^Program\.cs\([0-9]+,[0-9]+\): (error|warning)' <(sed "s#$d/app/##" build.log) | sed -E 's# \[[^]]*\]$##' | sort -u
  exit 1
fi
dotnet bin/Debug/net10.0/Program.dll
