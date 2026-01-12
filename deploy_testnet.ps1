# Type Arena - Testnet Conway Deployer (PowerShell)
# Use this script to deploy your game to the public Linera Testnet.

$ErrorActionPreference = "Stop"

$FAUCET_URL = "https://faucet.testnet-conway.linera.net"

Write-Host "Initializing Wallet against Testnet Conway..."
$WalletPath = "$env:USERPROFILE\.config\linera\wallet.json"

if (-not (Test-Path $WalletPath)) {
    linera wallet init --faucet $FAUCET_URL
} else {
    Write-Host "Wallet found. Using existing wallet."
}

# Sync balance
linera sync-balance

# Get Chain ID
$WalletInfo = linera wallet show | Out-String
$ChainId = $WalletInfo | Select-String "Chain ID: ([a-f0-9]+)" | ForEach-Object { $_.Matches.Groups[1].Value }
Write-Host "Deploying from Chain: $ChainId"

Write-Host "Cleaning and Building Rust Contracts..."
rustup target add wasm32-unknown-unknown
Push-Location contracts/type_arena
cargo clean
# Use -j 1 to avoid file locking issues on Windows
cargo build --release --target wasm32-unknown-unknown -j 1
Pop-Location

Write-Host "Publishing Module to Testnet..."
$ContractWasm = "contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_contract.wasm"
$ServiceWasm = "contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena_service.wasm"

# Use 'publish-module' for Linera v0.16.0+
$BytecodeOutput = linera publish-module $ContractWasm $ServiceWasm | Out-String
# Output format: "Module ID: <ID>"
$BytecodeId = $BytecodeOutput | Select-String "Module ID: ([a-f0-9]+)" | ForEach-Object { $_.Matches.Groups[1].Value }

if (-not $BytecodeId) {
    # Fallback to check for "Bytecode ID" just in case
    $BytecodeId = $BytecodeOutput | Select-String "Bytecode ID: ([a-f0-9]+)" | ForEach-Object { $_.Matches.Groups[1].Value }
}

if (-not $BytecodeId) {
    Write-Error "Failed to parse Module ID from output: $BytecodeOutput"
}

Write-Host "Module ID: $BytecodeId"

Write-Host "Creating Application on Testnet..."
# Using "null" for instantiation argument ()
$AppOutput = linera create-application $BytecodeId --json-argument "null" | Out-String
$AppId = $AppOutput | Select-String "Application ID: ([a-f0-9]+)" | ForEach-Object { $_.Matches.Groups[1].Value }

Write-Host "--------------------------------------------------------"
Write-Host "DEPLOYMENT SUCCESSFUL!"
Write-Host "--------------------------------------------------------"
Write-Host "Application ID: $AppId"
Write-Host "Chain ID:       $ChainId"
Write-Host "--------------------------------------------------------"

# Auto-update config.json
$ConfigPath = "frontend/client/public/config.json"
if (Test-Path $ConfigPath) {
    $ConfigContent = Get-Content $ConfigPath -Raw | ConvertFrom-Json
    $ConfigContent.marketAppId = $AppId
    $ConfigContent.chainId = $ChainId
    $ConfigContent | ConvertTo-Json -Depth 4 | Set-Content $ConfigPath
    Write-Host "Updated $ConfigPath with new credentials."
} else {
    Write-Host "WARNING: config.json not found at $ConfigPath"
}

Write-Host "Next Steps:"
Write-Host "1. cd frontend/client"
Write-Host "2. npm install"
Write-Host "3. npm run dev"
