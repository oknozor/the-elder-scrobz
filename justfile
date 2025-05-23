#!/usr/bin/env just --justfile

release:
  cargo build --release    

lint:
  cargo clippy

bin:
  cargo run --bin bin -- arg1

database-reset:
  sqlx database drop -y
  sqlx database create
  sqlx migrate run --source crates/elder-scrobz-db/migrations