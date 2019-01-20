FROM rust:latest

WORKDIR /server

RUN rustup default nightly \
    && rustup update \
    && cargo install cargo-watch

EXPOSE 8000
