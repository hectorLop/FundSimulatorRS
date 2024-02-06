# Builder stage
FROM rust:1.75.0 as builder

WORKDIR /app

COPY . .

EXPOSE 3000
ENV SQLX_OFFLINE true

RUN cargo build --release

# Runtime stage
FROM rust:1.75.0-slim as runtime

WORKDIR /app

# Copy the binary created in the "Builder" stage
COPY --from=builder /app/target/release/fund-simulator-rs fund-simulator-rs

ENTRYPOINT ["./fund-simulator-rs", "--mode", "server"]
