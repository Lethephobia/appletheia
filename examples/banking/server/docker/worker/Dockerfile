FROM rust:stable-bookworm AS base

WORKDIR /workspace

FROM base AS dev

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-w", "./examples/banking/server/crates", "-w", "./examples/banking/server/servers/worker", "-x", "run --manifest-path examples/banking/server/Cargo.toml -p worker"]

FROM base AS builder

COPY . .
RUN cargo build --manifest-path examples/banking/server/Cargo.toml --release -p worker

FROM debian:bookworm-slim AS prod

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /workspace/examples/banking/server/target/release/worker /usr/local/bin/worker

CMD ["worker"]
