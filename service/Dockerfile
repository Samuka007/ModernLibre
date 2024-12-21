FROM rust:1.83.0-slim-bullseye AS build

RUN apt-get update && apt-get install -y openssl libssl-dev pkg-config libpq-dev
 
WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml
COPY ./libre-book ./libre-book
COPY ./libre-core ./libre-core
COPY ./libre-user ./libre-user
COPY ./migrations ./migrations

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libpq5 libssl1.1 ca-certificates

WORKDIR /app
COPY --from=build /app/target/release/libre-book /app/libre-book
COPY --from=build /app/target/release/libre-user /app/libre-user
# COPY ./key ./key
