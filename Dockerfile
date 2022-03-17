FROM debian:bullseye as builder
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y build-essential curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
# add cargo to path
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /nucleus

COPY . /nucleus

RUN rustup update && cargo build --release

FROM alpine:3.15

# hadolint ignore=DL3018
RUN apk --no-cache add ca-certificates vim

USER nucleus

COPY --from=builder /nucleus/target/release/nucleus /usr/local/bin/nucleus

ENTRYPOINT ["/usr/local/bin/nucleus"]
