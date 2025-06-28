# Use pre-built binary approach to avoid SSL certificate issues
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false tangy-mango

# Create app directory
WORKDIR /app

# Copy the pre-built binary from local build
COPY tangy-mango-binary ./tangy-mango

# Copy migrations (config will be mounted via docker-compose)
COPY migrations ./migrations

# Change ownership to app user and make binary executable
RUN chown -R tangy-mango:tangy-mango /app && chmod +x tangy-mango

# Switch to app user
USER tangy-mango

# Expose port
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/api/v1/users || exit 1

# Run the application
CMD ["./tangy-mango"]