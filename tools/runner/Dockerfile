# The one place every program on the site is compiled and run.
#
# All nine toolchains, pinned. The site prints these versions (from `versions.json`,
# which `run.py versions` writes), so a reader knows exactly what produced each output.
# Linux on purpose: the C and C++ examples rely on GCC's AddressSanitizer and
# ThreadSanitizer, and CI runs Linux too, so a local run and a CI run agree.
#
# Bumping a version here changes outputs. Rebuild, re-run `bun run extract`, review the diff.
FROM debian:trixie-slim@sha256:62bd423aeee912639ca8ee681c2b762cb3e25137e0a92b20be455898a9f034ad

ARG RUST=1.99.1
ARG GO=1.27.3
ARG JDK=25.0.5+8
ARG KOTLIN=2.4.21
ARG NODE=24.22.1
ARG TYPESCRIPT=7.0.2
ARG DOTNET=10.0.402

ENV DEBIAN_FRONTEND=noninteractive \
    LANG=C.UTF-8 \
    RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo \
    JAVA_HOME=/opt/jdk \
    DOTNET_ROOT=/opt/dotnet DOTNET_CLI_TELEMETRY_OPTOUT=1 DOTNET_NOLOGO=1 \
    DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1 DOTNET_SYSTEM_GLOBALIZATION_INVARIANT=1 \
    CARGO_TERM_COLOR=never NO_COLOR=1 \
    PATH=/opt/cargo/bin:/opt/go/bin:/opt/jdk/bin:/opt/kotlinc/bin:/opt/node/bin:/opt/dotnet:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN apt-get update && apt-get install -y --no-install-recommends \
      ca-certificates curl xz-utils unzip \
      gcc g++ libc6-dev python3 \
      libgcc-s1 libstdc++6 zlib1g libssl3t64 \
    && rm -rf /var/lib/apt/lists/*

# Rust: exactly the toolchain in rust-toolchain.toml.
RUN curl -sSf --retry 8 --retry-all-errors --speed-limit 20000 --speed-time 30 https://sh.rustup.rs | RUSTUP_MAX_RETRIES=10 sh -s -- -y --no-modify-path --profile minimal --default-toolchain ${RUST} \
    && rustc --version

# Go
RUN curl -sSfL --retry 8 --retry-all-errors --speed-limit 20000 --speed-time 30 -o /tmp/go.tgz https://go.dev/dl/go${GO}.linux-$(dpkg --print-architecture).tar.gz \
    && tar -xzf /tmp/go.tgz -C /opt && rm /tmp/go.tgz

# Java (Temurin) and Kotlin
RUN set -eu; arch=$([ "$(dpkg --print-architecture)" = arm64 ] && echo aarch64 || echo x64); \
    v=$(echo ${JDK} | sed 's/+/_/'); \
    mkdir /opt/jdk; \
    curl -sSfL --retry 8 --retry-all-errors --speed-limit 20000 --speed-time 30 -o /tmp/jdk.tgz "https://github.com/adoptium/temurin25-binaries/releases/download/jdk-${JDK}/OpenJDK25U-jdk_${arch}_linux_hotspot_${v}.tar.gz"; \
    tar -xzf /tmp/jdk.tgz -C /opt/jdk --strip-components=1; rm /tmp/jdk.tgz

RUN set -eu; curl -sSfL --retry 8 --retry-all-errors --speed-limit 20000 --speed-time 30 -o /tmp/k.zip https://github.com/JetBrains/kotlin/releases/download/v${KOTLIN}/kotlin-compiler-${KOTLIN}.zip; \
    unzip -q /tmp/k.zip -d /opt; rm /tmp/k.zip

# Node and TypeScript
RUN set -eu; arch=$([ "$(dpkg --print-architecture)" = arm64 ] && echo arm64 || echo x64); \
    mkdir /opt/node; \
    curl -sSfL --retry 8 --retry-all-errors --speed-limit 20000 --speed-time 30 -o /tmp/node.txz https://nodejs.org/dist/v${NODE}/node-v${NODE}-linux-${arch}.tar.xz; \
    tar -xJf /tmp/node.txz -C /opt/node --strip-components=1; rm /tmp/node.txz; \
    npm install -g --no-fund --no-audit typescript@${TYPESCRIPT} @types/node@24

# .NET, plus a console project restored once so each C# program builds offline.
RUN curl -sSfL --retry 8 --retry-all-errors --speed-limit 20000 --speed-time 30 -o /tmp/dotnet-install.sh https://dot.net/v1/dotnet-install.sh && bash /tmp/dotnet-install.sh --version ${DOTNET} --install-dir /opt/dotnet \
    && dotnet new console -o /opt/templates/csharp -n Program \
    && dotnet build /opt/templates/csharp -v q

# Warm the slow compilers so the first real job is not also paying for this.
RUN mkdir -p /opt/templates/rust/src \
    && printf '[package]\nname = "main"\nversion = "0.1.0"\nedition = "2024"\n\n[dependencies]\n' > /opt/templates/rust/Cargo.toml \
    && echo 'fn main() {}' > /opt/templates/rust/src/main.rs \
    && cargo build --manifest-path /opt/templates/rust/Cargo.toml -q \
    && echo 'fun main() {}' > /tmp/w.kt && kotlinc /tmp/w.kt -d /tmp/w.jar 2>/dev/null; rm -rf /tmp/w.*

# rustfmt and clippy, which the first-project page runs (rust-toolchain.toml lists them too).
RUN rustup component add rustfmt clippy

# Tools the guide pages use: protoc for the gRPC guide, sqlite3 for the database one.
# A late layer on purpose, so adding a tool does not re-download every toolchain.
RUN apt-get update && apt-get install -y --no-install-recommends protobuf-compiler sqlite3 \
    && rm -rf /var/lib/apt/lists/*

COPY run.py /opt/runner/run.py
WORKDIR /work
ENTRYPOINT ["python3", "/opt/runner/run.py"]
