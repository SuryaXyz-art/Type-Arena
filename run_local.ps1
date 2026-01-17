# Type Arena - Local Network Runner (PowerShell)
# Starts a local Linera network, deploys the app, and configures the frontend.

$ErrorActionPreference = "Stop"

# --- CONFIGURATION ---
$LocalFaucetPort = 8080
$NodeServicePort = 8081
$StorageArgs = @("--storage", "memory")

# --- CLEANUP OLD RUNS ---
Write-Host "Cleaning up old processes..."
Stop-Process -Name "linera" -ErrorAction SilentlyContinue

Write-Host "Cleaning up old wallet..."
Remove-Item -Path "$env:APPDATA\linera\wallet.json" -Force -ErrorAction SilentlyContinue
Remove-Item -Path "$env:APPDATA\linera\keystore.json" -Force -ErrorAction SilentlyContinue

# --- START LOCAL NETWORK ---
Write-Host "Starting Local Linera Network (Background)..."
$NetProcess = Start-Process -FilePath "linera" -ArgumentList "net", "up", "--faucet-port", "$LocalFaucetPort", "--storage", "memory" -PassThru -NoNewWindow -RedirectStandardOutput "net.log" -RedirectStandardError "net_err.log"

Write-Host "Waiting for network to spin up (15 seconds)..."
Start-Sleep -Seconds 15

if ($NetProcess.HasExited) {
    Write-Error "Linera network failed to start. Check net_err.log."
    Get-Content "net_err.log" | Select-Object -Last 10
    exit 1
}

# --- INITIALIZE WALLET ---
Write-Host "Initializing Client Wallet..."
linera wallet init --faucet "http://localhost:$LocalFaucetPort" --storage memory
if ($LASTEXITCODE -ne 0) {
    Stop-Process -Id $NetProcess.Id
    Write-Error "Failed to init wallet."
    exit 1
}

# Sync to get initial balance
linera sync-balance --storage memory

# Get Chain ID
$WalletInfo = linera wallet show --storage memory | Out-String
$ChainId = $WalletInfo | Select-String "Chain ID: ([a-f0-9]+)" | ForEach-Object { $_.Matches.Groups[1].Value }
Write-Host "Using Chain ID: $ChainId"

# --- BUILD CONTRACTS ---
Write-Host "Building Rust Contracts..."
rustup target add wasm32-unknown-unknown
Push-Location contracts/type_arena
# Use serial build to avoid file locking on Windows
cargo build --release --target wasm32-unknown-unknown -j 1
if ($LASTEXITCODE -ne 0) { Pop-Location; Stop-Process -Id $NetProcess.Id; Write-Error "Build failed"; exit 1 }
Pop-Location

# --- DEPLOY APP ---
Write-Host "Publishing Module..."
$ContractWasm = "contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_contract.wasm"
$ServiceWasm = "contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_service.wasm"

$BytecodeOutput = linera publish-module $ContractWasm $ServiceWasm --storage memory | Out-String
$BytecodeId = $BytecodeOutput | Select-String "Module ID: ([a-f0-9]+)" | ForEach-Object { $_.Matches.Groups[1].Value }
if (-not $BytecodeId) { $BytecodeId = $BytecodeOutput | Select-String "Bytecode ID: ([a-f0-9]+)" | ForEach-Object { $_.Matches.Groups[1].Value } }

if (-not $BytecodeId) {
    Write-Error "Failed to get Module ID. Output: $BytecodeOutput"
    Stop-Process -Id $NetProcess.Id
    exit 1
}
Write-Host "Module ID: $BytecodeId"

Write-Host "Creating Application..."
$AppOutput = linera create-application $BytecodeId --json-argument "null" --storage memory | Out-String
$AppId = $AppOutput | Select-String "Application ID: ([a-f0-9]+)" | ForEach-Object { $_.Matches.Groups[1].Value }

if (-not $AppId) {
    Write-Error "Failed to get App ID. Output: $AppOutput"
    Stop-Process -Id $NetProcess.Id
    exit 1
}
Write-Host "App ID: $AppId"

# --- UPDATE FRONTEND CONFIG ---
$ConfigPath = "frontend/client/public/config.json"
$ConfigContent = @{
    chainId = $ChainId
    marketAppId = $AppId
    tokenAppId = ""
    oracleAppId = ""
}
$ConfigContent | ConvertTo-Json | Set-Content $ConfigPath
Write-Host "Updated $ConfigPath"

# --- START NODE SERVICE ---
Write-Host "Starting Node Service on port $NodeServicePort (Background)..."
$ServiceProcess = Start-Process -FilePath "linera" -ArgumentList "service", "--port", "$NodeServicePort", "--storage", "memory" -PassThru -NoNewWindow -RedirectStandardOutput "service.log" -RedirectStandardError "service_err.log"

Write-Host "---------------------------------------------------"
Write-Host "LOCAL NETWORK RUNNING!"
Write-Host "---------------------------------------------------"
Write-Host "Faucet:      http://localhost:$LocalFaucetPort"
Write-Host "Node Service: http://localhost:$NodeServicePort (GraphQL)"
Write-Host "Chain ID:    $ChainId"
Write-Host "App ID:      $AppId"
Write-Host "---------------------------------------------------"
Write-Host "Logs:"
Write-Host "  Network: net.log / net_err.log"
Write-Host "  Service: service.log / service_err.log"
Write-Host "---------------------------------------------------"
Write-Host "PRESS ENTER TO STOP THE NETWORK..."
Read-Host

Write-Host "Stopping processes..."
Stop-Process -Id $NetProcess.Id -ErrorAction SilentlyContinue
Stop-Process -Id $ServiceProcess.Id -ErrorAction SilentlyContinue
Write-Host "Done."
