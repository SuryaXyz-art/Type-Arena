$ErrorActionPreference = "Stop"

Write-Output "Setting up environment for Linera CLI installation..."

# 1. Add w64devkit to PATH (Essential for 'dlltool.exe', gcc, etc.)
$devkitPath = Resolve-Path "$PSScriptRoot\w64devkit\bin" -ErrorAction SilentlyContinue
if ($devkitPath) {
    Write-Output "Found w64devkit, adding to PATH..."
    $env:PATH = "$devkitPath;" + $env:PATH
}
else {
    Write-Warning "w64devkit not found in project root! compilation might fail."
}

# 2. Add protoc to PATH (Essential for compiling Linera SDK)
$protocPath = Resolve-Path "$PSScriptRoot\protoc\bin" -ErrorAction SilentlyContinue
if ($protocPath) {
    Write-Output "Found protoc, adding to PATH..."
    $env:PATH = "$protocPath;" + $env:PATH
}

Write-Output "PATH configured. Installing Linera Service..."
Write-Output "Current PATH suffix: $env:PATH"

# 3. Install Linera
# We install `linera-service` (the node/client) and `linera-storage-service` (for persistent storage)
cargo install linera-service linera-storage-service --features storage-service

Write-Output "---------------------------------------------------"
Write-Output "Installation Complete!"
Write-Output "Please ensure '%USERPROFILE%\.cargo\bin' is in your System PATH."
Write-Output "You can now run '.\deploy.ps1' to deploy your application."
