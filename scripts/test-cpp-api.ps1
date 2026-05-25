<#
.SYNOPSIS
    Compiles and runs a minimal C++ program that calls the LLDB SB API.
    This is the "gate check" from Phase 2 of the porting plan — if this
    fails, the entire project cannot proceed.

.EXAMPLE
    .\scripts\test-cpp-api.ps1
    .\scripts\test-cpp-api.ps1 -Prefix "C:\Program Files\LLVM"
#>
param(
    [string]$Prefix = $env:LLDB_SYS_PREFIX
)

$ErrorActionPreference = "Stop"

if (-not $Prefix) { $Prefix = "C:\Program Files\LLVM" }

$include = "$Prefix\include"
$lib     = "$Prefix\lib"
$bin     = "$Prefix\bin"

$src = Join-Path $env:TEMP "lldb_test.cpp"
$exe = Join-Path $env:TEMP "lldb_test.exe"

# Write minimal test program
@"
#include <lldb/API/SBDebugger.h>
#include <lldb/API/SBTarget.h>
#include <cstdio>

int main() {
    lldb::SBDebugger::Initialize();
    auto dbg = lldb::SBDebugger::Create(false);
    if (!dbg.IsValid()) {
        fprintf(stderr, "FAIL: SBDebugger::Create returned invalid debugger\n");
        return 1;
    }
    const char* version = lldb::SBDebugger::GetVersionString();
    printf("OK: LLDB version: %s\n", version ? version : "(null)");
    lldb::SBDebugger::Destroy(dbg);
    lldb::SBDebugger::Terminate();
    return 0;
}
"@ | Set-Content $src -Encoding UTF8

Write-Host "Compiling minimal SB API test..." -ForegroundColor Cyan

# Try clang-cl first, then cl.exe
$compiler = $null
if (Test-Path "$bin\clang-cl.exe") {
    $compiler = "$bin\clang-cl.exe"
    $compileArgs = @(
        "/EHsc", "/MD",
        "/I$include",
        "/Fe$exe",
        $src,
        "/link", "/LIBPATH:$lib", "liblldb.lib"
    )
} elseif (Get-Command cl -ErrorAction SilentlyContinue) {
    $compiler = "cl.exe"
    $compileArgs = @(
        "/EHsc", "/MD",
        "/I$include",
        "/Fe$exe",
        $src,
        "/link", "/LIBPATH:$lib", "liblldb.lib"
    )
} else {
    Write-Error "No C++ compiler found. Install Visual Studio Build Tools or ensure clang-cl is in PATH."
    exit 1
}

Write-Host "  Compiler: $compiler" -ForegroundColor Gray
& $compiler @compileArgs

if ($LASTEXITCODE -ne 0) {
    Write-Error "Compilation FAILED (exit $LASTEXITCODE).  Check that LLDB headers and liblldb.lib are present."
    exit 1
}

Write-Host "Compilation OK.  Running test..." -ForegroundColor Cyan

# Add LLDB bin to PATH so the DLL is found at runtime
$env:PATH = "$bin;$env:PATH"
& $exe

if ($LASTEXITCODE -ne 0) {
    Write-Error "Runtime test FAILED.  The LLDB SB API is not functional."
    exit 1
}

Write-Host ""
Write-Host "Phase 2 gate check PASSED.  LLDB SB API works on this machine." -ForegroundColor Green
Write-Host "You can proceed with  cargo build -p lldb-sys" -ForegroundColor Green
