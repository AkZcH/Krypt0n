# ---------- Build stage ----------
FROM rust:1.88 AS builder

WORKDIR /app

# Cache dependencies first for faster rebuilds.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Copy the real project and build the release binary.
COPY . .
RUN cargo build --release


# ---------- Runtime stage ----------
FROM debian:bookworm-slim

# Run as a non-root user inside the container.
RUN useradd -m krypton

WORKDIR /home/krypton

COPY --from=builder /app/target/release/krypton /usr/local/bin/krypton

USER krypton

ENTRYPOINT ["krypton"]
