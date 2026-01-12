# Use a base image with Rust installed
FROM rust:1.75-bookworm as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    clang \
    protobuf-compiler \
    libssl-dev \
    pkg-config \
    git \
    nodejs \
    npm

# Install Linera CLI
# Install Linera CLI (Optional - commented out for faster build times as we target Testnet Conway)
# RUN cargo install linera-service --features storage-service

# Set up the project directory
WORKDIR /app
COPY . .

# Build the Rust Contract
WORKDIR /app/contracts/type_arena
RUN rustup target add wasm32-unknown-unknown
RUN cargo build --release --target wasm32-unknown-unknown

# Build the Frontend
WORKDIR /app/frontend/client
RUN npm install
RUN npm run build

# Final Stage: Serve the App
FROM node:18-alpine

WORKDIR /app
COPY --from=builder /app/frontend/client/dist ./dist
# Note: We do not copy the linera binary here because the container is for serving the frontend.
# If you need the linera binary to run a local net inside docker, you would need a different setup.
# For production/judging, we assume the app connects to Testnet Conway or a running chain.

# Install http-server
RUN npm install -g http-server

# Expose port
EXPOSE 8080

# Command to serve with required headers
CMD ["http-server", "dist", "-p", "8080", "--cors", "-H", "{\"Cross-Origin-Embedder-Policy\": \"require-corp\", \"Cross-Origin-Opener-Policy\": \"same-origin\"}"]
