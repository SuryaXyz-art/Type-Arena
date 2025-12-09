.PHONY: build check test clean deploy run

# Build all applications
build:
	@echo "🔨 Building Flash Markets..."
	cargo build --release --target wasm32-unknown-unknown
	@echo "✅ Build complete!"

# Quick check without building
check:
	@echo "🔍 Checking code..."
	cargo check --target wasm32-unknown-unknown
	@echo "✅ Check complete!"

# Run tests
test:
	@echo "🧪 Running tests..."
	cargo test
	@echo "✅ Tests complete!"

# Lint code
lint:
	@echo "🧹 Linting code..."
	cargo clippy --target wasm32-unknown-unknown
	@echo "✅ Lint complete!"

# Format code
fmt:
	@echo "📝 Formatting code..."
	cargo fmt
	@echo "✅ Format complete!"

# Clean build artifacts
clean:
	@echo "🗑️ Cleaning build artifacts..."
	cargo clean
	@echo "✅ Clean complete!"

# Build individual apps
build-token:
	@echo "🔨 Building Token app..."
	cargo build -p token --release --target wasm32-unknown-unknown

build-market:
	@echo "🔨 Building Market app..."
	cargo build -p market --release --target wasm32-unknown-unknown

build-oracle:
	@echo "🔨 Building Oracle app..."
	cargo build -p oracle --release --target wasm32-unknown-unknown

# Docker commands
docker-build:
	@echo "🐳 Building Docker image..."
	docker compose build
	@echo "✅ Docker build complete!"

docker-up:
	@echo "🐳 Starting Docker containers..."
	docker compose up -d
	@echo "✅ Containers started!"

docker-down:
	@echo "🐳 Stopping Docker containers..."
	docker compose down
	@echo "✅ Containers stopped!"

docker-logs:
	@echo "📜 Showing Docker logs..."
	docker compose logs -f

# Development workflow
dev: check build
	@echo "✅ Development build complete!"

# Production workflow
prod: clean lint test build
	@echo "✅ Production build complete!"

# Quick start
quick: build docker-up
	@echo "✅ Flash Markets is running!"
	@echo "📡 GraphQL: http://localhost:8080/graphql"
	@echo "🌐 Frontend: http://localhost:3000"

# Help
help:
	@echo "Flash Markets - Makefile Commands"
	@echo ""
	@echo "  make build         - Build all applications"
	@echo "  make check         - Quick code check"
	@echo "  make test          - Run tests"
	@echo "  make lint          - Run clippy"
	@echo "  make fmt           - Format code"
	@echo "  make clean         - Clean build artifacts"
	@echo ""
	@echo "  make docker-build  - Build Docker image"
	@echo "  make docker-up     - Start containers"
	@echo "  make docker-down   - Stop containers"
	@echo "  make docker-logs   - View logs"
	@echo ""
	@echo "  make dev           - Quick development build"
	@echo "  make prod          - Full production build"
	@echo "  make quick         - Build and start everything"
