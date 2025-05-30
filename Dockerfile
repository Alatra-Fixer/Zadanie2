# ===== Builder Stage =====
FROM rust:latest AS builder
WORKDIR /app

RUN apt-get update && \
    apt-get install -y musl-tools && \
    rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

# Dummy build dla cache’owania zależności
RUN mkdir src && \
    echo 'fn main() { println!("build deps"); }' > src/main.rs && \
    rustup target add x86_64-unknown-linux-musl && \
    CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl

# Usuń dummy-owy kod i wklej prawdziwy kod źródłowy
RUN rm -rf src
COPY src ./src

# Usuń katalog target, aby wymusić pełną rekompilację prawdziwego kodu
RUN rm -rf target && \
    CC=x86_64-linux-musl-gcc cargo build --release --target x86_64-unknown-linux-musl && \
    strip target/x86_64-unknown-linux-musl/release/pogoda

# ===== Final Stage =====
FROM alpine:3.17 AS runtime
RUN apk add --no-cache wget

LABEL org.opencontainers.authors="Rafał Oleszczak"
LABEL org.opencontainers.source="https://github.com/rafal-oleszczak/pogoda"

WORKDIR /app
EXPOSE 8080

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pogoda /usr/local/bin/pogoda

HEALTHCHECK --interval=30s --timeout=5s --start-period=15s \
  CMD wget -qO- http://localhost:8080/ >/dev/null || exit 1

ENTRYPOINT ["/usr/local/bin/pogoda"]
