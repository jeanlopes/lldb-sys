# lldb-sys — Windows Implementation

[![Crates.io](https://img.shields.io/crates/v/lldb-sys.svg)](https://crates.io/crates/lldb-sys)

Raw bindings to the [LLDB](https://lldb.llvm.org/) C++ API for use on **Windows**, built on top of the [`lldb-sys`](https://crates.io/crates/lldb-sys) crate published by the [endoli](https://github.com/endoli/lldb-sys.rs) project.

This document describes everything you need to build and use `lldb-sys` on Windows.

---

## Overview

`lldb-sys` exposes raw FFI bindings to the LLDB debugger API, which is part of the LLVM project. On Windows, LLDB ships as a shared library (`liblldb.dll`) accompanied by an import library (`liblldb.lib`). The Rust build script (`build.rs`) relies on `llvm-config` to locate headers and libraries at compile time.

---

## Prerequisites

### 1. Rust toolchain

Install the **MSVC** ABI toolchain, which is required to link against the LLVM/LLDB libraries on Windows:

```powershell
rustup install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc
```

> **Note:** The GNU toolchain (`x86_64-pc-windows-gnu`) is **not** supported because LLDB's Windows libraries are built with MSVC.

### 2. Visual Studio Build Tools

Install [Visual Studio 2022 Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) and select the **"Desktop development with C++"** workload. This provides the MSVC compiler (`cl.exe`) and the Windows SDK, both of which are needed to compile the C++ bridge in `lldb-sys`.

### 3. LLVM with LLDB

Download and install a pre-built LLVM release that includes LLDB:

- Go to the [LLVM GitHub Releases page](https://github.com/llvm/llvm-project/releases).
- Download the Windows installer, e.g. `LLVM-19.x.x-win64.exe`.
- Run the installer and choose **"Add LLVM to the system PATH"** when prompted.

Default installation path: `C:\Program Files\LLVM`

Verify the installation:

```powershell
llvm-config --version
lldb --version
```

---

## Environment Setup

The `build.rs` script calls `llvm-config` to discover include paths and library directories. Set the `LLVM_CONFIG` environment variable to point to the correct binary if it is not already on your `PATH`:

```powershell
$env:LLVM_CONFIG = "C:\Program Files\LLVM\bin\llvm-config.exe"
```

To make this permanent, add it to your user or system environment variables via **System Properties → Advanced → Environment Variables**.

### Optional: Custom library path

If `liblldb.lib` lives in a directory that is not reported by `llvm-config --libdir` (e.g. a custom LLVM build), set:

```powershell
$env:LLDB_LIB_PATH = "C:\path\to\lldb\lib"
```

### Optional: Additional include directories

For in-tree LLVM builds or custom setups where LLDB headers are in a non-standard location:

```powershell
$env:LLDB_ADDITIONAL_INCLUDE_DIRS = "C:\llvm-project\lldb\include;C:\llvm-project\build\tools\lldb\include"
```

---

## Adding `lldb-sys` to Your Project

Add the dependency to `Cargo.toml` (replace `x.y.z` with the [latest version on crates.io](https://crates.io/crates/lldb-sys)):

```toml
[dependencies]
lldb-sys = "x.y.z"
```

---

## Building on Windows

With the environment variables set, build your project normally:

```powershell
cargo build
```

The build script will:

1. Call `llvm-config --includedir` to find the LLDB/LLVM headers.
2. Call `llvm-config --libdir` to find the directory containing `liblldb.lib`.
3. Compile the C++ bridge (`src/lldb/UnityBuild.cpp`) using the `cc` crate and the MSVC toolchain.
4. Link against `liblldb.lib`, which at runtime requires `liblldb.dll` to be on the `PATH`.

### Runtime: Making `liblldb.dll` Discoverable

At runtime your application needs `liblldb.dll` (and any LLDB plug-in DLLs) to be on the Windows `PATH`. The simplest approach is to add the LLVM `bin` directory:

```powershell
$env:PATH += ";C:\Program Files\LLVM\bin"
```

---

## How the Windows Linking Works

The `build.rs` script scans the LLDB library directory for a file matching `liblldb*.lib` and extracts the link name:

```rust
if name.starts_with("liblldb") && name.ends_with(".lib") {
    // Trim the trailing ".lib" — result is "liblldb"
    return Some(name[0..name.len() - 4].into());
}
```

This causes Cargo to emit `cargo:rustc-link-lib=liblldb`, which tells the MSVC linker to link against `liblldb.lib`.

---

## Troubleshooting

| Problem | Likely cause | Fix |
|---|---|---|
| `Could not run "llvm-config --includedir"` | `llvm-config` not found | Set `LLVM_CONFIG` env var |
| `unable to locate shared library of liblldb` | No `liblldb*.lib` in the lib dir | Set `LLDB_LIB_PATH` to the correct directory |
| Linker error: `cannot open input file 'liblldb.lib'` | LLVM installed without LLDB libraries | Reinstall LLVM including the LLDB component |
| Runtime error: `The code execution cannot proceed … liblldb.dll was not found` | `liblldb.dll` not on `PATH` | Add `C:\Program Files\LLVM\bin` to `PATH` |
| Build fails with `error C2059` or similar C++ errors | Wrong compiler (GNU instead of MSVC) | Switch to the `x86_64-pc-windows-msvc` Rust target |

---

## References

- [`lldb-sys` on crates.io](https://crates.io/crates/lldb-sys)
- [endoli/lldb-sys.rs on GitHub](https://github.com/endoli/lldb-sys.rs) — upstream repository
- [LLVM Releases](https://github.com/llvm/llvm-project/releases)
- [LLDB Homepage](https://lldb.llvm.org/)

---

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
