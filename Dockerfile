FROM rust:latest as builder

ENV SQLX_OFFLINE true

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src
RUN echo 'fn main(){}' > src/main.rs
RUN cargo build

COPY . .
RUN cargo build



FROM debian:stable-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/debug/webserver .
COPY ./regexes.yaml ./keys ./

CMD [ "./webserver" ]