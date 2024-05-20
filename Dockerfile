FROM rust:latest as builder

ENV SQLX_OFFLINE true

WORKDIR /usr/src/app

RUN mkdir src && echo 'fn main(){}' > src/main.rs

COPY Cargo.toml Cargo.lock ./
RUN cargo build

COPY . .
RUN cargo build



FROM debian:stable-slim

RUN apt-get update && \
    apt-get install -y openssl libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/debug/webserver ./app
COPY ./regexes.yaml ./
COPY ./keys/ ./keys/

CMD [ "./app" ]