FROM rust:1.85.1-slim AS builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN cargo install sqlx-cli --no-default-features --features postgres

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y libssl-dev ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/elder-scrobz-api /app/elder-scrobz-api

COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

COPY --from=builder /app/crates/elder-scrobz-db/migrations /app/migrations

COPY config.toml /app/
COPY .env /app/

COPY entrypoint.sh /app/
RUN chmod +x /app/entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]

CMD ["/app/elder-scrobz-api"]