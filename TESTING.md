# Testing Documentation

This document describes the testing approach and structure for the tangy-mango web service.

## Test Structure

The project includes comprehensive unit and integration tests organized in the following layers:

### Unit Tests

Unit tests are located within each module using `#[cfg(test)]` blocks:

#### 1. Models (`src/models/user.rs`)
- Tests data structure creation and validation
- Tests serialization/deserialization of user entities
- Tests conversion between User and UserResponse types
- **Coverage**: Struct creation, field validation, type conversions

#### 2. Services (`src/services/user_service.rs`)
- Tests business logic and validation rules
- Tests service initialization and structure
- Tests input validation (empty email/name detection)
- **Coverage**: Business logic, input validation, service structure

#### 3. DAO (`src/dao/user_dao.rs`)
- Tests data access layer structure and setup
- Tests UUID generation and timestamp creation
- Tests user struct creation from database-like data
- **Coverage**: Data structures, UUID/timestamp generation, struct mapping

#### 4. Handlers (`src/handlers/user_handler.rs`)
- Tests HTTP handler data structures
- Tests JSON serialization/deserialization
- Tests error response formatting
- Tests UUID parsing for path parameters
- **Coverage**: HTTP data handling, JSON processing, error responses

#### 5. Configuration (`src/config.rs`)
- Tests configuration structure creation
- Tests database URL generation from config
- Tests configuration validation and cloning
- **Coverage**: Configuration management, URL building, validation

#### 6. Database (`src/db.rs`)
- Tests database configuration integration
- Tests connection pool settings validation
- Tests type aliases and structure setup
- **Coverage**: Database setup logic, configuration integration

### Integration Tests (`tests/integration_tests.rs`)

Integration tests demonstrate how components work together:

- **User Workflow**: Tests complete request-to-response data flow
- **Error Scenarios**: Tests validation and error handling across layers
- **JSON Processing**: Tests end-to-end serialization/deserialization
- **Configuration Integration**: Tests how configuration components interact

## Running Tests

```bash
# Run all tests (unit + integration)
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_user_creation
```

## Test Coverage

Current test coverage includes:

- ✅ **Models**: 4 tests covering struct creation and conversions
- ✅ **Services**: 2 tests covering business logic and validation
- ✅ **DAO**: 5 tests covering data access patterns
- ✅ **Handlers**: 5 tests covering HTTP request/response handling
- ✅ **Configuration**: 6 tests covering config management
- ✅ **Database**: 3 tests covering database setup
- ✅ **Integration**: 4 tests covering component interaction

**Total**: 29 tests across all layers

## Testing Philosophy

### What We Test
- **Unit Tests**: Focus on individual component logic without external dependencies
- **Integration Tests**: Test component interactions and data flow
- **Input Validation**: Ensure proper handling of invalid/empty data
- **Error Handling**: Verify error responses and edge cases
- **Data Serialization**: Test JSON processing for API compatibility

### What We Don't Test (Requires External Setup)
- **Database Connections**: Actual PostgreSQL connections require test database
- **HTTP Endpoints**: Full HTTP integration requires test server setup
- **Network Operations**: External service calls require mock services

### Future Test Enhancements

For production environments, consider adding:

1. **Database Integration Tests**: Using test containers or test databases
2. **HTTP Integration Tests**: Using test servers and HTTP clients
3. **Performance Tests**: Load testing and benchmarking
4. **Mock Testing**: Using mockall or similar for complex dependency injection
5. **Property-Based Tests**: Using proptest for comprehensive input validation

## Test Best Practices

- **Fast**: Tests run quickly without external dependencies
- **Isolated**: Each test is independent and can run in any order
- **Deterministic**: Tests produce consistent results across runs
- **Comprehensive**: Cover both happy path and error scenarios
- **Maintainable**: Clear test names and structure for easy maintenance