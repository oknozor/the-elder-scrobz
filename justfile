#!/usr/bin/env just --justfile

release:
  cargo build --release

lint:
  cargo clippy && cd frontend && biome check

bin:
  cargo run --bin bin -- arg1

database-reset:
  sqlx database drop -y
  sqlx database create
  sqlx migrate run --source crates/elder-scrobz-db/migrations

generate-mock-data:
  assets/init_mock.sh assets/listens.jsonl assets/mock.jsonl
