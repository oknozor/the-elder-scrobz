FROM alpine AS arm-builder
COPY ./target/armv7-unknown-linux-musleabihf/release/scrobz /scrobz

FROM alpine AS arm64-builder
COPY ./target/aarch64-unknown-linux-gnu/release/scrobz /scrobz

FROM alpine AS amd64-builder
COPY ./target/x86_64-unknown-linux-musl/release/scrobz /scrobz

FROM ${TARGETARCH}-builder AS builder

FROM alpine
LABEL maintainer="Paul Delafosse <paul.delafosse@protonmail.com>"

RUN addgroup -S scrobz && adduser -S scrobz -G scrobz
USER scrobz

COPY --from=builder /scrobz /usr/bin/scrobz

EXPOSE 3000

COPY ./docker/entrypoint.sh /entrypoint.sh

CMD ["scrobz"]
ENTRYPOINT ["/entrypoint.sh"]
