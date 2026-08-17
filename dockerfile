# Multi-stage builds optimize continuous compilation speeds
FROM rust:1.75-slim AS backend-builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential && rm -rf /var/lib/apt/lists/*
COPY services/backend-rust .
RUN cargo build --release

FROM python:3.11-slim
WORKDIR /app

# Install explicit system dependencies for Python data engines
RUN pip install --no-cache-dir axum reqwest reportlab requests

# Port configuration mappings
EXPOSE 8080
COPY --from=backend-builder /app/target/release/soma_backend /app/soma_backend

CMD ["/app/soma_backend"]
