# Plano: lldb-sys Windows Port (LLDB 19, Fresh Start)

## Context

O objetivo é criar bindings Rust para LLDB 19 que funcionem nativamente no Windows (x86_64-pc-windows-msvc). O crate existente `endoli/lldb.rs` não tem suporte a Windows e usa arquitetura de crate único. Este projeto começa do zero com workspace multi-crate, priorizando Windows first com CI cross-platform.

**Ambiente atual:** Rust 1.95 (MSVC), LLVM 19.1.7 instalado via winget, Python 3.10.11.

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
│   │   │   └── bindings.rs     (gerado por bindgen em build time, em OUT_DIR)
│   │   └── wrapper/            ← C++ wrappers (camada de adaptação C++ → C)
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
│   ├── install-llvm.ps1        (winget install llvm + validação)
│   └── validate-env.ps1        (check LLVM, MSVC, bindgen)
└── .github/
    └── workflows/
        └── ci.yml              (matrix: windows-latest, ubuntu-24.04, macos-latest)
```

---

## Fase 0 — Pré-requisitos (ambiente local)

### Windows

1. **Instalar LLVM 19** via winget (Chocolatey não está disponível):
   ```powershell
   winget install LLVM.LLVM --version 19.1.7
   # instala em C:\Program Files\LLVM\
   ```
   > O instalador oficial **não inclui** os headers C++ do LLDB (`lldb/API/SB*.h`).
   > É necessário baixá-los separadamente — veja passo 2.

2. **Baixar os headers LLDB** (sparse-checkout do repositório LLVM):
   ```powershell
   git clone --filter=blob:none --no-checkout --depth=1 --branch llvmorg-19.1.7 `
       https://github.com/llvm/llvm-project.git C:\llvm-headers-tmp
   cd C:\llvm-headers-tmp
   git sparse-checkout init --cone
   git sparse-checkout set lldb/include
   git checkout llvmorg-19.1.7
   ```
   Depois criar o prefix virtual:
   ```powershell
   New-Item -ItemType Directory "C:\lldb-dev\include","C:\lldb-dev\lib","C:\lldb-dev\bin" -Force
   Copy-Item "C:\llvm-headers-tmp\lldb\include\lldb" "C:\lldb-dev\include\lldb" -Recurse
   Copy-Item "C:\Program Files\LLVM\lib\liblldb.lib"  "C:\lldb-dev\lib\"
   Copy-Item "C:\Program Files\LLVM\bin\liblldb.dll"  "C:\lldb-dev\bin\"
   Copy-Item "C:\Program Files\LLVM\bin\libclang.dll" "C:\lldb-dev\bin\"
   Copy-Item "C:\Program Files\LLVM\bin\clang.exe"    "C:\lldb-dev\bin\"
   Copy-Item "C:\Program Files\LLVM\bin\clang-cl.exe" "C:\lldb-dev\bin\"
   ```

3. **Instalar Python 3.10** — `liblldb.dll` depende de `python310.dll`:
   ```powershell
   winget install Python.Python.3.10
   ```

4. **Visual Studio Build Tools 2022** — para `cl.exe` / `link.exe`:
   Disponível em https://visualstudio.microsoft.com/visual-cpp-build-tools/
   Abrir o terminal como "Developer PowerShell for VS 2022" para build de release.

5. **Validar:**
   ```powershell
   .\scripts\validate-env.ps1   # deve mostrar "All checks passed!"
   .\scripts\test-cpp-api.ps1   # deve mostrar "Phase 2 gate check PASSED"
   ```

### Variáveis de ambiente (opcionais — o build.rs detecta automaticamente)

| Variável | Valor | Quando necessária |
|---|---|---|
| `LLDB_SYS_PREFIX` | `C:\lldb-dev` | Sobrescreve auto-detecção |
| `LIBCLANG_PATH` | `C:\lldb-dev\bin` | Sobrescreve auto-detecção do bindgen |

---

## Fase 1 — Workspace + Crates (scaffolding)

**Arquivo: `Cargo.toml` (workspace root)**
```toml
[workspace]
members = ["crates/lldb-build", "crates/lldb-sys", "crates/lldb-safe"]
resolver = "2"
```

**`crates/lldb-sys/Cargo.toml`**
```toml
[build-dependencies]
lldb-build = { path = "../lldb-build" }
cc = "1"
bindgen = "0.71"   # nota: 0.71+ quebrou use_core(bool) — use use_core() sem arg

[dependencies]
libc = "0.2"
```

---

## Fase 2 — lldb-build: Descoberta LLVM no Windows

**Arquivo: `crates/lldb-build/src/lib.rs`**

Função pública `find_lldb() -> Result<LldbConfig, String>` com estratégia em cascata:

```
1. LLDB_SYS_PREFIX   (env var)
2. LLVM_SYS_PREFIX   (env var)
3. LLVM_CONFIG       (env var, caminho para llvm-config.exe)
4. Registro Windows: HKLM\SOFTWARE\LLVM\LLVM → InstallDir
5. Chocolatey:       C:\ProgramData\chocolatey\lib\llvm\tools\llvm
6. Scoop:            %USERPROFILE%\scoop\apps\llvm\current
7. PATH bem-conhecidos: C:\lldb-dev  (prefix virtual), C:\Program Files\LLVM, C:\LLVM
8. llvm-config no PATH (fallback Unix-compat: llvm-config-19, llvm-config19, llvm-config)
```

`LldbConfig` contém:
- `prefix: PathBuf`    (raiz da instalação)
- `include_dir: PathBuf`  (para `-I` no cc — deve conter `lldb/API/LLDB.h`)
- `lib_dir: PathBuf`      (para rustc-link-search)
- `bin_dir: PathBuf`      (para lldb.exe e DLLs)
- `lib_name: String`      (`liblldb` no Windows, `lldb-19` ou `lldb` no Linux, `lldb` no macOS)
- `version: String`

A validação principal em `from_prefix()` é:
```rust
let lldb_header = include_dir.join("lldb").join("API").join("LLDB.h");
if !lldb_header.exists() { return None; }
```

**Registro Windows:**
```rust
#[cfg(target_os = "windows")]
fn try_registry() -> Option<LldbConfig> {
    let out = Command::new("reg")
        .args(["query", r"HKLM\SOFTWARE\LLVM\LLVM", "/v", "InstallDir"])
        .output().ok()?;
    parse_reg_value(&String::from_utf8_lossy(&out.stdout))
        .and_then(|p| LldbConfig::from_prefix(p))
}
```

---

## Fase 3 — C++ Wrapper Layer

LLDB 19 expõe API C++ apenas (classes SB*). Wrappers C permitem que o bindgen trabalhe.

**Classes implementadas:**

| Classe C++ | Arquivo wrapper | Prioridade |
|---|---|---|
| SBDebugger | wrapper/src/SBDebugger.cpp | P0 ✅ |
| SBTarget | wrapper/src/SBTarget.cpp | P0 ✅ |
| SBProcess | wrapper/src/SBProcess.cpp | P0 ✅ |
| SBThread | wrapper/src/SBThread.cpp | P0 ✅ |
| SBFrame | wrapper/src/SBFrame.cpp | P0 ✅ |
| SBBreakpoint | wrapper/src/SBBreakpoint.cpp | P1 ✅ |
| SBValue | wrapper/src/SBValue.cpp | P1 ✅ |
| SBError | wrapper/src/SBError.cpp | P0 ✅ |
| SBFileSpec | wrapper/src/SBFileSpec.cpp | P1 ✅ |
| SBListener | wrapper/src/SBListener.cpp | P1 ✅ |

**Padrão de wrapper** (cada arquivo):
```cpp
#include "lldb/API/SBDebugger.h"
#include "../include/lldb_c.h"

extern "C" {
    void LLDB_SBDebugger_Initialize(void) {
        lldb::SBDebugger::Initialize();
    }
    SBDebuggerRef LLDB_SBDebugger_Create(bool async) {
        return reinterpret_cast<SBDebuggerRef>(
            new lldb::SBDebugger(lldb::SBDebugger::Create(async)));
    }
    // ...
}
```

---

## Fase 4 — build.rs do lldb-sys

**`crates/lldb-sys/build.rs`** — resumo do que faz:

### 1. Compilar wrappers C++
```rust
let mut build = cc::Build::new();
build.cpp(true).std("c++17").warnings(false)
     .include("wrapper/include")
     .include(&lldb.include_dir);

if build.get_compiler().is_like_msvc() {
    build.flag("/EHsc").flag("/MD");  // exception handling + CRT dinâmico
}
build.compile("lldb_c_wrapper");
```

### 2. Linkar liblldb dinâmico
```rust
println!("cargo:rustc-link-search=native={}", lldb.lib_dir.display());
println!("cargo:rustc-link-lib=dylib={}", lldb.lib_name);
```

### 3. Copiar DLLs para o diretório de saída (Windows)
`liblldb.dll` depende de `python310.dll`. Como ambas são import implícitas, o
Windows as carrega **antes de `main()` rodar**, antes de qualquer código Rust.
A solução é copiar as DLLs para `target/debug/deps/` e `target/debug/`:

```rust
#[cfg(target_os = "windows")]
fn copy_runtime_dlls_windows(lldb_bin: &Path) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // OUT_DIR = target/<profile>/build/lldb-sys-<hash>/out
    // subir 3 níveis chega em target/<profile>
    let profile_dir = out_dir.parent()?.parent()?.parent()?;

    for dest in &[profile_dir.join("deps"), profile_dir.clone()] {
        copy(lldb_bin.join("liblldb.dll"), dest.join("liblldb.dll"));
        if let Some(py) = find_python310_dll() {
            copy(&py, dest.join("python310.dll"));
        }
    }
}
```

### 4. Descoberta do libclang para bindgen
Cascata: `LIBCLANG_PATH` env var → `lldb.bin_dir` (se tiver libclang.dll) →
`C:\Program Files\LLVM\bin` (fallback Windows) → usa `lldb.bin_dir` mesmo assim.

```rust
std::env::set_var("LIBCLANG_PATH", &libclang_path);  // clang-sys lê ao carregar
```

### 5. Gerar bindings com bindgen
```rust
bindgen::Builder::default()
    .header("wrapper/include/lldb_c.h")
    .clang_arg(format!("-I{}", lldb.include_dir.display()))
    // No Windows, tell clang to parse headers as MSVC would:
    .clang_arg("--target=x86_64-pc-windows-msvc")
    .clang_arg("-fms-compatibility")
    .allowlist_function("LLDB_.*")
    .allowlist_type("LLDB.*|SB.*Ref")
    .allowlist_var("LLDB.*")
    .opaque_type("std::.*")
    .ctypes_prefix("libc")
    .layout_tests(false)
    // NOTA: bindgen 0.71+ — use_core() não aceita argumento booleano
    .generate()
```

**Windows-specific no linking**: import lib é `liblldb.lib` em `C:\Program Files\LLVM\lib\`.

---

## Fase 5 — lldb-safe: Wrappers Safe

**`crates/lldb-safe/src/debugger.rs`**:
```rust
pub struct Debugger(SBDebuggerRef);

impl Debugger {
    pub fn initialize() { unsafe { LLDB_SBDebugger_Initialize() } }
    pub fn terminate()  { unsafe { LLDB_SBDebugger_Terminate() } }
    pub fn create(async_mode: bool) -> Option<Self> {
        let r = unsafe { LLDB_SBDebugger_Create(async_mode) };
        if r.is_null() { None } else { Some(Self(r)) }
    }
}

impl Drop for Debugger {
    fn drop(&mut self) { unsafe { LLDB_SBDebugger_Destroy(self.0) } }
}
// SBDebugger não é thread-safe — sem Send/Sync
```

**Windows runtime — `lldb-safe/src/lib.rs`**:
```rust
#[cfg(windows)]
pub fn set_lldb_dll_dir(path: &std::path::Path) {
    use std::os::windows::ffi::OsStrExt;
    let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
    unsafe { windows_sys::Win32::System::LibraryLoader::SetDllDirectoryW(wide.as_ptr()) };
}
```

> **Nota:** `set_lldb_dll_dir` é útil apenas quando o binário é executado fora
> do `cargo test`. O `build.rs` copia as DLLs para o diretório do executável,
> então durante `cargo test` a chamada é opcional.

---

## Fase 6 — Scripts PowerShell

**`scripts/install-llvm.ps1`** (usa winget, não choco):
```powershell
winget install LLVM.LLVM --version 19.1.7 --silent --accept-package-agreements
winget install Python.Python.3.10 --silent --accept-package-agreements
# Cria prefix virtual C:\lldb-dev com headers + libs
# Seta LLDB_SYS_PREFIX, LIBCLANG_PATH, LLVM_CONFIG no perfil do usuário
```

**`scripts/validate-env.ps1`** (auto-detecta prefixos):
```
Discovery: LLDB_SYS_PREFIX env var → C:\lldb-dev → C:\Program Files\LLVM
Checa: lldb.exe, LLDB.h, liblldb.dll, liblldb.lib, libclang.dll,
       versão >= 19, cl.exe ou clang-cl, rustc, MSVC target
```

**`scripts/test-cpp-api.ps1`** (gate check Phase 2):
- Descobre prefix e compilador (clang-cl ou cl.exe)
- Compila um `.cpp` mínimo que chama `SBDebugger::Create()`
- Adiciona Python 3.10 ao PATH antes de rodar o exe
- Deve imprimir `OK: LLDB version: lldb version 19.1.7`

---

## Fase 7 — CI (.github/workflows/ci.yml)

```yaml
jobs:
  build:
    strategy:
      matrix:
        include:
          - os: windows-latest
            # Nota: choco pode não estar disponível; preferir winget
            llvm_install: "winget install LLVM.LLVM --version 19.1.7 ..."
            lldb_prefix: "C:\\lldb-dev"   # prefix virtual com headers
            # Requer: baixar headers do GitHub + instalar Python 3.10
          - os: ubuntu-24.04
            llvm_install: |
              wget https://apt.llvm.org/llvm.sh && sudo ./llvm.sh 19
              sudo apt-get install -y liblldb-19-dev lldb-19
            lldb_prefix: "/usr/lib/llvm-19"
          - os: macos-latest
            llvm_install: "brew install llvm@19"
            lldb_prefix: "/opt/homebrew/opt/llvm@19"
            # Apple Silicon: toolchain aarch64-apple-darwin
```

**Atenção Windows no CI:**
1. O instalador LLVM não inclui headers — o CI precisa rodar o git sparse-checkout para baixá-los e criar o prefix virtual antes do `cargo build`
2. Python 3.10 precisa estar instalado (`winget install Python.Python.3.10`)
3. `PATH` precisa incluir `python310.dll` dir — ou confiar no `build.rs` que copia as DLLs

---

## Ordem de Implementação (sprints)

| Sprint | Entregável | Critério de sucesso | Estado |
|---|---|---|---|
| 1 | Workspace + lldb-build | `cargo build -p lldb-build` passa | ✅ |
| 2 | LLVM instalado + validado | `validate-env.ps1` all green | ✅ |
| 3 | Wrapper C++ MVP (todas as classes SB*) | Compila com clang-cl | ✅ |
| 4 | build.rs + bindings.rs | `cargo build -p lldb-sys` passa | ✅ |
| 5 | lldb-safe MVP | `Debugger::initialize()` não crasha | ✅ |
| 6 | Testes de integração | 6/6 testes passando, zero env vars | ✅ |
| **7** | **CI multi-plataforma** | **Green em Windows, Linux, macOS** | 🔜 |
| 8 | SBTarget, SBProcess, SBThread, SBFrame | Stepping funciona | — |
| 9 | SBBreakpoint, SBValue | Leitura de variáveis funciona | — |
| 10 | lldb-safe completo + docs | API pública estável | — |

---

## Verificação End-to-End

```rust
// crates/lldb-safe/tests/integration.rs
fn setup() {
    #[cfg(windows)]
    {
        // DLLs já foram copiadas pelo build.rs — chamada opcional.
        // Útil ao rodar o binário manualmente fora do cargo test.
        if let Ok(dir) = std::env::var("LLDB_DLL_DIR")
            .or_else(|_| std::env::var("LLDB_SYS_PREFIX").map(|p| format!("{}\\bin", p)))
            .or_else(|_| Ok(option_env!("LLDB_DLL_DIR").unwrap_or("").to_string()))
        {
            if !dir.is_empty() { lldb_safe::set_lldb_dll_dir(dir.as_ref()); }
        }
    }
    Debugger::initialize();
}
```

Rodar com:
```powershell
cargo test -p lldb-safe -- --test-threads=1   # sem env vars necessárias
```

---

## Riscos Conhecidos

| Risco | Mitigação |
|---|---|
| `liblldb.dll` não exporta todos os símbolos SB* | Verificar com `dumpbin /exports` — se faltar, compilar LLDB do fonte com `-DLLDB_API_LIBLLDB_EXPORTS=ON` |
| CRT mismatch (MSVC runtime) | Usar `/MD` no `cc::Build` para garantir mesmo CRT dinâmico da liblldb |
| bindgen 0.71+: `use_core()` sem argumento | API mudou — remover o argumento booleano |
| `liblldb.dll` não encontrada em runtime | `build.rs` copia DLLs para `target/`; `set_lldb_dll_dir()` para uso manual |
| `python310.dll` não encontrada em runtime | `liblldb.dll` depende de Python 3.10 — instalar com winget; `build.rs` copia automaticamente |
| CI Windows: headers LLDB ausentes no instalador | Fazer sparse-checkout de `lldb/include` do GitHub antes do `cargo build` |
| Python Traceback nos testes | LLDB tenta importar módulo `lldb` via Python scripting — é cosmético, não bloqueia |
| LLDB 19 API diverge de versões anteriores | A API SB é estável desde LLDB 3.x — breaking changes são raros |
