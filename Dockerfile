# Chef stage - install cargo-chef
FROM rust:1.93.0 AS chef
WORKDIR /app
RUN cargo install cargo-chef

# Planner stage - generate recipe.json
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage - build dependencies then application
FROM chef AS builder

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy dependency recipe
COPY --from=planner /app/recipe.json recipe.json

# Create stub files that cargo-chef doesn't generate for [[example]] entries
RUN mkdir -p crates/sevenmark_parser/examples && \
    echo "fn main() {}" > crates/sevenmark_parser/examples/gen_expected.rs

# Build dependencies (this layer is cached unless dependencies change)
RUN cargo chef cook --release --recipe-path recipe.json --bin sevenmark_server

# Copy source code
COPY . .

# Build application (dependencies already built, only source compilation)
RUN cargo build --release --bin sevenmark_server

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
COPY --from=builder /app/target/release/sevenmark_server .

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
CMD ["./sevenmark_server"]