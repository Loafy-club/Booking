#!/bin/bash

# Local development setup script

set -e

echo "🚀 Setting up Loafy Club Booking Platform for local development..."

# Check prerequisites
echo "Checking prerequisites..."

if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first: https://rustup.rs/"
    exit 1
fi

if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js first."
    exit 1
fi

echo "✓ All prerequisites installed"

# Check for .env file
if [ ! -f .env ]; then
    echo "Creating .env file from .env.example..."
    cp .env.example .env
    echo "⚠️  Please edit .env with your service credentials before continuing."
    echo "Press Enter when ready..."
    read
fi

# Start PostgreSQL
echo "Starting PostgreSQL database..."
docker-compose -f infra/docker/docker-compose.dev.yml up -d

# Wait for PostgreSQL to be ready
echo "Waiting for PostgreSQL to be ready..."
sleep 5

# Check if SQLx CLI is installed
if ! command -v sqlx &> /dev/null; then
    echo "Installing SQLx CLI..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Run database migrations
echo "Running database migrations..."
cd backend
sqlx database create || true
sqlx migrate run

# Install frontend dependencies
echo "Installing frontend dependencies..."
cd ../frontend
npm install

# Go back to root
cd ..

echo ""
echo "✅ Setup complete!"
echo ""
echo "To start the application, run these commands in separate terminals:"
echo ""
echo "  Terminal 1 (Backend API):"
echo "    cd backend && cargo run --bin loafy-api"
echo ""
echo "  Terminal 2 (Background Jobs):"
echo "    cd backend && cargo run --bin loafy-jobs"
echo ""
echo "  Terminal 3 (Frontend):"
echo "    cd frontend && npm run dev"
echo ""
echo "Then visit: http://localhost:5173"
echo ""
