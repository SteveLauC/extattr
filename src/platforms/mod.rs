#[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
pub mod freebsd;

#[cfg(any(target_os = "linux", target_os = "android", target_os = "netbsd"))]
pub mod linux_and_android;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub mod darwin;

#[cfg(target_os = "netbsd")]
pub mod netbsd;
