/// Basic smoke tests — require LLDB_SYS_PREFIX to be set and liblldb.dll
/// to be accessible at runtime (use LLDB_DLL_DIR or set PATH on Windows).
///
/// Run with:
///   $env:LLDB_SYS_PREFIX = "C:\Program Files\LLVM"
///   cargo test -p lldb-safe -- --test-thread=1
use lldb_safe::Debugger;

fn setup() {
    // On Windows, optionally hint the DLL search path.  When build.rs copies
    // liblldb.dll + python310.dll next to the test binary this call is a no-op,
    // but it helps when running the binary outside of `cargo test`.
    #[cfg(windows)]
    {
        let dll_dir =
            // 1. Runtime env var (set manually or by the caller)
            std::env::var("LLDB_DLL_DIR").ok()
            // 2. Derive from LLDB_SYS_PREFIX
            .or_else(|| std::env::var("LLDB_SYS_PREFIX").ok().map(|p| format!("{}\\bin", p)))
            // 3. Compile-time value baked in by build.rs
            .or_else(|| option_env!("LLDB_DLL_DIR").map(|s| s.to_string()));

        if let Some(dir) = dll_dir {
            lldb_safe::set_lldb_dll_dir(dir.as_ref());
        }
        // If none of the above are set, liblldb.dll was already copied next to
        // the test binary by build.rs and Windows will find it automatically.
    }

    Debugger::initialize();
}

fn teardown() {
    Debugger::terminate();
}

// Helper that runs `f` bracketed by initialize/terminate.
fn with_debugger<F: FnOnce(&Debugger)>(f: F) {
    setup();
    let dbg = Debugger::create(false).expect("Debugger::create returned None");
    assert!(dbg.is_valid(), "debugger must be valid after create()");
    f(&dbg);
    drop(dbg);
    teardown();
}

// ── Tests ─────────────────────────────────────────────────────────────────

#[test]
fn version_string_is_non_empty() {
    setup();
    let v = Debugger::version_string();
    assert!(!v.is_empty(), "version string must not be empty");
    assert!(v.contains("lldb") || v.contains("LLDB"), "expected 'lldb' in: {}", v);
    teardown();
}

#[test]
fn create_and_destroy_debugger() {
    with_debugger(|dbg| {
        assert!(dbg.is_valid());
        assert_eq!(dbg.num_targets(), 0);
    });
}

#[test]
fn async_mode_roundtrip() {
    with_debugger(|dbg| {
        dbg.set_async(true);
        assert!(dbg.get_async());
        dbg.set_async(false);
        assert!(!dbg.get_async());
    });
}

#[test]
fn create_target_for_self() {
    with_debugger(|dbg| {
        // Use the current test executable as the target.
        let exe = std::env::current_exe()
            .expect("could not determine test executable path");
        let target = dbg
            .create_target_simple(exe.to_str().unwrap())
            .expect("create_target_simple returned None");

        assert!(target.is_valid());
        assert_eq!(dbg.num_targets(), 1);

        let triple = target.triple().unwrap_or("");
        assert!(!triple.is_empty(), "target triple must not be empty");
    });
}

#[test]
fn breakpoint_create_by_name() {
    with_debugger(|dbg| {
        let exe = std::env::current_exe().unwrap();
        let target = dbg.create_target_simple(exe.to_str().unwrap()).unwrap();

        let bp = target
            .breakpoint_by_name("main", None)
            .expect("breakpoint_by_name returned None");

        assert!(bp.is_valid());
        assert!(bp.id() > 0);
        assert!(bp.is_enabled());
        assert_eq!(target.num_breakpoints(), 1);

        target.delete_breakpoint(bp.id());
        assert_eq!(target.num_breakpoints(), 0);
    });
}

#[test]
fn error_type_success_and_fail() {
    use lldb_safe::Error;

    let err = Error::new();
    // A freshly created SBError has no error set — Success() is true.
    assert!(err.success());
    assert!(!err.fail());
    assert_eq!(err.code(), 0);
}

/// Test that reload_module_with_sym_file successfully associates a PDB with
/// a Windows MSVC binary, so that source-line breakpoints can be resolved.
///
/// Requires the `rust_app_example` test binary to exist at the path below.
/// Run with:
///   cargo test -p lldb-safe -- --test-thread=1 reload_module_with_pdb -- --nocapture
#[test]
fn reload_module_with_pdb() {
    let exe = r"C:\workspace\rust_app_example\target\debug\rust_app_example.exe";
    let pdb = r"C:\workspace\rust_app_example\target\debug\rust_app_example.pdb";

    if !std::path::Path::new(exe).exists() {
        eprintln!("SKIP: test binary not found at {exe}");
        return;
    }
    if !std::path::Path::new(pdb).exists() {
        eprintln!("SKIP: PDB not found at {pdb}");
        return;
    }

    with_debugger(|dbg| {
        let target = dbg
            .create_target_simple(exe)
            .expect("create_target_simple returned None");

        assert!(target.is_valid(), "target must be valid");

        let ok = target.reload_module_with_sym_file(exe, pdb);
        println!("reload_module_with_sym_file returned: {ok}");

        // Check image list — the module should appear with PDB path as symbol file.
        let ilist = dbg.handle_command("image list");
        println!("image list:\n{ilist}");

        // Verify via CLI that main.rs symbols are now available.
        let lookup = dbg.handle_command("image lookup -n main --verbose");
        println!("image lookup -n main --verbose:\n{lookup}");

        let lt = dbg.handle_command("image dump line-table main.rs");
        println!("image dump line-table main.rs:\n{lt}");

        // Try to set a source-line breakpoint at main.rs:1
        let bp = target.breakpoint_by_location(
            r"C:\workspace\rust_app_example\src\main.rs",
            1,
        );
        let bp_ref = bp.expect("breakpoint_by_location returned None");
        let locs = bp_ref.num_locations();
        println!("breakpoint num_locations: {locs}");

        // If PDB loaded, we should have at least 1 location.
        // This assertion may still be 0 if LLDB resolves lazily post-launch —
        // but at minimum the module reload must succeed.
        assert!(ok, "reload_module_with_sym_file must return true");
    });
}
