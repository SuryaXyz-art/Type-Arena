# PowerShel script to build and run the local network in Docker

$ErrorActionPreference = "Stop"

Write-Host "Building Docker Image (type-arena-local)..."
docker build -f Dockerfile.local -t type-arena-local .

if ($LASTEXITCODE -ne 0) {
    Write-Error "Docker build failed. Is Docker Desktop running?"
    exit 1
}

Write-Host "Starting Docker Container..."
$CurrentDir = (Get-Location).Path
# Fix: Use correct PowerShell string interpolation for path with colon
$Mount = "$CurrentDir" + ":/app"

Write-Host "Mounting: $Mount"

# We map ports 8080 (Faucet) and 8081 (Service)
docker run --rm -it -p 8080:8080 -p 8081:8081 -v "$Mount" type-arena-local
