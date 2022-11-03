mod macros;
mod platforms;

// Platform-dependent re-export

#[cfg(target_os = "freebsd")]
pub use platforms::freebsd::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use platforms::linux_and_android::*;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use platforms::darwin::*;

#[cfg(target_os = "netbsd")]
pub use platforms::netbsd::*;

use errno::Errno;

/// Customized `Result` type for `extattr`.
pub type Result<T> = std::result::Result<T, Errno>;
