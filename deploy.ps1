$ErrorActionPreference = "Stop"

# Add w64devkit to PATH if present
$devkitPath = Resolve-Path "$PSScriptRoot\w64devkit\bin" -ErrorAction SilentlyContinue
if ($devkitPath) {
    Write-Output "Adding w64devkit to PATH..."
    $env:PATH = "$devkitPath;" + $env:PATH
}

Write-Output "Step 1: Building WASM..."
Push-Location contracts/type_arena
cargo build --release --target wasm32-unknown-unknown
Pop-Location


Write-Output "Step 2: Publishing Bytecode..."
# Note: Using the same WASM for both if built as a single lib, or use specific bins if configured. 
# We point to the artifact in the subdirectory.
$wasm_path = "contracts/type_arena/target/wasm32-unknown-unknown/release/type_arena.wasm"
$bytecode_output = linera publish-bytecode $wasm_path $wasm_path
$bytecode_id = $bytecode_output | Select-String -Pattern "Bytecode ID: (\w+)" | ForEach-Object { $_.Matches.Groups[1].Value }
Write-Output "Bytecode ID: $bytecode_id"

Write-Output "Step 3: Creating Application..."
$app_output = linera create-application $bytecode_id --json-argument "{}"
$app_id = $app_output | Select-String -Pattern "Application ID: (\w+)" | ForEach-Object { $_.Matches.Groups[1].Value }
Write-Output "Application ID: $app_id"
Write-Output "Chain ID: (Use 'linera wallet show' to get your default chain)"

Write-Output "Step 4: Update Frontend..."
Write-Output "Please update frontend/client/.env with:"
Write-Output "VITE_TOKEN_APP_ID=$app_id"
