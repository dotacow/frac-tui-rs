FROM rust:1.82-slim-bookworm as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src

# touch as a hack to reupdate timestamp and force rebuild.
RUN touch src/main.rs && cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

#ca-certificates edge case
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/frac-tui-rs /usr/local/bin/frac-tui-rs

ENTRYPOINT ["frac-tui-rs"]