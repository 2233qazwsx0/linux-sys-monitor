FROM rust:1.75-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release && strip target/release/linux-system-monitor

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/linux-system-monitor .
COPY --from=builder /app/src/frontend_dist ./frontend_dist

EXPOSE 8080

ENV RUST_LOG=info

CMD ["./linux-system-monitor"]
