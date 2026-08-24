FROM rust:1.97.0-bookworm AS builder

ARG AGAVE_VERSION=v4.1.2

RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
        build-essential \
        clang \
        cmake \
        curl \
        libclang-dev \
        libprotobuf-dev \
        libssl-dev \
        libudev-dev \
        llvm \
        pkg-config \
        protobuf-compiler \
        zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/agave

RUN curl -fsSL "https://github.com/anza-xyz/agave/archive/refs/tags/${AGAVE_VERSION}.tar.gz" \
    | tar --extract --gzip --strip-components=1

RUN cargo build --locked --release --bin solana-test-validator \
    && install --directory /opt/agave/bin \
    && install --mode=755 target/release/solana-test-validator /opt/agave/bin/solana-test-validator \
    && rm --recursive --force target

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
        ca-certificates \
        curl \
        libssl3 \
        libudev1 \
        zlib1g \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /opt/agave/bin/solana-test-validator /usr/local/bin/solana-test-validator

EXPOSE 8899 8900 9900

CMD ["sh", "-c", "HOST=$(hostname -i); exec solana-test-validator --ledger /var/lib/solana/ledger --bind-address ${HOST%% *} --rpc-port 8899 --faucet-port 9900"]
