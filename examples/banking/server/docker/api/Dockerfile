FROM rust:stable-bookworm AS base

WORKDIR /workspace

FROM base AS dev

RUN cargo install cargo-watch

EXPOSE 8080

CMD ["cargo", "watch", "-w", "./examples/banking/server/crates", "-w", "./examples/banking/server/servers/api", "-x", "run --manifest-path examples/banking/server/Cargo.toml -p api"]

FROM base AS builder

COPY . .
RUN cargo build --manifest-path examples/banking/server/Cargo.toml --release -p api

FROM debian:bookworm-slim AS prod

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /workspace/examples/banking/server/target/release/api /usr/local/bin/api

EXPOSE 8080

CMD ["api"]
