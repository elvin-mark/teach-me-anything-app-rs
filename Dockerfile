# Multi-stage build with cargo-chef for better caching

FROM rust:alpine AS chef
RUN apk add --no-cache musl-dev
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

# Install build dependencies
RUN apk add --no-cache musl-dev sqlite-dev openssl-dev openssl-libs-static pkgconfig

# Build dependencies - this layer is cached
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Copy source code and build
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Strip the binary to reduce size even further
RUN strip target/x86_64-unknown-linux-musl/release/teach-me-anything-app-rs

# Runtime stage
FROM scratch

# Copy CA certificates for HTTPS
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy the binary
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/teach-me-anything-app-rs /app

# Expose port
EXPOSE 8000

# Set environment variables
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

# Run the binary
ENTRYPOINT ["/app"]