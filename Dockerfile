# Builder stage
FROM rust:1.90 AS builder

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Set work directory
WORKDIR /app

# Copy manifest files first to leverage Docker cache for dependencies
COPY Cargo.toml Cargo.lock ./

# Create dummy src directory and files for dependency caching
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Build dependencies for sevenmark-server only with server features
RUN cargo build --release --bin sevenmark-server --features server --target-dir /app/target_deps

# Remove the dummy src directory and its content
RUN rm -rf src

# Copy the actual source code
COPY src ./src

# Build the sevenmark-server binary using previously cached dependencies
RUN cargo build --release --bin sevenmark-server --features server

# Runtime stage
FROM debian:stable-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
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