$ErrorActionPreference = "Stop"

echo "Step 1: Building WASM..."
cargo build --release --target wasm32-unknown-unknown

echo "Step 2: Publishing Bytecode..."
$bytecode_output = linera publish-bytecode target/wasm32-unknown-unknown/release/type_arena_contract.wasm target/wasm32-unknown-unknown/release/type_arena_service.wasm
$bytecode_id = $bytecode_output | Select-String -Pattern "Bytecode ID: (\w+)" | ForEach-Object { $_.Matches.Groups[1].Value }
echo "Bytecode ID: $bytecode_id"

echo "Step 3: Creating Application..."
$app_output = linera create-application $bytecode_id --json-argument "{}"
$app_id = $app_output | Select-String -Pattern "Application ID: (\w+)" | ForEach-Object { $_.Matches.Groups[1].Value }
echo "Application ID: $app_id"
echo "Chain ID: (Use your default chain)"

echo "Step 4: Update Frontend..."
echo "Please update frontend/client/src/services/LineraService.ts with:"
echo "APPLICATION_ID = `"$app_id`""
