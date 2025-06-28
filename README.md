# tangy-mango

A RESTful web service built with Rust using actix-web, sqlx, and PostgreSQL.

## Features

- **Web Framework**: actix-web for HTTP server
- **Database**: sqlx for PostgreSQL with connection pooling
- **Configuration**: TOML-based config using config crate
- **Architecture**: Clean separation with models, DAO, services, and handlers
- **JSON Support**: Serde for serialization/deserialization
- **Error Handling**: Basic error handling in handlers

## Project Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── db.rs                # Database connection and pooling
├── models/
│   └── user.rs          # User entity and DTOs
├── dao/
│   └── user_dao.rs      # Data Access Object for User
├── services/
│   └── user_service.rs  # Business logic for User
└── handlers/
    └── user_handler.rs  # HTTP handlers for User endpoints
migrations/
└── 001_create_users_table.sql  # Database migration
Config.toml              # Configuration file
```

## Setup

### Prerequisites

- Rust (1.70+)
- PostgreSQL (13+)

### Database Setup

1. Install and start PostgreSQL
2. Create a database:
   ```sql
   CREATE DATABASE tangy_mango;
   ```

3. Update the database configuration in `Config.toml`:
   ```toml
   [database]
   host = "localhost"
   port = 5432
   username = "your_username"
   password = "your_password"
   database_name = "tangy_mango"
   max_connections = 10
   ```

### Running the Application

1. Install dependencies:
   ```bash
   cargo build
   ```

2. Run the application:
   ```bash
   cargo run
   ```

The server will start at `http://127.0.0.1:8080` by default.

## API Endpoints

### Users

- **GET /api/v1/users** - Get all users
- **GET /api/v1/users/{id}** - Get user by ID
- **POST /api/v1/users** - Create a new user

### Example Usage

#### Create a user:
```bash
curl -X POST http://localhost:8080/api/v1/users \
  -H "Content-Type: application/json" \
  -d '{"email": "user@example.com", "name": "John Doe"}'
```

#### Get all users:
```bash
curl http://localhost:8080/api/v1/users
```

#### Get user by ID:
```bash
curl http://localhost:8080/api/v1/users/{user-id}
```

## Database Migration

The application will automatically run migrations on startup. The migration creates a `users` table with the following structure:

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## Configuration

The application uses a TOML configuration file (`Config.toml`) with the following structure:

```toml
[server]
host = "127.0.0.1"
port = 8080

[database]
host = "localhost"
port = 5432
username = "postgres"
password = "password"
database_name = "tangy_mango"
max_connections = 10
```

## 🐳 Docker Setup

The application can be easily run using Docker and Docker Compose, with support for both PostgreSQL and MySQL databases.

### Quick Start with Docker

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd tangy-mango
   ```

2. **Start with PostgreSQL (recommended)**:
   ```bash
   ./docker.sh start
   ```

3. **Or start with MySQL**:
   ```bash
   ./docker.sh start-mysql
   ```

4. **Test the API**:
   ```bash
   ./docker.sh test
   ```

The application will be available at `http://localhost:8080`.

### Docker Commands

The `docker.sh` script provides convenient commands:

- `./docker.sh start` - Start with PostgreSQL
- `./docker.sh start-mysql` - Start with MySQL  
- `./docker.sh stop` - Stop all services
- `./docker.sh restart` - Restart services
- `./docker.sh logs` - View application logs
- `./docker.sh logs-db` - View database logs
- `./docker.sh shell` - Open shell in app container
- `./docker.sh db-shell` - Open database shell
- `./docker.sh clean` - Remove containers and volumes
- `./docker.sh status` - Show service status

### Manual Docker Compose

You can also use Docker Compose directly:

```bash
# Start with PostgreSQL
docker compose up -d tangy-mango-api postgres

# Start with MySQL
docker compose --profile mysql up -d tangy-mango-api mysql

# With admin tools (pgAdmin/phpMyAdmin)
docker compose --profile tools up -d

# View logs
docker compose logs -f tangy-mango-api

# Stop services
docker compose down
```

### Database Access

**PostgreSQL**:
- Host: `localhost:5432`
- Database: `tangy_mango` 
- User: `postgres`
- Password: `password`
- pgAdmin: `http://localhost:5050` (admin@tangy-mango.com / admin)

**MySQL** (when using MySQL profile):
- Host: `localhost:3306`
- Database: `tangy_mango`
- User: `tangy_user`
- Password: `password`
- phpMyAdmin: `http://localhost:8081`

### Environment Configuration

Copy `.env.example` to `.env` and modify as needed:

```bash
cp .env.example .env
```

## Local Development

### Prerequisites

- Rust (1.75+)
- PostgreSQL (13+) or MySQL (8.0+)

### Building
```bash
cargo build
```

### Running in development mode with logs
```bash
RUST_LOG=info cargo run
```

### Testing
```bash
# Run unit tests
cargo test

# Run API tests (requires running server)
./test-api.sh
```

### Checking code
```bash
cargo check
```