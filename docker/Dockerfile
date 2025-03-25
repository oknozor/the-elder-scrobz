FROM alpine as arm-builder
COPY ./target/armv7-unknown-linux-musleabihf/release/scrobz /scrobz

FROM alpine as arm64-builder
COPY ./target/aarch64-unknown-linux-musl/release/scrobz /scrobz

FROM alpine as amd64-builder
COPY ./target/x86_64-unknown-linux-musl/release/scrobz /scrobz

FROM ${targetarch}-builder AS builder

FROM alpine
LABEL maintainer="Paul Delafosse <paul.delafosse@protonmail.com>"

RUN addgroup -S scrobz && adduser -S scrobz -G scrobz
USER scrobz

COPY --from=builder /scrobz /usr/bin/scrobz

EXPOSE 3000

COPY ./docker/entrypoint.sh /entrypoint.sh

CMD ["scrobz"]
ENTRYPOINT ["/entrypoint.sh"]
