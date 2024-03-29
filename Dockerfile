# FROM debian:bullseye as builder
FROM rust:1.59-bullseye as builder
ARG DEBIAN_FRONTEND=noninteractive

# hadolint ignore=DL3008
RUN apt-get update && apt-get install --no-install-recommends -y build-essential curl pkg-config ca-certificates
# RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
# add cargo to path
# ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /nucleus

COPY . /nucleus

# RUN rustup update
RUN cargo build --release

FROM alpine:3.15

# hadolint ignore=DL3018
RUN apk --no-cache add ca-certificates vim

USER nucleus

COPY --from=builder /nucleus/target/release/nucleus /usr/local/bin/nucleus

ENTRYPOINT ["/usr/local/bin/nucleus"]
