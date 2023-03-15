FROM rust:1.68.0 AS builder
WORKDIR /usr/src

RUN cargo new udp-flare
WORKDIR /usr/src/udp-flare
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --path .

FROM gcr.io/distroless/cc
COPY --from=builder /usr/local/cargo/bin/udp-flare .
USER 1000
CMD ["./udp-flare"]