/// Basic smoke tests — require LLDB_SYS_PREFIX to be set and liblldb.dll
/// to be accessible at runtime (use LLDB_DLL_DIR or set PATH on Windows).
///
/// Run with:
///   $env:LLDB_SYS_PREFIX = "C:\Program Files\LLVM"
///   cargo test -p lldb-safe -- --test-thread=1
use lldb_safe::{Debugger, State};

fn setup() {
    // On Windows, add the LLDB bin dir to the DLL search path before any
    // LLDB symbols are resolved.
    #[cfg(windows)]
    {
        let dll_dir = std::env::var("LLDB_DLL_DIR")
            .or_else(|_| {
                std::env::var("LLDB_SYS_PREFIX")
                    .map(|p| format!("{}\\bin", p))
            })
            .expect(
                "Set LLDB_DLL_DIR or LLDB_SYS_PREFIX so that liblldb.dll can be found at runtime",
            );
        lldb_safe::set_lldb_dll_dir(dll_dir.as_ref());
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
