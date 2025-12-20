#!/bin/bash

# Generate TypeScript types from Rust using ts-rs

set -e

echo "Generating TypeScript types from Rust..."

# Navigate to backend
cd backend

# Run tests with ts-rs export feature
# This triggers ts-rs to export TypeScript types
cargo test --quiet --features ts-rs 2>/dev/null || true

echo "✓ TypeScript types generated in frontend/src/lib/types/"

# Navigate to frontend to check types
cd ../frontend

# Run TypeScript type checking
echo "Checking TypeScript types..."
npm run check

echo "✓ All done!"
