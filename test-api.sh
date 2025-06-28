#!/bin/bash

# Example script to test the tangy-mango API
# Make sure the server is running before executing this script

BASE_URL="http://localhost:8080/api/v1"

echo "=== Testing tangy-mango API ==="
echo

# Create a new user
echo "1. Creating a new user..."
USER_RESPONSE=$(curl -s -X POST "$BASE_URL/users" \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "name": "Test User"}')

echo "Response: $USER_RESPONSE"
echo

# Extract user ID from response (requires jq)
if command -v jq &> /dev/null; then
    USER_ID=$(echo "$USER_RESPONSE" | jq -r '.id')
    echo "Created user with ID: $USER_ID"
    echo

    # Get user by ID
    echo "2. Getting user by ID..."
    curl -s "$BASE_URL/users/$USER_ID" | jq '.'
    echo
fi

# Get all users
echo "3. Getting all users..."
curl -s "$BASE_URL/users" | jq '.' || curl -s "$BASE_URL/users"
echo

echo "=== Testing complete ==="