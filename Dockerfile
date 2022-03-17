# syntax = docker/dockerfile:experimental
FROM rust:1.59 as builder
RUN apt-get update

WORKDIR /nucleus

COPY . /nucleus

# RUN cargo build --release
RUN --security=insecure mkdir -p /root/.cargo && chmod 777 /root/.cargo && mount -t tmpfs none /root/.cargo && cargo build --release

FROM alpine:3.15

# hadolint ignore=DL3018
RUN apk --no-cache add ca-certificates vim

USER nucleus

COPY --from=builder /nucleus/target/release/nucleus /usr/local/bin/nucleus

ENTRYPOINT ["/usr/local/bin/nucleus"]
