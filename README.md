# lldb-rs

Safe Rust bindings for LLDB 19 — **Windows-first**, with Linux and macOS support.

## Crates

| Crate | Description |
|---|---|
| `lldb-build` | Build helper: LLVM/LLDB discovery for all platforms |
| `lldb-sys` | Raw FFI bindings (unsafe) |
| `lldb-safe` | Safe Rust wrappers (start here) |

## Quick Start

### 1. Install LLVM 19

**Windows (Chocolatey):**
```powershell
.\scripts\install-llvm.ps1
```

**Linux:**
```bash
wget https://apt.llvm.org/llvm.sh && chmod +x llvm.sh && sudo ./llvm.sh 19
sudo apt-get install -y liblldb-19-dev lldb-19
export LLDB_SYS_PREFIX=/usr/lib/llvm-19
export LIBCLANG_PATH=/usr/lib/llvm-19/lib
```

**macOS:**
```bash
brew install llvm@19
export LLDB_SYS_PREFIX=/opt/homebrew/opt/llvm@19
export LIBCLANG_PATH=/opt/homebrew/opt/llvm@19/lib
```

### 2. Validate the environment

```powershell
.\scripts\validate-env.ps1        # Windows
```

### 3. Gate check: verify the C++ SB API works

```powershell
.\scripts\test-cpp-api.ps1        # Windows
```

If this fails, `cargo build` will also fail — fix it first.

### 4. Build

```powershell
$env:LLDB_SYS_PREFIX = "C:\Program Files\LLVM"
$env:LIBCLANG_PATH   = "C:\Program Files\LLVM\bin"
cargo build
```

### 5. Use

```rust
use lldb_safe::Debugger;

fn main() {
    // Windows: set DLL search path before any LLDB code runs.
    let dll_dir = std::env::var("LLDB_DLL_DIR")
        .unwrap_or_else(|_| r"C:\Program Files\LLVM\bin".into());
    lldb_safe::set_lldb_dll_dir(dll_dir.as_ref());

    Debugger::initialize();

    let dbg = Debugger::create(false).unwrap();
    println!("{}", Debugger::version_string());

    let target = dbg.create_target_simple("C:\\path\\to\\my.exe").unwrap();
    let bp = target.breakpoint_by_name("main", None).unwrap();
    println!("Breakpoint {} set ({} locations)", bp.id(), bp.num_locations());

    let process = target
        .launch(&[], &[], None, true)
        .expect("launch failed");

    println!("Process state: {:?}", process.state());

    Debugger::terminate();
}
```

## Environment Variables

| Variable | Description |
|---|---|
| `LLDB_SYS_PREFIX` | Root of LLVM install (e.g. `C:\Program Files\LLVM`) |
| `LLVM_SYS_PREFIX` | Alias for `LLDB_SYS_PREFIX` |
| `LLVM_CONFIG` | Path to `llvm-config` binary |
| `LIBCLANG_PATH` | Directory containing `libclang.dll` (needed by bindgen) |
| `LLDB_DLL_DIR` | Runtime DLL directory (Windows; set automatically by build.rs) |

## Windows-specific Notes

### Runtime DLL
`liblldb.dll` must be findable at runtime. Call `set_lldb_dll_dir()` before
any LLDB code, or copy `liblldb.dll` next to your executable.

### CRT compatibility
`liblldb.dll` from the official LLVM installer uses the **dynamic MSVC CRT**
(`/MD`). The C++ wrapper layer is compiled with `/MD` to match. Do not mix
with `/MT` builds.

### MSVC environment
Ensure you run `cargo build` from a **Developer Command Prompt** (or run
`vcvarsall.bat x64` first) so that `cl.exe` and `link.exe` are on PATH.
The CI uses `ilammy/msvc-dev-cmd@v1` for this.

## Architecture

```
┌─────────────────────────┐
│       lldb-safe          │  Safe Rust API (Debugger, Target, Process, …)
└───────────┬─────────────┘
            │  calls
┌───────────▼─────────────┐
│       lldb-sys           │  Raw FFI (bindgen-generated bindings)
└───────────┬─────────────┘
            │  links
┌───────────▼─────────────┐
│   lldb_c_wrapper.lib     │  Thin C++ → C adapter (compiled by build.rs)
└───────────┬─────────────┘
            │  links
┌───────────▼─────────────┐
│      liblldb.dll         │  Official LLDB 19 (from LLVM installer)
└─────────────────────────┘
```

## Running Tests

```powershell
$env:LLDB_SYS_PREFIX = "C:\Program Files\LLVM"
$env:LIBCLANG_PATH   = "C:\Program Files\LLVM\bin"
# Add DLL to PATH so the test runner finds it at runtime
$env:PATH = "C:\Program Files\LLVM\bin;$env:PATH"

cargo test -p lldb-safe -- --test-threads=1
```

> Tests must run single-threaded (`--test-threads=1`) because LLDB's
> `SBDebugger::Initialize` / `Terminate` are global and not reentrant.

## Contributing

See [`docs/CONTRIBUTING.md`](docs/CONTRIBUTING.md) (TODO).

Known hard problems:
- Callbacks / event listeners across FFI (function pointers + userdata)
- Structured exceptions on Windows vs POSIX signals
- UTF-16 paths on Windows (LLDB internals expect UTF-8)
