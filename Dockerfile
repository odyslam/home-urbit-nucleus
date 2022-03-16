FROM rust:1.59 as builder

RUN rustup target add aarch64-unknown-none && rustup add armv7-unknown-linux-gnueabihf
RUN apt-get update

WORKDIR /nucleus

COPY . /nucleus

RUN cargo build --release

FROM alpine:3.15

# hadolint ignore=DL3018
RUN apk --no-cache add ca-certificates vim

USER nucleus

COPY --from=builder /nucleus/target/release/nucleus /usr/local/bin/nucleus

ENTRYPOINT ["/usr/local/bin/nucleus"]
