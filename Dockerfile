# Builder stage
FROM rust:1.91 AS builder

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Set work directory
WORKDIR /app

# Copy workspace manifest files
COPY Cargo.toml Cargo.lock ./
COPY sevenmark-parser/Cargo.toml ./sevenmark-parser/Cargo.toml
COPY sevenmark-transform/Cargo.toml ./sevenmark-transform/Cargo.toml
COPY sevenmark-server/Cargo.toml ./sevenmark-server/Cargo.toml

# Copy all source code
COPY sevenmark-parser/src ./sevenmark-parser/src
COPY sevenmark-transform/src ./sevenmark-transform/src
COPY sevenmark-server/src ./sevenmark-server/src

# Build the binary
RUN cargo build --release -p sevenmark-server

# Runtime stage
FROM debian:stable-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -r app && useradd -r -g app app

# Set work directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/sevenmark-server .

# Copy environment file template (optional)
# If you have an actual .env file, uncomment the line below.
# COPY .env .env

# Change ownership to app user
RUN chown -R app:app /app

# Switch to app user
USER app

# Expose port
EXPOSE 9000

# Run the application
CMD ["./sevenmark-server"]