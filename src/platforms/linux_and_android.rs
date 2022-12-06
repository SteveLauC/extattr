//! EA syscall bindings for Linux and Android

use crate::Result;
use bitflags::bitflags;
use errno::{errno, Errno};
use std::{
    ffi::{CString, OsStr, OsString},
    os::unix::{ffi::OsStrExt, io::RawFd},
    path::Path,
    ptr::null_mut,
};

bitflags! {
    /// `flags` used when setting EAs
    pub struct Flags: libc::c_int {
        /// Perform a pure create, which fails if the named attribute exists
        /// already.
        const XATTR_CREATE = libc::XATTR_CREATE;
        /// Perform a pure replace operation, which fails if the named attribute
        /// does not already exist.
        const XATTR_REPLACE = libc::XATTR_REPLACE;
    }
}

/// Retrieves the list of extended attribute names associated with the given `path`
/// in the filesystem. If `path` is a symbolic link, it will be dereferenced.
///
/// For more infomation, see [listxattr(2)](https://man7.org/linux/man-pages/man2/listxattr.2.html)
pub fn listxattr<P: AsRef<Path>>(path: P) -> Result<Vec<OsString>> {
    let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
        Ok(p) => p,
        _ => return Err(Errno(libc::EINVAL)),
    };

    // query the buffer size
    let buffer_size =
        match unsafe { libc::listxattr(path.as_ptr(), null_mut(), 0) } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            buffer_size => buffer_size,
        };

    let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);
    let res = unsafe {
        libc::listxattr(
            path.as_ptr(),
            buffer.as_mut_ptr().cast(),
            buffer.capacity(),
        )
    };

    match res {
        -1 => Err(errno()),
        len => {
            unsafe { buffer.set_len(len as usize) };
            Ok(buffer[..(len - 1) as usize]
                .split(|&item| item == 0)
                .map(OsStr::from_bytes)
                .map(|str| str.to_owned())
                .collect::<Vec<OsString>>())
        }
    }
}

/// Retrieves the list of extended attribute names associated with the given `path`
/// in the filesystem. If `path` is a symbolic link, the list of names associated
/// with the link *itself* will be returned.
///
/// For more infomation, see [llistxattr(2)](https://man7.org/linux/man-pages/man2/listxattr.2.html)
pub fn llistxattr<P: AsRef<Path>>(path: P) -> Result<Vec<OsString>> {
    let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
        Ok(p) => p,
        _ => return Err(Errno(libc::EINVAL)),
    };

    // query the buffer size
    let buffer_size =
        match unsafe { libc::llistxattr(path.as_ptr(), null_mut(), 0) } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            buffer_size => buffer_size,
        };

    let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);
    let res = unsafe {
        libc::llistxattr(
            path.as_ptr(),
            buffer.as_mut_ptr().cast(),
            buffer.capacity(),
        )
    };

    match res {
        -1 => Err(errno()),
        len => {
            unsafe { buffer.set_len(len as usize) };
            Ok(buffer[..(len - 1) as usize]
                .split(|&item| item == 0)
                .map(OsStr::from_bytes)
                .map(|str| str.to_owned())
                .collect::<Vec<OsString>>())
        }
    }
}

/// Retrieves the list of extended attribute names associated with the file
/// specified by the open file descriptor `fd` in the filesystem.
///
/// For more infomation, see [flistxattr(2)](https://man7.org/linux/man-pages/man2/listxattr.2.html)
pub fn flistxattr(fd: RawFd) -> Result<Vec<OsString>> {
    // query the buffer size
    let buffer_size = match unsafe { libc::flistxattr(fd, null_mut(), 0) } {
        -1 => return Err(errno()),
        0 => return Ok(Vec::new()),
        buffer_size => buffer_size,
    };

    let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);
    let res = unsafe {
        libc::flistxattr(
            fd,
            buffer.as_mut_ptr().cast(),
            buffer.capacity(),
        )
    };

    match res {
        -1 => Err(errno()),
        len => {
            unsafe { buffer.set_len(len as usize) };
            Ok(buffer[..(len - 1) as usize]
                .split(|&item| item == 0)
                .map(OsStr::from_bytes)
                .map(|str| str.to_owned())
                .collect::<Vec<OsString>>())
        }
    }
}

/// Retrieves the value of the extended attribute identified by `name` and
/// associated with the given `path` in the filesystem. If `path` is a symbolic
/// link, it will be dereferenced.
///
/// For more information, see [getxattr(2)](https://man7.org/linux/man-pages/man2/getxattr.2.html)
pub fn getxattr<P, S>(path: P, name: S) -> Result<Vec<u8>>
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
{
    let name = match CString::new(name.as_ref().as_bytes()) {
        Ok(n) => n,
        _ => return Err(Errno(libc::EINVAL)),
    };
    let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
        Ok(n) => n,
        _ => return Err(Errno(libc::EINVAL)),
    };

    // query the buffer size
    let buffer_size = match unsafe {
        libc::getxattr(path.as_ptr(), name.as_ptr(), null_mut(), 0)
    } {
        -1 => return Err(errno()),
        0 => return Ok(Vec::new()),
        buffer_size => buffer_size,
    };

    let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);

    let res = unsafe {
        libc::getxattr(
            path.as_ptr(),
            name.as_ptr(),
            buffer.as_mut_ptr().cast(),
            buffer_size as usize,
        )
    };

    match res {
        -1 => Err(errno()),
        len => {
            unsafe { buffer.set_len(len as usize) };
            Ok(buffer)
        }
    }
}

/// Retrieves the value of the extended attribute identified by `name` and
/// associated with the given `path` in the filesystem. If `path` is a symbolic
/// link, the list of names associated with the link *itself* will be returned.
///
/// For more information, see [lgetxattr(2)](https://man7.org/linux/man-pages/man2/getxattr.2.html)
pub fn lgetxattr<P, S>(path: P, name: S) -> Result<Vec<u8>>
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
{
    let name = match CString::new(name.as_ref().as_bytes()) {
        Ok(n) => n,
        _ => return Err(Errno(libc::EINVAL)),
    };
    let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
        Ok(n) => n,
        _ => return Err(Errno(libc::EINVAL)),
    };

    // query the buffer size
    let buffer_size = match unsafe {
        libc::lgetxattr(path.as_ptr(), name.as_ptr(), null_mut(), 0)
    } {
        -1 => return Err(errno()),
        0 => return Ok(Vec::new()),
        buffer_size => buffer_size,
    };

    let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);

    let res = unsafe {
        libc::lgetxattr(
            path.as_ptr(),
            name.as_ptr(),
            buffer.as_mut_ptr().cast(),
            buffer_size as usize,
        )
    };

    match res {
        -1 => Err(errno()),
        len => {
            unsafe { buffer.set_len(len as usize) };
            Ok(buffer)
        }
    }
}

/// Retrieves the value of the extended attribute identified by `name` and
/// associated with the file specified by the open file descriptor `fd` in the
/// filesystem.
///
/// For more information, see [fgetxattr(2)](https://man7.org/linux/man-pages/man2/getxattr.2.html)
pub fn fgetxattr<S>(fd: RawFd, name: S) -> Result<Vec<u8>>
    where
        S: AsRef<OsStr>,
{
    let name = match CString::new(name.as_ref().as_bytes()) {
        Ok(name) => name,
        _ => return Err(Errno(libc::EINVAL)),
    };

    // query the buffer size
    let buffer_size =
        match unsafe { libc::fgetxattr(fd, name.as_ptr(), null_mut(), 0) } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            buffer_size => buffer_size,
        };

    let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);

    let res = unsafe {
        libc::fgetxattr(
            fd,
            name.as_ptr(),
            buffer.as_mut_ptr().cast(),
            buffer_size as usize,
        )
    };

    match res {
        -1 => Err(errno()),
        len => {
            unsafe { buffer.set_len(len as usize) };
            Ok(buffer)
        }
    }
}

/// Removes the extended attribute identified by `name` and associated with the
/// given `path` in the filesystem. If `path` is a symbolic link, it will be
/// dereferenced.
///
/// For more information, see [removexattr(2)](https://man7.org/linux/man-pages/man2/removexattr.2.html)
pub fn removexattr<P, S>(path: P, name: S) -> Result<()>
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
{
    let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
        Ok(n) => n,
        _ => return Err(Errno(libc::EINVAL)),
    };
    let name = match CString::new(name.as_ref().as_bytes()) {
        Ok(name) => name,
        _ => return Err(Errno(libc::EINVAL)),
    };

    let res = unsafe { libc::removexattr(path.as_ptr(), name.as_ptr()) };

    match res {
        -1 => Err(errno()),
        _ => Ok(()),
    }
}

/// Removes the extended attribute identified by `name` and associated with the
/// given `path` in the filesystem. If `path` is a symbolic link, extended
/// attribute is removed from the link *itself*.
///
/// For more information, see [lremovexattr(2)](https://man7.org/linux/man-pages/man2/removexattr.2.html)
pub fn lremovexattr<P, S>(path: P, name: S) -> Result<()>
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
{
    let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
        Ok(n) => n,
        _ => return Err(Errno(libc::EINVAL)),
    };
    let name = match CString::new(name.as_ref().as_bytes()) {
        Ok(name) => name,
        _ => return Err(Errno(libc::EINVAL)),
    };

    let res = unsafe { libc::lremovexattr(path.as_ptr(), name.as_ptr()) };

    match res {
        -1 => Err(errno()),
        _ => Ok(()),
    }
}

/// Removes the extended attribute identified by `name` and associated with the
/// file specified by the open file descriptor `fd`.
///
/// For more information, see [fremovexattr(2)](https://man7.org/linux/man-pages/man2/removexattr.2.html)
pub fn fremovexattr<S>(fd: RawFd, name: S) -> Result<()>
    where
        S: AsRef<OsStr>,
{
    let name = match CString::new(name.as_ref().as_bytes()) {
        Ok(name) => name,
        _ => return Err(Errno(libc::EINVAL)),
    };
    let res = unsafe { libc::fremovexattr(fd, name.as_ptr()) };

    match res {
        -1 => Err(errno()),
        _ => Ok(()),
    }
}

/// Sets the `value` of the extended attribute identified by `name` and associated
/// with the given `path` in the filesystem. If `path` is a symbolic link, it will
/// be dereferenced.
///
/// For more information, see [setxattr(2)](https://man7.org/linux/man-pages/man2/lsetxattr.2.html)
pub fn setxattr<P, S, B>(path: P, name: S, value: B, flags: Flags) -> Result<()>
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
        B: AsRef<[u8]>,
{
    let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
        Ok(n) => n,
        _ => return Err(Errno(libc::EINVAL)),
    };
    let name = match CString::new(name.as_ref().as_bytes()) {
        Ok(name) => name,
        _ => return Err(Errno(libc::EINVAL)),
    };

    let value_ptr = value.as_ref().as_ptr().cast();
    let value_len = value.as_ref().len();

    let res = unsafe {
        libc::setxattr(
            path.as_ptr(),
            name.as_ptr(),
            value_ptr,
            value_len,
            flags.bits(),
        )
    };

    match res {
        -1 => Err(errno()),
        _ => Ok(()),
    }
}

/// Sets the `value` of the extended attribute identified by `name` and associated
/// with the given `path` in the filesystem. If `path` is a symbolic link, the
/// extended attribute is set on the link *itself*.
///
/// For more information, see [lsetxattr(2)](https://man7.org/linux/man-pages/man2/lsetxattr.2.html)
pub fn lsetxattr<P, S, B>(
    path: P,
    name: S,
    value: B,
    flags: Flags,
) -> Result<()>
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
        B: AsRef<[u8]>,
{
    let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
        Ok(n) => n,
        _ => return Err(Errno(libc::EINVAL)),
    };
    let name = match CString::new(name.as_ref().as_bytes()) {
        Ok(name) => name,
        _ => return Err(Errno(libc::EINVAL)),
    };

    let value_ptr = value.as_ref().as_ptr().cast();
    let value_len = value.as_ref().len();

    let res = unsafe {
        libc::lsetxattr(
            path.as_ptr(),
            name.as_ptr(),
            value_ptr,
            value_len,
            flags.bits(),
        )
    };

    match res {
        -1 => Err(errno()),
        _ => Ok(()),
    }
}

/// Sets the `value` of the extended attribute identified by `name` and associated
/// with the file specified by the open file descriptor `fd`.
///
/// For more information, see [fsetxattr(2)](https://man7.org/linux/man-pages/man2/lsetxattr.2.html)
pub fn fsetxattr<S, B>(fd: RawFd, name: S, value: B, flags: Flags) -> Result<()>
    where
        S: AsRef<OsStr>,
        B: AsRef<[u8]>,
{
    let name = match CString::new(name.as_ref().as_bytes()) {
        Ok(name) => name,
        _ => return Err(Errno(libc::EINVAL)),
    };

    let value_ptr = value.as_ref().as_ptr().cast();
    let value_len = value.as_ref().len();

    let res = unsafe {
        libc::fsetxattr(fd, name.as_ptr(), value_ptr, value_len, flags.bits())
    };

    match res {
        -1 => Err(errno()),
        _ => Ok(()),
    }
}
