-- MySQL initialization script
-- This script will be executed when the container starts for the first time

-- The database 'tangy_mango' is already created by the MYSQL_DATABASE environment variable
-- The user 'tangy_user' is already created by MYSQL_USER and MYSQL_PASSWORD

USE tangy_mango;

-- Create the users table equivalent to PostgreSQL version
-- Note: This is for MySQL, the current Rust code uses PostgreSQL
CREATE TABLE IF NOT EXISTS users (
    id CHAR(36) PRIMARY KEY,  -- UUID as string in MySQL
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    -- Indexes for better performance
    INDEX idx_users_email (email),
    INDEX idx_users_created_at (created_at)
);

-- Grant permissions to the application user
GRANT ALL PRIVILEGES ON tangy_mango.* TO 'tangy_user'@'%';
FLUSH PRIVILEGES;