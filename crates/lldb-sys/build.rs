use std::path::PathBuf;

fn main() {
    // Skip the entire build for docs.rs (no LLDB available there).
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    let lldb = lldb_build::find_lldb().unwrap_or_else(|e| panic!("{}", e));

    println!("cargo:rerun-if-changed=wrapper/include/lldb_c.h");
    println!("cargo:rerun-if-changed=wrapper/src");

    // ── 1. Compile the thin C++ wrapper layer ──────────────────────────────
    let wrapper_sources = glob_cpp_files("wrapper/src");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .warnings(false)
        .include("wrapper/include")
        .include(&lldb.include_dir);

    // MSVC-specific flags: exception handling + dynamic CRT (must match liblldb).
    if build.get_compiler().is_like_msvc() {
        build.flag("/EHsc").flag("/MD");
    }

    for src in &wrapper_sources {
        build.file(src);
    }
    build.compile("lldb_c_wrapper");

    // ── 2. Link against liblldb (dynamic) ──────────────────────────────────
    println!("cargo:rustc-link-search=native={}", lldb.lib_dir.display());
    println!("cargo:rustc-link-lib=dylib={}", lldb.lib_name);

    // ── 3. On Windows, hint the runtime DLL directory via a cargo env var.
    //       The application must call set_lldb_dll_dir() before using LLDB.
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-env=LLDB_DLL_DIR={}", lldb.bin_dir.display());
    }

    // ── 4. Generate Rust bindings with bindgen ─────────────────────────────
    let libclang_path = std::env::var("LIBCLANG_PATH")
        .unwrap_or_else(|_| lldb.bin_dir.to_string_lossy().to_string());

    // bindgen needs LIBCLANG_PATH to find libclang.dll / libclang.so.
    std::env::set_var("LIBCLANG_PATH", &libclang_path);

    let mut builder = bindgen::Builder::default()
        .header("wrapper/include/lldb_c.h")
        .clang_arg(format!("-I{}", lldb.include_dir.display()))
        // Only expose our own LLDB_* functions and the SB*Ref / LLDBStateType etc.
        .allowlist_function("LLDB_.*")
        .allowlist_type("LLDB.*|SB.*Ref")
        .allowlist_var("LLDB.*")
        // Prevent std::* internals from leaking into the bindings.
        .opaque_type("std::.*")
        // Use libc types for C primitives.
        .ctypes_prefix("libc")
        .use_core(false)
        .layout_tests(false);

    // On Windows, tell clang to behave like MSVC so the LLDB headers parse correctly.
    if cfg!(target_os = "windows") {
        builder = builder
            .clang_arg("--target=x86_64-pc-windows-msvc")
            .clang_arg("-fms-compatibility");
    }

    let bindings = builder.generate().expect(
        "bindgen failed to generate bindings. \
         Make sure LLDB_SYS_PREFIX is set and LIBCLANG_PATH points to your LLVM bin directory.",
    );

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings.rs");
}

/// Collect all *.cpp files under `dir`.
fn glob_cpp_files(dir: &str) -> Vec<PathBuf> {
    let pattern = format!("{}/*.cpp", dir);
    glob::glob(&pattern)
        .expect("bad glob pattern")
        .filter_map(|e| e.ok())
        .collect()
}
