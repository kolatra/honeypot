FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
RUN apt-get update && apt-get install lld clang -y
RUN rustup toolchain install nightly
RUN cargo +nightly chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo +nightly build --release --bin mc-honeypot

FROM debian:bullseye-slim
RUN apt-get update && \
    apt-get install -y ca-certificates libpq-dev && \
    apt-get clean
WORKDIR /app
COPY --from=builder /app/target/release/mc-honeypot /usr/local/bin
ENTRYPOINT ["/usr/local/bin/mc-honeypot"]