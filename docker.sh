#!/bin/bash

# Tangy Mango Docker Management Script
# This script provides easy commands to manage the dockerized application

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_color() {
    echo -e "${1}${2}${NC}"
}

# Function to check if Docker is running
check_docker() {
    if ! docker info > /dev/null 2>&1; then
        print_color $RED "Error: Docker is not running. Please start Docker and try again."
        exit 1
    fi
}

# Function to display usage
usage() {
    echo "Tangy Mango Docker Management Script"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  start           Start the application with PostgreSQL (default)"
    echo "  start-mysql     Start the application with MySQL"
    echo "  stop            Stop all services"
    echo "  restart         Restart all services"
    echo "  build           Build the application image"
    echo "  logs            Show application logs"
    echo "  logs-db         Show database logs"
    echo "  clean           Remove all containers and volumes"
    echo "  shell           Open a shell in the running application container"
    echo "  db-shell        Open a database shell (PostgreSQL by default)"
    echo "  test            Run API tests against the running application"
    echo "  status          Show status of all services"
    echo "  help            Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 start                    # Start with PostgreSQL"
    echo "  $0 start-mysql             # Start with MySQL"
    echo "  $0 logs                    # View application logs"
    echo "  $0 shell                   # Open shell in app container"
}

# Main commands
case "${1:-}" in
    start)
        check_docker
        print_color $GREEN "Starting Tangy Mango with PostgreSQL..."
        docker compose up -d tangy-mango-api postgres
        print_color $GREEN "Services started! Application available at: http://localhost:8080"
        print_color $YELLOW "Use '$0 logs' to view logs"
        ;;
    
    start-mysql)
        check_docker
        print_color $GREEN "Starting Tangy Mango with MySQL..."
        docker compose --profile mysql up -d tangy-mango-api mysql
        print_color $GREEN "Services started! Application available at: http://localhost:8080"
        print_color $YELLOW "Use '$0 logs' to view logs"
        ;;
    
    stop)
        check_docker
        print_color $YELLOW "Stopping all services..."
        docker compose down
        print_color $GREEN "All services stopped."
        ;;
    
    restart)
        check_docker
        print_color $YELLOW "Restarting services..."
        docker compose restart
        print_color $GREEN "Services restarted."
        ;;
    
    build)
        check_docker
        print_color $YELLOW "Building application image..."
        docker compose build tangy-mango-api
        print_color $GREEN "Build completed."
        ;;
    
    logs)
        check_docker
        print_color $BLUE "Showing application logs (Ctrl+C to exit)..."
        docker compose logs -f tangy-mango-api
        ;;
    
    logs-db)
        check_docker
        print_color $BLUE "Showing database logs (Ctrl+C to exit)..."
        if docker compose ps postgres | grep -q "Up"; then
            docker compose logs -f postgres
        elif docker compose ps mysql | grep -q "Up"; then
            docker compose logs -f mysql
        else
            print_color $RED "No database container is running."
        fi
        ;;
    
    clean)
        check_docker
        print_color $YELLOW "Removing all containers and volumes..."
        docker compose down -v --remove-orphans
        docker system prune -f
        print_color $GREEN "Cleanup completed."
        ;;
    
    shell)
        check_docker
        if docker compose ps tangy-mango-api | grep -q "Up"; then
            print_color $BLUE "Opening shell in application container..."
            docker compose exec tangy-mango-api /bin/bash
        else
            print_color $RED "Application container is not running. Start it first with '$0 start'"
        fi
        ;;
    
    db-shell)
        check_docker
        if docker compose ps postgres | grep -q "Up"; then
            print_color $BLUE "Opening PostgreSQL shell..."
            docker compose exec postgres psql -U postgres -d tangy_mango
        elif docker compose ps mysql | grep -q "Up"; then
            print_color $BLUE "Opening MySQL shell..."
            docker compose exec mysql mysql -u tangy_user -ppassword tangy_mango
        else
            print_color $RED "No database container is running."
        fi
        ;;
    
    test)
        check_docker
        if ! docker compose ps tangy-mango-api | grep -q "Up"; then
            print_color $RED "Application is not running. Start it first with '$0 start'"
            exit 1
        fi
        print_color $BLUE "Running API tests..."
        # Wait a moment for the service to be ready
        sleep 2
        ./test-api.sh
        ;;
    
    status)
        check_docker
        print_color $BLUE "Service Status:"
        docker compose ps
        ;;
    
    help|--help|-h)
        usage
        ;;
    
    "")
        usage
        ;;
    
    *)
        print_color $RED "Unknown command: $1"
        echo ""
        usage
        exit 1
        ;;
esac