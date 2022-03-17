FROM rust:1.59-buster as builder
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y apt-utils build-essential

WORKDIR /nucleus

COPY . /nucleus

RUN cargo build --release

FROM alpine:3.15

# hadolint ignore=DL3018
RUN apk --no-cache add ca-certificates vim

USER nucleus

COPY --from=builder /nucleus/target/release/nucleus /usr/local/bin/nucleus

ENTRYPOINT ["/usr/local/bin/nucleus"]
