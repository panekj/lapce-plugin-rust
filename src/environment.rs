use std::env;

/// Helper struct abstracting environment variables
/// names used in lapce to provide revelant host
/// environment information, so plugin maintainers
/// don't have to hardcode specific variable names
pub struct VoltEnvironment {}

impl VoltEnvironment {
    /// Plugin location path encoded as Url
    pub fn uri() -> Result<String, env::VarError> {
        env::var("VOLT_URI")
    }

    /// Operating system name as provided by
    /// std::env::consts::OS
    pub fn operating_system() -> Result<String, env::VarError> {
        env::var("VOLT_OS")
    }

    /// Processor architecture name as provided by
    /// std::env::consts::ARCH
    pub fn architecture() -> Result<String, env::VarError> {
        env::var("VOLT_ARCH")
    }

    /// C library used on host detected by parsing ldd output
    /// provided because of musl-based linux distros and distros
    /// that need statically linked binaries due to how
    /// linking works (e.g. nixOS)
    /// Currently only 2 options are available: glibc | musl
    /// This function will return empty string on non-linux
    /// hosts
    pub fn libc() -> Result<String, env::VarError> {
        env::var("VOLT_LIBC")
    }
}
