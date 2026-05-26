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
