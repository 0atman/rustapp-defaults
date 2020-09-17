# Build Stage #

FROM rust:1.40.0 AS builder
WORKDIR /usr/src/
RUN rustup install nightly
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new app
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle Stage #

FROM alpine
COPY --from=builder /usr/local/cargo/bin/rustapitest .
USER 1000
CMD ["./rustapitest"]
