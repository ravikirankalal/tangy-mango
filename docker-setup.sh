#!/bin/bash
# Docker Setup Script for Tangy Mango
# This script handles the SSL certificate workaround for Docker builds

set -e

echo "🥭 Tangy Mango Docker Setup"
echo "=========================="

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed or not in PATH"
    exit 1
fi

if ! command -v docker compose &> /dev/null; then
    echo "❌ Docker Compose is not available"
    exit 1
fi

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed or not in PATH"
    echo "Please install Rust and Cargo first: https://rustup.rs/"
    exit 1
fi

echo "✅ Docker and Cargo are available"

# Build the Rust project locally to avoid SSL certificate issues in Docker
echo "🔨 Building Rust project locally..."
cargo build --release

# Copy the binary for Docker context
echo "📦 Preparing binary for Docker..."
cp target/release/tangy-mango ./tangy-mango-binary

echo "🐳 Building and starting Docker containers..."
docker compose up --build -d

echo "⏳ Waiting for services to be ready..."
sleep 10

# Check if services are running
if docker compose ps tangy-mango-api | grep -q "Up"; then
    echo "✅ Tangy Mango API is running!"
    
    # Test the API endpoint
    if curl -s -f http://localhost:8080/api/v1/users > /dev/null; then
        echo "✅ API is responding to requests!"
        echo "🌐 API available at: http://localhost:8080"
        echo "🗄️  Database available at: localhost:5432"
        echo ""
        echo "📖 Next steps:"
        echo "   - Test API: curl http://localhost:8080/api/v1/users"
        echo "   - View logs: docker compose logs -f tangy-mango-api"
        echo "   - Stop services: docker compose down"
    else
        echo "⚠️  API container is running but not responding to requests"
        echo "📋 Checking logs..."
        docker compose logs tangy-mango-api
    fi
else
    echo "❌ Services failed to start properly"
    echo "📋 Checking logs..."
    docker compose logs
    exit 1
fi