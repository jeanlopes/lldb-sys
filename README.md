# lldb-rs

Safe Rust bindings for LLDB 19 — **Windows only** (`x86_64-pc-windows-msvc`).

## Crates

| Crate | Description |
|---|---|
| `lldb-build` | Build helper: LLVM/LLDB discovery for all platforms |
| `lldb-sys` | Raw FFI bindings (unsafe) |
| `lldb-safe` | Safe Rust wrappers (start here) |

## Quick Start

### 1. Install LLVM 19

```powershell
.\scripts\install-llvm.ps1
```

Or manually via winget:

```powershell
winget install LLVM.LLVM --version 19.1.7
```

> The official LLVM Windows installer does **not** include LLDB C++ API headers (`lldb/API/SB*.h`).
> The install script sets up the virtual prefix `C:\lldb-dev` which combines headers from the
> LLVM GitHub release with the binaries from the installer. See [scripts/install-llvm.ps1](scripts/install-llvm.ps1).

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
cargo build
```

No environment variables are required if `C:\lldb-dev` exists (created by the install script).

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
| `LLDB_SYS_PREFIX` | Root of LLVM install (default: `C:\lldb-dev` or `C:\Program Files\LLVM`) |
| `LLVM_SYS_PREFIX` | Alias for `LLDB_SYS_PREFIX` |
| `LLVM_CONFIG` | Path to `llvm-config` binary (optional, advisory only) |
| `LIBCLANG_PATH` | Directory containing `libclang.dll` (auto-detected by `build.rs`) |
| `LLDB_DLL_DIR` | Runtime DLL directory (baked in by `build.rs`; rarely needed manually) |

All variables are optional when `C:\lldb-dev` is present.

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
cargo test -p lldb-safe -- --test-threads=1
```

No environment variables needed. `build.rs` copies `liblldb.dll` and `python310.dll` next to the
test binary automatically.

> Tests must run single-threaded (`--test-threads=1`) because LLDB's
> `SBDebugger::Initialize` / `Terminate` are global and not reentrant.

## Contributing

See [`docs/CONTRIBUTING.md`](docs/CONTRIBUTING.md) (TODO).

Known hard problems:
- Callbacks / event listeners across FFI (function pointers + userdata)
- Structured exceptions on Windows (SEH vs C++ exceptions)
- UTF-16 paths on Windows (LLDB internals expect UTF-8)
