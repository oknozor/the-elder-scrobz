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

android-build:
    export ANDROID_HOME=$HOME/Android/Sdk
    export PATH=$PATH:$ANDROID_HOME/emulator
    export PATH=$PATH:$ANDROID_HOME/tools
    export PATH=$PATH:$ANDROID_HOME/tools/bin
    export PATH=$PATH:$ANDROID_HOME/platform-tools
    export NDK_HOME=/opt/android-ndk
    cd frontend
    npx tauri android build --verbose
