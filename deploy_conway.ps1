# Type Arena - Conway Testnet Deployment (PowerShell)
# This script uses Docker to deploy to Linera Conway testnet

$ErrorActionPreference = "Stop"

Write-Host "=========================================="
Write-Host "Type Arena - Conway Testnet Deployment"
Write-Host "=========================================="
Write-Host ""

# Check if Docker is running
Write-Host "[1/4] Checking Docker..."
try {
    docker info | Out-Null
    Write-Host "✓ Docker is running"
}
catch {
    Write-Error "Docker is not running. Please start Docker Desktop and try again."
    exit 1
}

# Build deployment image
Write-Host ""
Write-Host "[2/4] Building deployment Docker image..."
Write-Host "This may take several minutes on first run..."
docker build -f Dockerfile.deploy -t type-arena-deploy .

if ($LASTEXITCODE -ne 0) {
    Write-Error "Failed to build Docker image"
    exit 1
}

Write-Host "✓ Docker image built successfully"

# Run deployment
Write-Host ""
Write-Host "[3/4] Running deployment to Conway testnet..."
Write-Host "This will:"
Write-Host "  - Initialize Linera wallet"
Write-Host "  - Build smart contracts"
Write-Host "  - Publish to Conway testnet"
Write-Host "  - Update frontend configuration"
Write-Host ""

# Create volume for wallet persistence if it doesn't exist
docker volume create type-arena-wallet | Out-Null

# Run deployment container
docker run --rm `
    --name type-arena-deploy `
    -v type-arena-wallet:/root/.config/linera `
    -v "${PWD}/frontend/client/public:/app/frontend/client/public" `
    type-arena-deploy

if ($LASTEXITCODE -ne 0) {
    Write-Error "Deployment failed"
    exit 1
}

Write-Host ""
Write-Host "✓ Deployment completed successfully"

# Read updated configuration
Write-Host ""
Write-Host "[4/4] Deployment Summary"
Write-Host "=========================================="
$ConfigPath = "frontend/client/public/config.json"
if (Test-Path $ConfigPath) {
    $Config = Get-Content $ConfigPath -Raw | ConvertFrom-Json
    Write-Host "Chain ID:       $($Config.chainId)"
    Write-Host "Application ID: $($Config.marketAppId)"
}
else {
    Write-Host "Warning: Could not read config.json"
}

Write-Host ""
Write-Host "=========================================="
Write-Host "Next Steps:"
Write-Host "1. Build frontend:"
Write-Host "   cd frontend/client"
Write-Host "   npm install"
Write-Host "   npm run build"
Write-Host ""
Write-Host "2. Serve frontend locally:"
Write-Host "   npm run preview"
Write-Host ""
Write-Host "3. Or build and run complete stack:"
Write-Host "   docker-compose up"
Write-Host "=========================================="
