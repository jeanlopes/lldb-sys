<#
.SYNOPSIS
    Validates that the LLDB / LLVM build environment is correctly configured.

.DESCRIPTION
    Checks for: LLVM binaries, required DLLs, symbol exports, Rust toolchain,
    Visual Studio build tools, and the LIBCLANG_PATH needed by bindgen.

.EXAMPLE
    .\scripts\validate-env.ps1
#>
param()

$ErrorActionPreference = "Continue"
$allOk = $true

function Check([string]$label, [scriptblock]$test) {
    try {
        $result = & $test
        if ($result) {
            Write-Host "  [OK]  $label" -ForegroundColor Green
        } else {
            Write-Host "  [!!]  $label" -ForegroundColor Red
            $script:allOk = $false
        }
    } catch {
        Write-Host "  [!!]  $label  ($($_.Exception.Message))" -ForegroundColor Red
        $script:allOk = $false
    }
}

# ── LLVM prefix ────────────────────────────────────────────────────────────
$prefix = $env:LLDB_SYS_PREFIX
if (-not $prefix) { $prefix = "C:\Program Files\LLVM" }

Write-Host ""
Write-Host "=== LLDB Build Environment Validator ===" -ForegroundColor Cyan
Write-Host "  LLDB_SYS_PREFIX = $prefix"
Write-Host "  LIBCLANG_PATH   = $env:LIBCLANG_PATH"
Write-Host ""

# ── Section 1: LLVM binaries ───────────────────────────────────────────────
Write-Host "LLVM binaries:" -ForegroundColor Yellow

Check "lldb.exe found" {
    Test-Path (Join-Path $prefix "bin\lldb.exe")
}
Check "llvm-config.exe found" {
    Test-Path (Join-Path $prefix "bin\llvm-config.exe")
}
Check "clang.exe found" {
    Test-Path (Join-Path $prefix "bin\clang.exe")
}
Check "liblldb.dll found" {
    Test-Path (Join-Path $prefix "bin\liblldb.dll")
}
Check "liblldb.lib (import lib) found" {
    Test-Path (Join-Path $prefix "lib\liblldb.lib")
}
Check "LLDB.h header found" {
    Test-Path (Join-Path $prefix "include\lldb\API\LLDB.h")
}
Check "libclang.dll found" {
    Test-Path (Join-Path $prefix "bin\libclang.dll")
}

# ── Section 2: LLDB version ────────────────────────────────────────────────
Write-Host ""
Write-Host "LLDB version:" -ForegroundColor Yellow
try {
    $v = & "$prefix\bin\lldb.exe" --version 2>&1 | Select-String "version"
    Write-Host "  $v" -ForegroundColor Gray
    Check "LLDB version >= 19" { $v -match "version 19\." }
} catch {
    Write-Host "  [!!]  Could not run lldb.exe" -ForegroundColor Red
    $allOk = $false
}

# ── Section 3: DLL exports ─────────────────────────────────────────────────
Write-Host ""
Write-Host "liblldb.dll exports:" -ForegroundColor Yellow

$dll = Join-Path $prefix "bin\liblldb.dll"
if (Test-Path $dll) {
    try {
        # Use dumpbin if VS tools are available, else use llvm-nm
        if (Get-Command dumpbin -ErrorAction SilentlyContinue) {
            $exports = dumpbin /exports $dll 2>&1
            $sbCount = ($exports | Select-String "LLDB_SB" | Measure-Object).Count
            Check "SBDebugger_Initialize exported" { $exports -match "LLDB_SBDebugger_Initialize" }
            Check "SBTarget_LaunchSimple exported"  { $exports -match "LLDB_SBTarget_LaunchSimple" }
            Write-Host "  Total LLDB_SB* exports: $sbCount" -ForegroundColor Gray
        } elseif (Get-Command llvm-nm -ErrorAction SilentlyContinue) {
            $nm = llvm-nm $dll 2>&1
            Check "SBDebugger_Initialize exported" { $nm -match "SBDebugger_Initialize" }
        } else {
            Write-Host "  [--]  dumpbin/llvm-nm not found; skipping export check" -ForegroundColor DarkYellow
        }
    } catch {
        Write-Host "  [--]  Could not inspect DLL exports: $($_.Exception.Message)" -ForegroundColor DarkYellow
    }
} else {
    Write-Host "  [!!]  liblldb.dll not found at $dll" -ForegroundColor Red
    $allOk = $false
}

# ── Section 4: MSVC / clang-cl ─────────────────────────────────────────────
Write-Host ""
Write-Host "C++ compiler:" -ForegroundColor Yellow

Check "cl.exe (MSVC) in PATH" {
    $null -ne (Get-Command cl -ErrorAction SilentlyContinue)
}
Check "clang-cl.exe in LLVM" {
    Test-Path (Join-Path $prefix "bin\clang-cl.exe")
}

# ── Section 5: Rust toolchain ──────────────────────────────────────────────
Write-Host ""
Write-Host "Rust toolchain:" -ForegroundColor Yellow

Check "rustc installed" {
    $null -ne (Get-Command rustc -ErrorAction SilentlyContinue)
}
Check "cargo installed" {
    $null -ne (Get-Command cargo -ErrorAction SilentlyContinue)
}
Check "MSVC target active" {
    (rustup show active-toolchain 2>&1) -match "msvc"
}

# ── Section 6: bindgen ─────────────────────────────────────────────────────
Write-Host ""
Write-Host "bindgen / LIBCLANG_PATH:" -ForegroundColor Yellow

Check "LIBCLANG_PATH is set" {
    -not [string]::IsNullOrEmpty($env:LIBCLANG_PATH)
}
if ($env:LIBCLANG_PATH) {
    Check "libclang.dll in LIBCLANG_PATH" {
        Test-Path (Join-Path $env:LIBCLANG_PATH "libclang.dll")
    }
}

# ── Summary ────────────────────────────────────────────────────────────────
Write-Host ""
if ($allOk) {
    Write-Host "All checks passed!  Run:  cargo build -p lldb-sys" -ForegroundColor Green
} else {
    Write-Host "Some checks FAILED.  Fix the issues above before building." -ForegroundColor Red
    Write-Host "Hint: run  .\scripts\install-llvm.ps1  to install LLVM 19." -ForegroundColor Yellow
}
Write-Host ""
