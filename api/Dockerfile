FROM rust:latest AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev pkg-config

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/stock-management-api /usr/local/bin/stock-management-api

CMD ["stock-management-api"]
