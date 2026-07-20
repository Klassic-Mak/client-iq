# Stage 1: Build the binary
FROM rust:latest AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev

WORKDIR /app
COPY . .

# Build the release binary
RUN cargo build --release

# Stage 2: Runtime environment
FROM debian:bookworm-slim

# Install runtime dependencies (OpenSSL is often needed for MongoDB driver)
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/client-iq-backend .


# Expose the port from your .env
EXPOSE 5000

CMD ["./client-iq-backend"]