FROM rust:1.86.0

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# Install Linera CLI
RUN cargo install linera-service

# Set working directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml rust-toolchain.toml ./

# Copy all application code
COPY abi ./abi
COPY token ./token
COPY market ./market
COPY oracle ./oracle

# Build applications
RUN cargo build --release --target wasm32-unknown-unknown

# Expose ports
EXPOSE 8080 3000

CMD ["bash"]
