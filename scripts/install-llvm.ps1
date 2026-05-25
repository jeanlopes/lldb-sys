<#
.SYNOPSIS
    Installs LLVM 19 on Windows via Chocolatey and configures environment variables.

.DESCRIPTION
    Downloads and installs LLVM 19 (which includes LLDB, clang, libclang).
    Sets LLDB_SYS_PREFIX and LIBCLANG_PATH for the current user.

.EXAMPLE
    .\scripts\install-llvm.ps1
    .\scripts\install-llvm.ps1 -Version 19.1.7 -Force
#>
param(
    [string]$Version  = "19.1.7",
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# ── 1. Check Chocolatey ────────────────────────────────────────────────────
if (-not (Get-Command choco -ErrorAction SilentlyContinue)) {
    Write-Error "Chocolatey is not installed. Install it from https://chocolatey.org/install"
    exit 1
}

# ── 2. Install LLVM ────────────────────────────────────────────────────────
Write-Host "Installing LLVM $Version via Chocolatey..." -ForegroundColor Cyan

$chocoArgs = @("install", "llvm", "--version", $Version, "-y")
if ($Force) { $chocoArgs += "--force" }

& choco @chocoArgs
if ($LASTEXITCODE -ne 0) {
    Write-Error "Chocolatey failed to install LLVM (exit code $LASTEXITCODE)"
    exit 1
}

# ── 3. Locate install directory ────────────────────────────────────────────
$candidates = @(
    "C:\Program Files\LLVM",
    "C:\ProgramData\chocolatey\lib\llvm\tools\llvm"
)

$llvmPath = $null
foreach ($c in $candidates) {
    if (Test-Path (Join-Path $c "bin\lldb.exe")) {
        $llvmPath = $c
        break
    }
}

if (-not $llvmPath) {
    Write-Error "LLVM was installed but could not be located. Try setting LLDB_SYS_PREFIX manually."
    exit 1
}

Write-Host "LLVM found at: $llvmPath" -ForegroundColor Green

# ── 4. Set environment variables ───────────────────────────────────────────
[Environment]::SetEnvironmentVariable("LLDB_SYS_PREFIX",  $llvmPath,            "User")
[Environment]::SetEnvironmentVariable("LIBCLANG_PATH",    "$llvmPath\bin",       "User")
[Environment]::SetEnvironmentVariable("LLVM_CONFIG",      "$llvmPath\bin\llvm-config.exe", "User")

# Also update the current session so subsequent commands work immediately.
$env:LLDB_SYS_PREFIX = $llvmPath
$env:LIBCLANG_PATH   = "$llvmPath\bin"
$env:LLVM_CONFIG     = "$llvmPath\bin\llvm-config.exe"

Write-Host ""
Write-Host "Environment variables set (User scope):" -ForegroundColor Cyan
Write-Host "  LLDB_SYS_PREFIX = $env:LLDB_SYS_PREFIX"
Write-Host "  LIBCLANG_PATH   = $env:LIBCLANG_PATH"
Write-Host "  LLVM_CONFIG     = $env:LLVM_CONFIG"
Write-Host ""
Write-Host "Run .\scripts\validate-env.ps1 to verify the installation." -ForegroundColor Yellow
