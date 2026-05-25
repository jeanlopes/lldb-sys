# Plano: lldb-sys Windows Port (LLDB 19, Fresh Start)

## Context

O objetivo é criar bindings Rust para LLDB 19 que funcionem nativamente no Windows (x86_64-pc-windows-msvc). O crate existente `endoli/lldb.rs` não tem suporte a Windows e usa arquitetura de crate único. Este projeto começa do zero com workspace multi-crate, priorizando Windows first com CI cross-platform.

**Ambiente atual:** Rust 1.95 (MSVC), sem LLVM instalado, Chocolatey disponível.

---

## Arquitetura

```
C:\workspace\lldb-sys\          ← workspace root
├── Cargo.toml                  (workspace: members = [lldb-sys, lldb-safe, lldb-build])
├── crates/
│   ├── lldb-build/             ← biblioteca de descoberta LLVM (build-dependency)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── lldb-sys/               ← FFI raw bindings
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   ├── src/
│   │   │   ├── lib.rs          (re-export bindings, #[link] attrs)
│   │   │   └── bindings.rs     (gerado por bindgen em build time)
│   │   └── wrapper/            ← C++ wrappers (a camada mais trabalhosa)
│   │       ├── include/        (headers C públicos)
│   │       └── src/            (implementações C++ → C)
│   └── lldb-safe/              ← abstrações Rust seguras
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── debugger.rs
│           ├── target.rs
│           ├── process.rs
│           └── ...
├── scripts/
│   ├── install-llvm.ps1        (choco install llvm + validação)
│   └── validate-env.ps1        (check LLVM, MSVC, bindgen)
└── .github/
    └── workflows/
        └── ci.yml              (matrix: windows-latest, ubuntu-24.04, macos-latest)
```

---

## Fase 0 — Pré-requisitos (ambiente local)

Tarefas manuais antes de qualquer código:

1. **Instalar LLVM 19** via Chocolatey:
   ```powershell
   choco install llvm --version 19.1.7
   # instala em C:\Program Files\LLVM\
   ```

2. **Validar instalação:**
   ```powershell
   lldb --version        # LLDB 19.x
   llvm-config --version # 19.x
   clang --version       # 19.x
   dumpbin /exports "C:\Program Files\LLVM\bin\liblldb.dll"
   ```
   Confirmar que `LLDB_SBDebugger_Initialize` e outros símbolos `LLDB_SB*` aparecem nos exports.

3. **Instalar Visual Studio Build Tools** (se `cl.exe` não estiver disponível):
   ```powershell
   choco install visualstudio2022buildtools
   ```

4. **Instalar bindgen CLI** (para geração manual):
   ```powershell
   cargo install bindgen-cli
   ```

---

## Fase 1 — Workspace + Crates (scaffolding)

**Arquivo: `Cargo.toml` (workspace root)**
```toml
[workspace]
members = ["crates/lldb-build", "crates/lldb-sys", "crates/lldb-safe"]
resolver = "2"
```

**`crates/lldb-build/Cargo.toml`** — sem dependências externas, só std.

**`crates/lldb-sys/Cargo.toml`**
```toml
[build-dependencies]
lldb-build = { path = "../lldb-build" }
cc = "1"
bindgen = "0.70"

[dependencies]
libc = "0.2"
```

**`crates/lldb-safe/Cargo.toml`**
```toml
[dependencies]
lldb-sys = { path = "../lldb-sys" }
```

---

## Fase 2 — lldb-build: Descoberta LLVM no Windows

**Arquivo: `crates/lldb-build/src/lib.rs`**

Função pública `find_lldb() -> LldbConfig` com estratégia em cascata:

```
1. LLDB_SYS_PREFIX   (env var)
2. LLVM_SYS_PREFIX   (env var)
3. LLVM_CONFIG       (env var, caminho para llvm-config.exe)
4. Registro Windows: HKLM\SOFTWARE\LLVM\LLVM → InstallDir
5. Chocolatey:       C:\ProgramData\chocolatey\lib\llvm\tools\llvm
6. PATH bem-conhecidos: C:\Program Files\LLVM
7. llvm-config no PATH (fallback Unix-compat)
```

`LldbConfig` contém:
- `include_dir: PathBuf`  (para `-I` no cc)
- `lib_dir: PathBuf`      (para rustc-link-search)
- `lib_name: String`      (`liblldb` no Windows, `lldb` no Unix)
- `version: String`

**Registro Windows** (único no projeto, não existe em endoli/lldb.rs):
```rust
#[cfg(target_os = "windows")]
fn find_via_registry() -> Option<PathBuf> {
    use std::process::Command;
    // reg query HKLM\SOFTWARE\LLVM\LLVM /v InstallDir
    let out = Command::new("reg")
        .args(["query", r"HKLM\SOFTWARE\LLVM\LLVM", "/v", "InstallDir"])
        .output().ok()?;
    // parse output para extrair o path
}
```

---

## Fase 3 — C++ Wrapper Layer (a parte mais trabalhosa)

LLDB 19 expõe API C++ apenas (classes SB*). Precisamos de wrappers C para o bindgen conseguir trabalhar.

**Estratégia MVP** — implementar primeiro as classes core:

| Classe C++ | Arquivo wrapper | Prioridade |
|---|---|---|
| SBDebugger | wrapper/src/SBDebugger.cpp | P0 |
| SBTarget | wrapper/src/SBTarget.cpp | P0 |
| SBProcess | wrapper/src/SBProcess.cpp | P0 |
| SBThread | wrapper/src/SBThread.cpp | P0 |
| SBFrame | wrapper/src/SBFrame.cpp | P0 |
| SBBreakpoint | wrapper/src/SBBreakpoint.cpp | P1 |
| SBValue | wrapper/src/SBValue.cpp | P1 |
| SBError | wrapper/src/SBError.cpp | P0 |
| SBFileSpec | wrapper/src/SBFileSpec.cpp | P1 |

**Padrão de wrapper** (cada arquivo):
```cpp
// wrapper/src/SBDebugger.cpp
#include "lldb/API/SBDebugger.h"
#include "../include/lldb_c.h"

extern "C" {
    void LLDB_SBDebugger_Initialize() {
        lldb::SBDebugger::Initialize();
    }
    void LLDB_SBDebugger_Terminate() {
        lldb::SBDebugger::Terminate();
    }
    SBDebuggerRef LLDB_SBDebugger_Create(bool async) {
        auto* dbg = new lldb::SBDebugger(lldb::SBDebugger::Create(async));
        return reinterpret_cast<SBDebuggerRef>(dbg);
    }
    void LLDB_SBDebugger_Destroy(SBDebuggerRef ref) {
        delete reinterpret_cast<lldb::SBDebugger*>(ref);
    }
    // ...
}
```

**Header C público** (`wrapper/include/lldb_c.h`):
```c
typedef void* SBDebuggerRef;
typedef void* SBTargetRef;
// ...
void LLDB_SBDebugger_Initialize(void);
SBDebuggerRef LLDB_SBDebugger_Create(int async);
// ...
```

---

## Fase 4 — build.rs do lldb-sys

**`crates/lldb-sys/build.rs`**:

```rust
fn main() {
    let lldb = lldb_build::find_lldb().expect("LLDB 19 not found");

    // 1. Compilar os wrappers C++
    cc::Build::new()
        .cpp(true)
        .std("c++17")           // LLDB 19 requer C++17
        .warnings(false)
        .include("wrapper/include")
        .include(&lldb.include_dir)
        .files(glob("wrapper/src/*.cpp"))
        // Windows: /EHsc para exceptions, /MD para CRT dinâmico
        .flag_if_supported("/EHsc")
        .compile("lldb_c_wrapper");

    // 2. Linkar liblldb dinâmico
    println!("cargo:rustc-link-search=native={}", lldb.lib_dir.display());
    println!("cargo:rustc-link-lib=dylib={}", lldb.lib_name);

    // 3. Gerar bindings com bindgen
    let bindings = bindgen::Builder::default()
        .header("wrapper/include/lldb_c.h")
        .clang_arg(format!("-I{}", lldb.include_dir.display()))
        .allowlist_function("LLDB_.*")
        .allowlist_type("SB.*Ref")
        // Opaque: std::* não pode cruzar FFI
        .opaque_type("std::.*")
        .generate()
        .expect("bindgen failed");

    bindings.write_to_file("src/bindings.rs").unwrap();
}
```

**Windows-specific no linking**: `liblldb.dll` em LLVM 19 Windows.
O import lib é `liblldb.lib` em `C:\Program Files\LLVM\lib\`.

---

## Fase 5 — lldb-safe: Wrappers Safe

**`crates/lldb-safe/src/debugger.rs`**:
```rust
use lldb_sys::*;

pub struct Debugger(SBDebuggerRef);

impl Debugger {
    pub fn initialize() { unsafe { LLDB_SBDebugger_Initialize() } }
    pub fn terminate() { unsafe { LLDB_SBDebugger_Terminate() } }
    pub fn create(async_mode: bool) -> Self {
        Self(unsafe { LLDB_SBDebugger_Create(async_mode as _) })
    }
}

impl Drop for Debugger {
    fn drop(&mut self) { unsafe { LLDB_SBDebugger_Destroy(self.0) } }
}

// LLDB SBDebugger NÃO é thread-safe — não impl Send/Sync por padrão
```

**Windows runtime quirk** — adicionar em `lldb-safe/src/lib.rs`:
```rust
#[cfg(windows)]
pub fn set_lldb_dll_dir(path: &std::path::Path) {
    use std::os::windows::ffi::OsStrExt;
    let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    unsafe { windows_sys::Win32::System::LibraryLoader::SetDllDirectoryW(wide.as_ptr()) };
}
```

---

## Fase 6 — Scripts PowerShell

**`scripts/install-llvm.ps1`**:
```powershell
choco install llvm --version 19.1.7 -y
$llvmPath = "C:\Program Files\LLVM"
[Environment]::SetEnvironmentVariable("LLDB_SYS_PREFIX", $llvmPath, "User")
[Environment]::SetEnvironmentVariable("LIBCLANG_PATH", "$llvmPath\bin", "User")
```

**`scripts/validate-env.ps1`**:
```powershell
# Verifica LLVM, MSVC, rust toolchain, exports da DLL
lldb --version
llvm-config --version
dumpbin /exports "$env:LLDB_SYS_PREFIX\bin\liblldb.dll" | Select-String "LLDB_SB"
```

---

## Fase 7 — CI (.github/workflows/ci.yml)

```yaml
jobs:
  build:
    strategy:
      matrix:
        include:
          - os: windows-latest
            llvm_install: "choco install llvm --version 19.1.7 -y"
            lldb_prefix: "C:\\Program Files\\LLVM"
          - os: ubuntu-24.04
            llvm_install: "sudo apt-get install -y llvm-19 liblldb-19-dev"
            lldb_prefix: "/usr/lib/llvm-19"
          - os: macos-latest
            llvm_install: "brew install llvm@19"
            lldb_prefix: "/opt/homebrew/opt/llvm@19"
    steps:
      - uses: actions/checkout@v4
      - run: ${{ matrix.llvm_install }}
      - run: cargo build
        env:
          LLDB_SYS_PREFIX: ${{ matrix.lldb_prefix }}
      - run: cargo test
        env:
          LLDB_SYS_PREFIX: ${{ matrix.lldb_prefix }}
```

---

## Ordem de Implementação (sprints)

| Sprint | Entregável | Critério de sucesso |
|---|---|---|
| 1 | Workspace + lldb-build | `cargo build -p lldb-build` passa |
| 2 | Script install-llvm.ps1 + validação LLVM | `dumpbin` mostra símbolos SB* |
| 3 | Wrapper C++ MVP (SBDebugger + SBError) | Compila com `cl.exe` manualmente |
| 4 | build.rs + bindings.rs | `cargo build -p lldb-sys` passa |
| 5 | lldb-safe MVP | `Debugger::initialize()` não crasha |
| 6 | Teste de integração | Attach a processo dummy, lê stack |
| 7 | CI multi-plataforma | Green em Windows, Linux, macOS |
| 8 | SBTarget, SBProcess, SBThread, SBFrame | Stepping funciona |
| 9 | SBBreakpoint, SBValue | Leitura de variáveis funciona |
| 10 | lldb-safe completo + docs | API pública estável |

---

## Verificação End-to-End

```rust
// crates/lldb-safe/tests/integration.rs
#[test]
fn test_debugger_initialize() {
    lldb_safe::set_lldb_dll_dir("C:\\Program Files\\LLVM\\bin".as_ref()); // Windows only
    Debugger::initialize();
    let dbg = Debugger::create(false);
    drop(dbg);
    Debugger::terminate();
}
```

Rodar com:
```powershell
$env:LLDB_SYS_PREFIX = "C:\Program Files\LLVM"
cargo test -p lldb-safe
```

---

## Riscos Conhecidos

| Risco | Mitigação |
|---|---|
| `liblldb.dll` não exporta todos os símbolos SB* | Verificar com `dumpbin` antes — se faltar, precisará compilar LLDB do fonte com `-DLLDB_API_LIBLLDB_EXPORTS=ON` |
| CRT mismatch (MSVC runtime) | Usar `/MD` no `cc::Build` para garantir mesmo CRT dinâmico da liblldb |
| bindgen + clang-cl quebrando em headers C++17 | Definir `LIBCLANG_PATH` para `llvm19/bin`, usar `clang-cl` como compiler para o cc |
| `liblldb.dll` não encontrada em runtime | Chamar `SetDllDirectoryW` antes de qualquer uso, ou copiar DLL para output dir |
| LLDB 19 API diverge de versões anteriores | A API SB é estável desde LLDB 3.x — breaking changes são raros |