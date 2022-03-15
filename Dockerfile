FROM rust:latest as builder

RUN apt update && apt install -y

WORKDIR /nucleus

ADD . /nucleus

RUN cargo build --release

FROM alpine:latest

RUN apk --no-cache add ca-certificates vim

USER nucleus

COPY --from=builder /nucleus/target/release/nucleus /usr/local/bin/nucleus

ENTRYPOINT ["/usr/local/bin/nucleus"]
