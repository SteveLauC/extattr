//! # extattr
//! [![Cirrus Build Status](https://api.cirrus-ci.com/github/SteveLauC/extattr.svg)](https://cirrus-ci.com/github/SteveLauC/extattr)
//! [![crates.io](https://img.shields.io/crates/v/extattr.svg)](https://crates.io/crates/extattr)
//! [![docs.rs](https://docs.rs/extattr/badge.svg)](https://docs.rs/extattr)
//!
//! Yet another Extended Attributes library for Rust.
//!
//! ## Why another crate for EA? Any difference from [`xattr`](https://crates.io/crates/xattr)?
//!
//! Extended Attributes syscalls vary across implementations, for example, to set an EA:
//!
//! ```c
//! // Linux
//! int setxattr(const char *path, const char *name, const void *value,
//! size_t size, int flags);
//!
//! // FreeBSD
//! ssize_t extattr_set_file(const char *path, int attrnamespace,
//! const char *attrname, const void *data, size_t	nbytes);
//!
//! // macOS
//! int setxattr(const char *path, const char *name, void *value, size_t size,
//! u_int32_t position, int options);
//! ```
//!
//! `xattr` erases differences in those APIs and provides a consistent, rusty
//! interface.
//!
//! ```ignore
//! // A consistent API that would work on every OS
//! pub fn set<N, P>(path: P, name: N, value: &[u8]) -> Result<()>
//! ```
//!
//! `extattr` aims to provide bindings close to the native one.
//!
//! ```ignore
//! // Linux
//! pub fn setxattr<P, S, B>(
//!     path: P,
//!     name: S,
//!     value: B,
//!     flags: Flags,
//! ) -> Result<()>
//!
//! // FreeBSD
//! pub fn extattr_set_file<P, S, B>(
//!     path: P,
//!     attrnamespace: AttrNamespace,
//!     attrname: S,
//!     data: B
//! ) -> Result<()>
//!
//! // macOS
//! pub fn setxattr<P, S, B>(
//!     path: P,
//!     name: S,
//!     value: B,
//!     position: u32,
//!     options: Options
//! ) -> Result<()>
//! ```
//!
//! In most cases, you would like to use `xattr` instead of `extattr`. However, if
//! you are on Linux and want to use that extra `flags` argument, or you are on macOS
//! and want to use the arguments `position` and `options`, then `extattr` probably
//! is a good choice:)

mod macros;
mod platforms;

use errno::Errno;

/// Customized `Result` type for `extattr`.
pub type Result<T> = std::result::Result<T, Errno>;

// Platform-dependent re-export

#[cfg(target_os = "freebsd")]
pub use platforms::freebsd::*;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use platforms::linux_and_android::*;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use platforms::darwin::*;

#[cfg(target_os = "netbsd")]
pub use platforms::netbsd::*;
