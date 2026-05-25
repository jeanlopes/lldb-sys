# Status do Projeto

## O que foi construído

O projeto está **completo no código** — 35 arquivos criados do zero:

### Estrutura final
```
lldb-sys/
├── Cargo.toml                          workspace (3 crates)
├── README.md                           docs completas
├── .gitignore
├── .github/workflows/ci.yml            matrix Windows + Linux + macOS
│
├── crates/
│   ├── lldb-build/src/lib.rs           descoberta LLVM: registry → choco → scoop → paths → llvm-config
│   │
│   ├── lldb-sys/
│   │   ├── build.rs                    compila C++, roda bindgen, linka liblldb
│   │   ├── src/lib.rs                  include!(bindings.rs)
│   │   └── wrapper/
│   │       ├── include/lldb_c.h        API C pública (SBDebugger...SBValue)
│   │       └── src/
│   │           ├── SBDebugger.cpp
│   │           ├── SBError.cpp
│   │           ├── SBTarget.cpp
│   │           ├── SBProcess.cpp
│   │           ├── SBThread.cpp
│   │           ├── SBFrame.cpp
│   │           ├── SBBreakpoint.cpp
│   │           ├── SBValue.cpp
│   │           ├── SBFileSpec.cpp
│   │           └── SBListener.cpp
│   │
│   └── lldb-safe/
│       ├── src/{debugger,target,process,thread,frame,breakpoint,value,error}.rs
│       └── tests/integration.rs        5 smoke tests
│
└── scripts/
    ├── install-llvm.ps1                instala via choco, seta env vars
    ├── validate-env.ps1                verifica tudo (DLL, imports, MSVC, bindgen)
    └── test-cpp-api.ps1                gate check Phase 2: compila e roda C++ mínimo
```

---

## Próximos passos imediatos (você precisa fazer)

**1. Instalar LLVM 19** (pré-requisito para qualquer `cargo build`):
```powershell
# Abrir PowerShell como administrador
.\scripts\install-llvm.ps1
```

**2. Abrir Developer Command Prompt** (para `cl.exe` / `link.exe`):
```powershell
# Ou instalar se não tiver:
choco install visualstudio2022buildtools -y
```

**3. Validar o ambiente:**
```powershell
.\scripts\validate-env.ps1
```

**4. Gate check — verificar que a SB API funciona em C++:**
```powershell
.\scripts\test-cpp-api.ps1
# Se isso falhar → o projeto para aqui até resolver o linking
```

**5. Primeiro build Rust:**
```powershell
$env:LLDB_SYS_PREFIX = "C:\Program Files\LLVM"
$env:LIBCLANG_PATH   = "C:\Program Files\LLVM\bin"
cargo build -p lldb-sys
```

**6. Testes:**
```powershell
$env:PATH = "C:\Program Files\LLVM\bin;$env:PATH"
cargo test -p lldb-safe -- --test-threads=1
```

---

## Riscos que ainda podem aparecer no primeiro build

| Sintoma | Causa provável | Fix |
|---|---|---|
| `bindgen: error: header not found` | `LIBCLANG_PATH` errado | Aponta para `C:\Program Files\LLVM\bin` |
| `error LNK2019: unresolved external` em `liblldb` | import lib não encontrado | Confirma que `C:\Program Files\LLVM\lib\liblldb.lib` existe |
| `cannot open input file 'liblldb.lib'` | `lib_dir` errado no `lldb-build` | Seta `LLDB_SYS_PREFIX` explicitamente |
| Runtime: `DLL not found` | `liblldb.dll` não está no PATH | Chama `set_lldb_dll_dir()` ou adiciona `C:\Program Files\LLVM\bin` ao PATH |
| `C2589: const in unexpected location` | Incompatibilidade clang-cl + MSVC headers | Adiciona `/permissive-` no `cc::Build` |
