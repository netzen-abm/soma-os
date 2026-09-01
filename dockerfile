# Build the Rust backend in an isolated stage.
FROM rust:1.75-slim AS backend-builder

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY services/backend-rust ./services/backend-rust

RUN cargo build \
    --manifest-path services/backend-rust/Cargo.toml \
    --release

# Runtime contains only the compiled backend.
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

COPY --from=backend-builder \
    /app/services/backend-rust/target/release/soma_backend \
    /app/soma_backend

CMD ["/app/soma_backend"]
