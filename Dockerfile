FROM rust:1.80.1 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM rust:1.80.1
ENV CONFIG_PATH=/app/Config.toml

WORKDIR /app
COPY --from=builder /app/target/release/bin .
CMD ["./bin"]