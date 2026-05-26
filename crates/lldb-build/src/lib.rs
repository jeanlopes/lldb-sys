use std::path::{Path, PathBuf};
use std::process::Command;

/// Resolved LLVM/LLDB installation paths and metadata.
#[derive(Debug, Clone)]
pub struct LldbConfig {
    /// Root prefix (e.g. C:\Program Files\LLVM or /usr/lib/llvm-19)
    pub prefix: PathBuf,
    /// Path to LLDB/LLVM headers (contains lldb/API/LLDB.h)
    pub include_dir: PathBuf,
    /// Directory containing the import/shared library
    pub lib_dir: PathBuf,
    /// Directory containing lldb.exe / lldb binary
    pub bin_dir: PathBuf,
    /// Library name for cargo:rustc-link-lib (liblldb on Windows, lldb on Unix)
    pub lib_name: String,
    /// LLVM/LLDB version string
    pub version: String,
}

impl LldbConfig {
    fn from_prefix(prefix: impl AsRef<Path>) -> Option<Self> {
        let prefix = prefix.as_ref().to_path_buf();

        let include_dir = prefix.join("include");
        let lib_dir = prefix.join("lib");
        let bin_dir = prefix.join("bin");

        // Sanity-check: the LLDB API header must exist.
        let lldb_header = include_dir.join("lldb").join("API").join("LLDB.h");
        if !lldb_header.exists() {
            return None;
        }

        // Resolve the library name and verify the library file exists.
        let (lib_name, lib_file) = if cfg!(target_os = "windows") {
            ("liblldb".to_string(), lib_dir.join("liblldb.lib"))
        } else if cfg!(target_os = "macos") {
            ("lldb".to_string(), lib_dir.join("liblldb.dylib"))
        } else {
            // Linux: may be versioned (liblldb-19.so) or plain (liblldb.so)
            let versioned = lib_dir.join("liblldb-19.so");
            if versioned.exists() {
                ("lldb-19".to_string(), versioned)
            } else {
                ("lldb".to_string(), lib_dir.join("liblldb.so"))
            }
        };

        if !lib_file.exists() {
            return None;
        }

        let version = probe_version(&bin_dir).unwrap_or_else(|| "unknown".to_string());

        Some(LldbConfig {
            prefix,
            include_dir,
            lib_dir,
            bin_dir,
            lib_name,
            version,
        })
    }
}

/// Find a usable LLDB 19 installation.
///
/// Discovery order (first match wins):
/// 1. `LLDB_SYS_PREFIX` env var
/// 2. `LLVM_SYS_PREFIX` env var
/// 3. `LLVM_CONFIG` env var (path to llvm-config binary)
/// 4. Windows registry (`HKLM\SOFTWARE\LLVM\LLVM`)
/// 5. Chocolatey default path
/// 6. Well-known install paths (OS-specific)
/// 7. `llvm-config` in PATH
pub fn find_lldb() -> Result<LldbConfig, String> {
    // Inform cargo to re-run if these env vars change.
    println!("cargo:rerun-if-env-changed=LLDB_SYS_PREFIX");
    println!("cargo:rerun-if-env-changed=LLVM_SYS_PREFIX");
    println!("cargo:rerun-if-env-changed=LLVM_CONFIG");

    try_env_prefix("LLDB_SYS_PREFIX")
        .or_else(|| try_env_prefix("LLVM_SYS_PREFIX"))
        .or_else(try_llvm_config_env)
        .or_else(try_registry)
        .or_else(try_chocolatey)
        .or_else(try_scoop)
        .or_else(try_known_paths)
        .or_else(try_llvm_config_path)
        .ok_or_else(|| {
            "LLDB not found. Set LLDB_SYS_PREFIX to your LLVM installation directory \
             (e.g. C:\\Program Files\\LLVM on Windows, /usr/lib/llvm-19 on Linux, \
             /opt/homebrew/opt/llvm@19 on macOS)."
                .to_string()
        })
}

fn try_env_prefix(var: &str) -> Option<LldbConfig> {
    let val = std::env::var(var).ok()?;
    LldbConfig::from_prefix(&val)
}

fn try_llvm_config_env() -> Option<LldbConfig> {
    let config_bin = std::env::var("LLVM_CONFIG").ok()?;
    prefix_from_llvm_config(&config_bin)
}

fn try_llvm_config_path() -> Option<LldbConfig> {
    // Try common versioned names first, then the plain name.
    for name in &["llvm-config-19", "llvm-config19", "llvm-config"] {
        if let Some(cfg) = prefix_from_llvm_config(name) {
            return Some(cfg);
        }
    }
    None
}

fn prefix_from_llvm_config(binary: &str) -> Option<LldbConfig> {
    let prefix_out = Command::new(binary)
        .arg("--prefix")
        .output()
        .ok()?;
    if !prefix_out.status.success() {
        return None;
    }
    let prefix = PathBuf::from(String::from_utf8_lossy(&prefix_out.stdout).trim());
    LldbConfig::from_prefix(prefix)
}

#[cfg(target_os = "windows")]
fn try_registry() -> Option<LldbConfig> {
    // LLVM installer writes: HKLM\SOFTWARE\LLVM\LLVM  ->  InstallDir = C:\Program Files\LLVM
    let out = Command::new("reg")
        .args(["query", r"HKLM\SOFTWARE\LLVM\LLVM", "/v", "InstallDir"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    parse_reg_value(&text).and_then(|p| LldbConfig::from_prefix(p))
}

#[cfg(not(target_os = "windows"))]
fn try_registry() -> Option<LldbConfig> {
    None
}

/// Parse the path value from `reg query` stdout.
/// Sample line: "    InstallDir    REG_SZ    C:\Program Files\LLVM"
fn parse_reg_value(text: &str) -> Option<PathBuf> {
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("InstallDir") {
            // Split on runs of whitespace; the path is the last token group.
            // Using rfind of "    " is more robust than split() when path has spaces.
            if let Some(pos) = line.rfind("    ") {
                let path = line[pos + 4..].trim();
                if !path.is_empty() {
                    return Some(PathBuf::from(path));
                }
            }
        }
    }
    None
}

#[cfg(target_os = "windows")]
fn try_chocolatey() -> Option<LldbConfig> {
    // Chocolatey installs LLVM under ProgramData by default.
    let choco_root = PathBuf::from(r"C:\ProgramData\chocolatey\lib\llvm\tools\llvm");
    LldbConfig::from_prefix(&choco_root)
}

#[cfg(not(target_os = "windows"))]
fn try_chocolatey() -> Option<LldbConfig> {
    None
}

#[cfg(target_os = "windows")]
fn try_scoop() -> Option<LldbConfig> {
    // Scoop installs to %USERPROFILE%\scoop\apps\llvm\current
    let home = std::env::var("USERPROFILE").ok()?;
    let path = PathBuf::from(home).join("scoop").join("apps").join("llvm").join("current");
    LldbConfig::from_prefix(&path)
}

#[cfg(not(target_os = "windows"))]
fn try_scoop() -> Option<LldbConfig> {
    None
}

fn try_known_paths() -> Option<LldbConfig> {
    let candidates: &[&str] = if cfg!(target_os = "windows") {
        &[
            r"C:\lldb-dev",
            r"C:\Program Files\LLVM",
            r"C:\LLVM",
        ]
    } else if cfg!(target_os = "macos") {
        &[
            "/opt/homebrew/opt/llvm@19",
            "/opt/homebrew/opt/llvm",
            "/usr/local/opt/llvm@19",
            "/usr/local/opt/llvm",
        ]
    } else {
        // Linux
        &[
            "/usr/lib/llvm-19",
            "/usr/local/lib/llvm-19",
            "/usr/lib/llvm",
        ]
    };

    for candidate in candidates {
        if let Some(cfg) = LldbConfig::from_prefix(candidate) {
            return Some(cfg);
        }
    }
    None
}

fn probe_version(bin_dir: &Path) -> Option<String> {
    let lldb = if cfg!(target_os = "windows") {
        bin_dir.join("lldb.exe")
    } else {
        bin_dir.join("lldb")
    };

    let out = Command::new(&lldb)
        .arg("--version")
        .output()
        .ok()?;

    let text = String::from_utf8_lossy(&out.stdout);
    // "lldb version 19.1.7" -> "19.1.7"
    for word in text.split_whitespace() {
        if word.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            return Some(word.to_string());
        }
    }
    None
}
