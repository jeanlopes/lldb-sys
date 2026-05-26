# Status do Projeto

## Estado atual: **FUNCIONANDO** ✅

`cargo build` e `cargo test` passam sem nenhuma variável de ambiente.

```
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## O que foi construído

Projeto completo — 35+ arquivos criados do zero:

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
│   │   ├── build.rs                    compila C++, roda bindgen, linka liblldb,
│   │   │                               copia DLLs para target/ automaticamente
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
│       └── tests/integration.rs        6 smoke tests (todos passando)
│
└── scripts/
    ├── install-llvm.ps1                instala via winget, seta env vars
    ├── validate-env.ps1                verifica tudo (DLL, headers, MSVC, bindgen)
    └── test-cpp-api.ps1                gate check: compila e roda C++ mínimo
```

---

## Ambiente configurado (Windows)

| Componente | Versão | Localização |
|---|---|---|
| LLVM / LLDB | 19.1.7 | `C:\Program Files\LLVM` |
| Python | 3.10.11 | `C:\Users\jeano\AppData\Local\Programs\Python\Python310` |
| LLDB headers | 19.1.7 | `C:\lldb-dev\include\lldb\` (baixados do GitHub) |
| Visual Studio Build Tools | 2022 v17.14 | — |
| Rust toolchain | stable-x86_64-pc-windows-msvc | — |

O prefixo virtual `C:\lldb-dev` combina:
- Headers do GitHub (`llvmorg-19.1.7`, sparse-checkout de `lldb/include`)
- `liblldb.lib` + `liblldb.dll` + `libclang.dll` copiados de `C:\Program Files\LLVM`

---

## Como rodar

### Build
```powershell
cargo build          # sem variáveis de ambiente necessárias
```

### Testes
```powershell
cargo test -p lldb-safe -- --test-threads=1   # sem variáveis de ambiente necessárias
```

O `build.rs` copia automaticamente `liblldb.dll` e `python310.dll` para
`target/debug/deps/` e `target/debug/`, então o Windows encontra as DLLs
sem PATH manual.

### Validação do ambiente
```powershell
.\scripts\validate-env.ps1    # deve mostrar "All checks passed!"
.\scripts\test-cpp-api.ps1    # deve mostrar "Phase 2 gate check PASSED"
```

---

## O que foi resolvido durante o setup

| Problema | Causa | Solução |
|---|---|---|
| `bindgen::use_core(false)` não compila | API mudou no bindgen 0.71 | Argumento removido |
| LLDB não encontrado no build | LLVM instalado mas sem headers | Prefix virtual `C:\lldb-dev` + `C:\lldb-dev` adicionado a `try_known_paths()` |
| `bindgen` não achava `libclang.dll` | Faltava no prefix virtual | Copiado + fallback em cascata no `build.rs` |
| `cargo test` falhava com `STATUS_DLL_NOT_FOUND` | `python310.dll` faltando em runtime | Python 3.10 instalado; `build.rs` copia DLLs para `target/` |
| `test-cpp-api.ps1` não achava headers | Script usava `C:\Program Files\LLVM\include` | Script atualizado para usar prefix correto |
| `validate-env.ps1` reportava falsos negativos | Script não conhecia `C:\lldb-dev` | Script atualizado com auto-detecção e checks revisados |

---

## Próximos sprints

| Sprint | Entregável | Critério de sucesso |
|---|---|---|
| ~~1~~ | ~~Workspace + lldb-build~~ | ✅ |
| ~~2~~ | ~~LLVM instalado + validado~~ | ✅ |
| ~~3~~ | ~~Wrapper C++ MVP~~ | ✅ |
| ~~4~~ | ~~build.rs + bindings.rs~~ | ✅ |
| ~~5~~ | ~~lldb-safe MVP~~ | ✅ |
| ~~6~~ | ~~Testes de integração~~ | ✅ 6/6 passando |
| **7** | **CI multi-plataforma** | Green em Windows, Linux, macOS |
| 8 | SBTarget, SBProcess, SBThread, SBFrame completos | Stepping funciona |
| 9 | SBBreakpoint, SBValue | Leitura de variáveis funciona |
| 10 | lldb-safe completo + docs | API pública estável |

---

## Riscos conhecidos (ainda válidos)

| Sintoma | Causa provável | Fix |
|---|---|---|
| CI Linux/macOS falhando | `LLDB_SYS_PREFIX` não setado na matrix | Conferir `.github/workflows/ci.yml` |
| `liblldb.dll` não exporta símbolo esperado | Versão LLDB diferente da compilada | Verificar com `dumpbin /exports` |
| `C2589: const in unexpected location` | Incompatibilidade clang-cl + MSVC headers | Adicionar `/permissive-` no `cc::Build` |
| Python Traceback no início dos testes | LLDB tenta importar módulo `lldb` via Python | Cosmético — não afeta os testes |
