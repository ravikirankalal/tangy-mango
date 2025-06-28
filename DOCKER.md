# Docker Setup Guide

This document provides comprehensive instructions for running the Tangy Mango web service using Docker.

## Quick Start

### Prerequisites

- Docker (20.10+)
- Docker Compose v2 (or `docker compose` command)
- Rust and Cargo (for local builds to avoid SSL issues)

### Automated Setup (Recommended)

The easiest way to get started is using the provided setup script:

```bash
# Clone the repository
git clone <repository-url>
cd tangy-mango

# Run the automated setup script
./docker-setup.sh
```

This script will:
1. Build the Rust project locally
2. Prepare the Docker context
3. Build and start all services
4. Verify everything is working

### Manual Setup

### Option 1: Using Pre-built Configuration (Recommended)

If you encounter Docker build issues due to network/SSL certificate problems in your environment:

1. **Build the Rust project locally**:
   ```bash
   git clone <repository-url>
   cd tangy-mango
   cargo build --release
   ```

2. **Copy the binary to Docker context**:
   ```bash
   cp target/release/tangy-mango ./tangy-mango-binary
   ```

3. **Build and start with Docker Compose**:
   ```bash
   docker compose up --build -d
   ```

This approach avoids SSL certificate issues by building the Rust dependencies locally first, then using the pre-built binary in a lightweight Docker container.
   USER tangy-mango
   EXPOSE 8080
   CMD ["./tangy-mango"]
   ```

### Option 2: Standard Docker Build

If your environment doesn't have SSL certificate issues, you can use the standard approach:

```bash
# Start with PostgreSQL (default)
./docker.sh start

# Or start with MySQL
./docker.sh start-mysql
```

Note: If you encounter SSL certificate errors during the Docker build process, use Option 1 or the automated setup script instead.

## Available Services

### Core Services

1. **tangy-mango-api**: Main Rust web service
   - Port: 8080
   - Health check endpoint: `GET /api/v1/users`

2. **postgres**: PostgreSQL database (default)
   - Port: 5432
   - Database: `tangy_mango`
   - User: `postgres`
   - Password: `password`

3. **mysql**: MySQL database (alternative)
   - Port: 3306  
   - Database: `tangy_mango`
   - User: `tangy_user`
   - Password: `password`

### Optional Admin Tools

4. **pgadmin**: PostgreSQL admin interface
   - Port: 5050
   - Email: `admin@tangy-mango.com`
   - Password: `admin`

5. **phpmyadmin**: MySQL admin interface
   - Port: 8081
   - Auto-login configured

## Docker Commands Reference

### Using the management script (`docker.sh`)

```bash
# Service Management
./docker.sh start           # Start with PostgreSQL
./docker.sh start-mysql     # Start with MySQL
./docker.sh stop            # Stop all services
./docker.sh restart         # Restart services
./docker.sh status          # Show service status

# Development
./docker.sh build           # Build application image
./docker.sh logs            # View application logs
./docker.sh logs-db         # View database logs
./docker.sh shell           # Open shell in app container
./docker.sh db-shell        # Open database shell

# Testing & Cleanup
./docker.sh test            # Run API tests
./docker.sh clean           # Remove containers and volumes
```

### Using Docker Compose directly

```bash
# Start core services with PostgreSQL
docker compose up -d tangy-mango-api postgres

# Start with MySQL instead
docker compose --profile mysql up -d tangy-mango-api mysql

# Include admin tools
docker compose --profile tools up -d

# View logs
docker compose logs -f tangy-mango-api
docker compose logs -f postgres

# Stop services
docker compose down

# Remove volumes (data will be lost)
docker compose down -v
```

## Configuration

### Environment Variables

Copy `.env.example` to `.env` and customize:

```bash
cp .env.example .env
```

Key variables:
- `DB_HOST`: Database hostname (postgres/mysql)
- `DB_PORT`: Database port (5432/3306)
- `DB_USERNAME`: Database user
- `DB_PASSWORD`: Database password
- `SERVER_HOST`: API server host (0.0.0.0 for Docker)
- `SERVER_PORT`: API server port (8080)

### Configuration Files

- `Config.docker.toml`: PostgreSQL configuration for Docker
- `Config.mysql.toml`: MySQL configuration for Docker
- `Config.toml`: Local development configuration

## Database Initialization

### PostgreSQL
Database and tables are created automatically via:
1. Environment variables (`POSTGRES_DB`, etc.)
2. Initialization script (`init-postgres.sql`)
3. Application migrations (`migrations/001_create_users_table.sql`)

### MySQL
For MySQL setup:
1. Database created via `MYSQL_DATABASE` environment variable
2. User created via `MYSQL_USER`/`MYSQL_PASSWORD`
3. Schema initialized via `init-mysql.sql`
4. **Note**: Current application uses PostgreSQL-specific features

## API Testing

### Automated Tests
```bash
# Start services first
./docker.sh start

# Run tests
./docker.sh test
```

### Manual Testing

```bash
# Create a user
curl -X POST http://localhost:8080/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "name": "Test User"}'

# Get all users
curl http://localhost:8080/api/v1/users

# Get specific user (replace {id} with actual UUID)
curl http://localhost:8080/api/v1/users/{id}
```

## Troubleshooting

## Troubleshooting

### Common Issues

1. **Build failures due to SSL certificates**:
   - **Problem**: Docker build fails with SSL certificate errors when downloading Rust crates
   - **Solution**: Use the pre-built binary approach with `docker-setup.sh` script
   - **Alternative**: Use Option 1 (pre-built binary approach) manually
   - **Details**: This is a common issue in certain environments where SSL certificates aren't properly configured

2. **Port conflicts**:
   - Modify port mappings in `docker-compose.yml`
   - Default ports: 8080 (API), 5432 (PostgreSQL), 3306 (MySQL)

3. **Database connection issues**:
   - Ensure database container is healthy: `docker compose ps`
   - Check network connectivity: `docker compose logs postgres`

4. **Permission errors**:
   - Verify file ownership: `ls -la`
   - Rebuild image: `docker compose build --no-cache`

5. **Config file not found errors**:
   - **Problem**: Application can't find Config.toml
   - **Solution**: Ensure `Config.docker.toml` exists and is properly mounted
   - **Check**: Verify docker-compose.yml volume mount is correct

### Debugging

```bash
# Check service status
docker compose ps

# View detailed logs
docker compose logs -f [service-name]

# Access container shell
docker compose exec tangy-mango-api /bin/bash

# Check database connection
docker compose exec postgres psql -U postgres -d tangy_mango

# Inspect network
docker network inspect tangy-mango_tangy-mango-network
```

### Performance Tuning

1. **Database connections**: Adjust `max_connections` in config
2. **Memory limits**: Add resource limits to compose file
3. **Build caching**: Use BuildKit for faster builds

## Security Considerations

- Change default passwords in production
- Use environment-specific configuration files
- Consider using Docker secrets for sensitive data
- Run containers as non-root users (already configured)
- Use specific image tags instead of `latest`

## Production Deployment

For production deployment:

1. **Use environment-specific configurations**
2. **Set up proper logging and monitoring**
3. **Configure backup strategies for database volumes**
4. **Use orchestration tools like Kubernetes or Docker Swarm**
5. **Implement proper secret management**
6. **Set up SSL/TLS termination (e.g., nginx proxy)**

Example production compose override:

```yaml
# docker-compose.prod.yml
services:
  tangy-mango-api:
    environment:
      - RUST_LOG=warn
    restart: always
    deploy:
      resources:
        limits:
          memory: 512M
        reservations:
          memory: 256M
          
  postgres:
    volumes:
      - /var/lib/postgresql/data:/var/lib/postgresql/data
    environment:
      - POSTGRES_PASSWORD_FILE=/run/secrets/postgres_password
    secrets:
      - postgres_password
```