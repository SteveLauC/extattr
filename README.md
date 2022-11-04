# extattr
[![Cirrus Build Status](https://api.cirrus-ci.com/github/SteveLauC/extattr.svg)](https://cirrus-ci.com/github/SteveLauC/extattr)
[![crates.io](https://img.shields.io/crates/v/extattr.svg)](https://crates.io/crates/extattr)
[![docs.rs](https://docs.rs/extattr/badge.svg)](https://docs.rs/extattr)

Yet another Extended Attributes library for Rust.

## Table of contents

  * [Supported platforms and Documents](https://github.com/SteveLauC/extattr#supported-platforms-and-their-documents)
  * [Why another crate for EA? Diff from `xattr`](https://github.com/SteveLauC/extattr#why-another-crate-for-ea-any-difference-from-xattr)
  * [MSRV](https://github.com/SteveLauC/extattr#minimum-supported-rust-version-msrv)
  * [Contributing](https://github.com/SteveLauC/extattr#contributing)

## Supported platforms and their documents

* [Linux](https://docs.rs/extattr/latest/x86_64-unknown-linux-gnu/extattr/index.html)
* [FreeBSD](https://docs.rs/extattr/latest/x86_64-unknown-freebsd/extattr/index.html)
* [NetBSD](https://docs.rs/extattr/latest/x86_64-unknown-netbsd/extattr/index.html)
* [macOS](https://docs.rs/extattr/latest/aarch64-apple-darwin/extattr/index.html)
* [Android](https://docs.rs/extattr/latest/aarch64-linux-android/extattr/index.html)
* [iOS](https://docs.rs/extattr/latest/aarch64-apple-ios/extattr/index.html)

## Why another crate for EA? Any difference from [`xattr`](https://crates.io/crates/xattr)?

Extended Attributes syscalls vary across implementations, for example, to set an EA:

```c
// Linux
int setxattr(const char *path, const char *name, const void *value, 
             size_t size, int flags);

// FreeBSD
ssize_t extattr_set_file(const char *path, int attrnamespace,
	 const char *attrname, const void *data, size_t	nbytes);

// macOS
int setxattr(const char *path, const char *name, void *value, size_t size,
         u_int32_t position, int options);
```

`xattr` erases differences in those APIs and provides a consistent, rusty 
interface. 

```rust
// A consistent API that would work on every OS
pub fn set<N, P>(path: P, name: N, value: &[u8]) -> Result<()> 
```

`extattr` aims to provide bindings close to the native one.

```rust
// Linux
pub fn setxattr<P, S, B>(
    path: P,
    name: S,
    value: B,
    flags: Flags,
) -> Result<()>

// FreeBSD
pub fn extattr_set_file<P, S, B>(
    path: P,
    attrnamespace: AttrNamespace,
    attrname: S,
    data: B
) -> Result<()>

// macOS
pub fn setxattr<P, S, B>(
    path: P,
    name: S,
    value: B,
    position: u32,
    options: Options
) -> Result<()>
```

In most cases, you would like to use `xattr` instead of `extattr`. However, if 
you are on Linux and want to use that extra `flags` argument, or you are on macOS
and want to use the arguments `position` and `options`, then `extattr` probably 
is a good choice:)

## Minimum Supported Rust Version (MSRV)
`extattr` is supported on Rust 1.56.1 and higher. The MSRV will not be changed 
without bumping the major version.

## Contributing

Contributions of all forms are welcome, feel free to file an issue or make a pull request!

##### Test before your commit

1. Format the code

   ```shell
   $ cargo fmt
   ```

2. Pass the tests

   ```shell
   $ cargo test
   ``` 
   