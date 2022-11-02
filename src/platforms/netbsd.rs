//! EA APIs of NetBSD
//!
//! We simply re-export the API of Linux and FreeBSD since NetBSD is compatible
//! with them.
pub use super::linux_and_android::*;
pub use super::freebsd::*;

