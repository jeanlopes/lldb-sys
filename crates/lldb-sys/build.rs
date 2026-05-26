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

    // ── 3. On Windows, hint the runtime DLL directory and copy DLLs so that
    //       test binaries find them without PATH setup.
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-env=LLDB_DLL_DIR={}", lldb.bin_dir.display());
        copy_runtime_dlls_windows(&lldb.bin_dir);
    }

    // ── 4. Generate Rust bindings with bindgen ─────────────────────────────
    // Resolve the directory that contains libclang.dll / libclang.so.
    // Try LIBCLANG_PATH first, then lldb.bin_dir, then the LLVM system install.
    let libclang_path = std::env::var("LIBCLANG_PATH").ok()
        .or_else(|| {
            let dll = lldb.bin_dir.join(if cfg!(windows) { "libclang.dll" } else { "libclang.so" });
            if dll.exists() { Some(lldb.bin_dir.to_string_lossy().to_string()) } else { None }
        })
        .or_else(|| {
            // Fallback: the official LLVM installer puts binaries in Program Files\LLVM\bin
            #[cfg(target_os = "windows")]
            {
                let p = std::path::Path::new(r"C:\Program Files\LLVM\bin");
                if p.join("libclang.dll").exists() { return Some(p.to_string_lossy().to_string()); }
            }
            None
        })
        .unwrap_or_else(|| lldb.bin_dir.to_string_lossy().to_string());

    // bindgen (via clang-sys) reads LIBCLANG_PATH at load time.
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

/// Copy liblldb.dll and python310.dll next to the Cargo output binaries so
/// that `cargo test` finds them without any PATH manipulation.
///
/// OUT_DIR is `.../target/<profile>/build/lldb-sys-<hash>/out`.
/// Test binaries live in `.../target/<profile>/deps/`.
/// We copy to both `deps/` and the profile root so `cargo run` also works.
#[cfg(target_os = "windows")]
fn copy_runtime_dlls_windows(lldb_bin: &std::path::Path) {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    // Navigate: out → lldb-sys-hash → build → profile (debug/release)
    let profile_dir = match out_dir.parent().and_then(|p| p.parent()).and_then(|p| p.parent()) {
        Some(d) => d.to_path_buf(),
        None => return,
    };

    let dlls_to_copy: &[&str] = &["liblldb.dll"];
    let python_dll = find_python310_dll();

    for dest_subdir in &["deps", ""] {
        let dest = if dest_subdir.is_empty() {
            profile_dir.clone()
        } else {
            profile_dir.join(dest_subdir)
        };
        if !dest.exists() {
            continue;
        }
        for dll_name in dlls_to_copy {
            let src = lldb_bin.join(dll_name);
            if src.exists() {
                let _ = std::fs::copy(&src, dest.join(dll_name));
            }
        }
        if let Some(ref py_dll) = python_dll {
            let name = py_dll.file_name().unwrap();
            let _ = std::fs::copy(py_dll, dest.join(name));
        }
    }
}

/// Find python310.dll in well-known locations on Windows.
#[cfg(target_os = "windows")]
fn find_python310_dll() -> Option<PathBuf> {
    let username = std::env::var("USERNAME").unwrap_or_default();
    let candidates = [
        format!(r"C:\Users\{}\AppData\Local\Programs\Python\Python310\python310.dll", username),
        r"C:\Program Files\Python310\python310.dll".to_string(),
        r"C:\Python310\python310.dll".to_string(),
        // Also check next to liblldb in case it was bundled there
        r"C:\Program Files\LLVM\bin\python310.dll".to_string(),
        r"C:\lldb-dev\bin\python310.dll".to_string(),
    ];
    candidates.iter().map(PathBuf::from).find(|p| p.exists())
}

/// Collect all *.cpp files under `dir`.
fn glob_cpp_files(dir: &str) -> Vec<PathBuf> {
    let pattern = format!("{}/*.cpp", dir);
    glob::glob(&pattern)
        .expect("bad glob pattern")
        .filter_map(|e| e.ok())
        .collect()
}
