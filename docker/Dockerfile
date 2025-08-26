# syntax=docker/dockerfile:1

ARG TARGETARCH

FROM alpine:3.22.1 AS arm-builder
COPY ./target/armv7-unknown-linux-musleabihf/release/scrobz /scrobz

FROM alpine:3.22.1 AS arm64-builder
COPY ./target/aarch64-unknown-linux-gnu/release/scrobz /scrobz

FROM alpine:3.22.1 AS amd64-builder
COPY ./target/x86_64-unknown-linux-musl/release/scrobz /scrobz

FROM ${TARGETARCH}-builder AS builder

FROM alpine:3.22.1
LABEL maintainer="Paul Delafosse <paul.delafosse@protonmail.com>"

USER root
WORKDIR /app

RUN apk add --no-cache bash \
    mkdir -p /app/frontend \
    mkdir -p /app/backend \
    && chown -R root:root /app && chmod -R 755 /app

COPY --from=builder /scrobz /app/backend
COPY ./frontend/dist/ /app/frontend/
COPY ./docker/entrypoint.sh /entrypoint.sh

USER nobody:nogroup

ENTRYPOINT ["/usr/bin/catatonit", "--", "/entrypoint.sh"]
