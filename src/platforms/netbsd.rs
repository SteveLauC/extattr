//! EA syscall bindings for NetBSD

#![allow(unused, non_camel_case_types)]

// FIXME: Directly Re-export bindings from Linux and FreeBSD
//
// When the raw bindings for NetBSD in libc are fixed, we can directly re-use the code
// from Linux and FreeBSD modules.
//
// pub use super::freebsd::*;
// pub use super::linux_and_android::*;

pub use freebsd::*;
pub use linux::*;

// Currently, we have to make the bindings ourselves.
mod bindings {
    extern "C" {
        // bindings for Linux APIs
        pub fn getxattr(
            path: *const libc::c_char,
            name: *const libc::c_char,
            value: *mut libc::c_void,
            size: libc::size_t,
        ) -> libc::ssize_t;
        pub fn lgetxattr(
            path: *const libc::c_char,
            name: *const libc::c_char,
            value: *mut libc::c_void,
            size: libc::size_t,
        ) -> libc::ssize_t;
        pub fn fgetxattr(
            filedes: libc::c_int,
            name: *const libc::c_char,
            value: *mut libc::c_void,
            size: libc::size_t,
        ) -> libc::ssize_t;
        pub fn setxattr(
            path: *const libc::c_char,
            name: *const libc::c_char,
            value: *const libc::c_void,
            size: libc::size_t,
            flags: libc::c_int,
        ) -> libc::c_int;
        pub fn lsetxattr(
            path: *const libc::c_char,
            name: *const libc::c_char,
            value: *const libc::c_void,
            size: libc::size_t,
            flags: libc::c_int,
        ) -> libc::c_int;
        pub fn fsetxattr(
            filedes: libc::c_int,
            name: *const libc::c_char,
            value: *const libc::c_void,
            size: libc::size_t,
            flags: libc::c_int,
        ) -> libc::c_int;
        pub fn listxattr(
            path: *const libc::c_char,
            list: *mut libc::c_char,
            size: libc::size_t,
        ) -> libc::ssize_t;
        pub fn llistxattr(
            path: *const libc::c_char,
            list: *mut libc::c_char,
            size: libc::size_t,
        ) -> libc::ssize_t;
        pub fn flistxattr(
            filedes: libc::c_int,
            list: *mut libc::c_char,
            size: libc::size_t,
        ) -> libc::ssize_t;
        pub fn removexattr(
            path: *const libc::c_char,
            name: *const libc::c_char,
        ) -> libc::c_int;
        pub fn lremovexattr(
            path: *const libc::c_char,
            name: *const libc::c_char,
        ) -> libc::c_int;
        pub fn fremovexattr(
            filedes: libc::c_int,
            name: *const libc::c_char,
        ) -> libc::c_int;

        // Bindings for FreeBSD APIs
        pub fn extattr_delete_fd(
            fd: libc::c_int,
            attrnamespace: libc::c_int,
            attrname: *const libc::c_char,
        ) -> libc::c_int;
        pub fn extattr_delete_file(
            path: *const libc::c_char,
            attrnamespace: libc::c_int,
            attrname: *const libc::c_char,
        ) -> libc::c_int;
        pub fn extattr_delete_link(
            path: *const libc::c_char,
            attrnamespace: libc::c_int,
            attrname: *const libc::c_char,
        ) -> libc::c_int;
        pub fn extattr_get_fd(
            fd: libc::c_int,
            attrnamespace: libc::c_int,
            attrname: *const libc::c_char,
            data: *mut libc::c_void,
            nbytes: libc::size_t,
        ) -> libc::ssize_t;
        pub fn extattr_get_file(
            path: *const libc::c_char,
            attrnamespace: libc::c_int,
            attrname: *const libc::c_char,
            data: *mut libc::c_void,
            nbytes: libc::size_t,
        ) -> libc::ssize_t;
        pub fn extattr_get_link(
            path: *const libc::c_char,
            attrnamespace: libc::c_int,
            attrname: *const libc::c_char,
            data: *mut libc::c_void,
            nbytes: libc::size_t,
        ) -> libc::ssize_t;
        pub fn extattr_list_fd(
            fd: libc::c_int,
            attrnamespace: libc::c_int,
            data: *mut libc::c_void,
            nbytes: libc::size_t,
        ) -> libc::ssize_t;
        pub fn extattr_list_file(
            path: *const libc::c_char,
            attrnamespace: libc::c_int,
            data: *mut libc::c_void,
            nbytes: libc::size_t,
        ) -> libc::ssize_t;
        pub fn extattr_list_link(
            path: *const libc::c_char,
            attrnamespace: libc::c_int,
            data: *mut libc::c_void,
            nbytes: libc::size_t,
        ) -> libc::ssize_t;
        pub fn extattr_set_fd(
            fd: libc::c_int,
            attrnamespace: libc::c_int,
            attrname: *const libc::c_char,
            data: *const libc::c_void,
            nbytes: libc::size_t,
        ) -> libc::ssize_t;
        pub fn extattr_set_file(
            path: *const libc::c_char,
            attrnamespace: libc::c_int,
            attrname: *const libc::c_char,
            data: *const libc::c_void,
            nbytes: libc::size_t,
        ) -> libc::ssize_t;
        pub fn extattr_set_link(
            path: *const libc::c_char,
            attrnamespace: libc::c_int,
            attrname: *const libc::c_char,
            data: *const libc::c_void,
            nbytes: libc::size_t,
        ) -> libc::ssize_t;
    }
}

mod freebsd {
    use crate::{macros::libc_enum, Result};
    use errno::{errno, Errno};
    use std::{
        ffi::{CString, OsStr, OsString},
        os::unix::{ffi::OsStrExt, io::RawFd},
        path::Path,
        ptr::null_mut,
    };

    libc_enum! {
        /// Attribute Namespace of EA.
        #[repr(i32)]
        pub enum AttrNamespace {
            EXTATTR_NAMESPACE_USER,
            EXTATTR_NAMESPACE_SYSTEM,
        }
    }

    /// Deletes the extended attribute specified in `attrnamespace` and `attrname`
    /// for the file referred by the open file descriptor `fd`.
    ///
    /// For more information, see [extattr](https://www.freebsd.org/cgi/man.cgi?extattr).
    pub fn extattr_delete_fd<S: AsRef<OsStr>>(
        fd: RawFd,
        attrnamespace: AttrNamespace,
        attrname: S,
    ) -> Result<()> {
        let namespace = attrnamespace as libc::c_int;
        let attr_name = match CString::new(attrname.as_ref().as_bytes()) {
            Ok(n) => n,
            _ => return Err(Errno(libc::EINVAL)),
        };

        let res = unsafe {
            super::bindings::extattr_delete_fd(
                fd,
                namespace,
                attr_name.as_ptr() as *mut libc::c_char,
            )
        };

        match res {
            -1 => Err(errno()),
            _ => Ok(()),
        }
    }

    /// Deletes the extended attribute specified in `attrnamespace` and `attrname`
    /// for the file referred by `path`. If `path` is a symlink, it will be
    /// dereferenced.
    ///
    /// For more information, see [extattr](https://www.freebsd.org/cgi/man.cgi?extattr).
    pub fn extattr_delete_file<P, S>(
        path: P,
        attrnamespace: AttrNamespace,
        attrname: S,
    ) -> Result<()>
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
    {
        let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
            Ok(p) => p,
            _ => return Err(Errno(libc::EINVAL)),
        };
        let namespace = attrnamespace as libc::c_int;
        let attr_name = match CString::new(attrname.as_ref().as_bytes()) {
            Ok(n) => n,
            _ => return Err(Errno(libc::EINVAL)),
        };

        let res = unsafe {
            super::bindings::extattr_delete_file(
                path.as_ptr() as *mut libc::c_char,
                namespace,
                attr_name.as_ptr() as *mut libc::c_char,
            )
        };

        match res {
            -1 => Err(errno()),
            _ => Ok(()),
        }
    }

    /// Deletes the extended attribute specified in `attrnamespace` and `attrname`
    /// for the file referred by `path`. If `path` is a symlink, extended attribute
    /// will be removed from the link **itself**.
    ///
    /// For more information, see [extattr](https://www.freebsd.org/cgi/man.cgi?extattr).
    pub fn extattr_delete_link<P, S>(
        path: P,
        attrnamespace: AttrNamespace,
        attrname: S,
    ) -> Result<()>
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
    {
        let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
            Ok(p) => p,
            _ => return Err(Errno(libc::EINVAL)),
        };
        let namespace = attrnamespace as libc::c_int;
        let attr_name = match CString::new(attrname.as_ref().as_bytes()) {
            Ok(n) => n,
            _ => return Err(Errno(libc::EINVAL)),
        };

        let res = unsafe {
            super::bindings::extattr_delete_link(
                path.as_ptr() as *mut libc::c_char,
                namespace,
                attr_name.as_ptr() as *mut libc::c_char,
            )
        };

        match res {
            -1 => Err(errno()),
            _ => Ok(()),
        }
    }

    /// A function to help us parse the return value of `extattr_list_xx(2)`
    ///
    /// On FreeBSD, the return value of `extattr_list_xx(2)` is something like
    /// "\x08attrname\x0fanotherattrname"
    ///
    /// > Cite the man page:
    /// >
    /// > The `extattr_list_file()` returns a list of attributes present in the
    /// > requested namespace. **Each list entry consists of a single byte containing
    /// > the length of the attribute name, followed by the attribute name.** The
    /// > attribute name is not terminated by ASCII 0 (nul).
    ///
    /// We need to parse it into `vec!["attrname", "antherattrname"]`
    fn parse_ea_entries(bytes: &[u8]) -> Vec<OsString> {
        let mut ret = Vec::new();
        let mut idx = 0;
        let len = bytes.len();

        while idx < len {
            let entry_len = bytes[idx] as usize;
            ret.push(
                OsStr::from_bytes(&bytes[idx + 1..idx + entry_len + 1])
                    .to_owned(),
            );

            // update idx to parse the next entry
            idx += entry_len + 1;
        }

        ret
    }

    /// Returns a list of attribute names present in the requested namespace for
    /// the file specified in the open file descriptor `fd`.
    ///
    /// For more information, see [extattr](https://www.freebsd.org/cgi/man.cgi?extattr).
    pub fn extattr_list_fd(
        fd: RawFd,
        attrnamespace: AttrNamespace,
    ) -> Result<Vec<OsString>> {
        let namespace = attrnamespace as libc::c_int;

        // query the buffer size
        let buffer_size = match unsafe {
            super::bindings::extattr_list_fd(
                fd,
                namespace,
                null_mut() as *mut libc::c_void,
                0,
            )
        } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            size => size,
        };

        let mut buffer = Vec::with_capacity(buffer_size as usize);

        let res = unsafe {
            super::bindings::extattr_list_fd(
                fd,
                namespace,
                buffer.as_ptr() as *mut libc::c_void,
                buffer_size as libc::size_t,
            )
        };

        match res {
            -1 => Err(errno()),
            len => {
                unsafe { buffer.set_len(len as usize) };
                Ok(parse_ea_entries(&buffer))
            }
        }
    }

    /// Returns a list of attribute names present in the requested namespace for
    /// the file specified in `path`. If `path` is a symlink, it will be dereferenced.
    ///
    /// For more information, see [extattr](https://www.freebsd.org/cgi/man.cgi?extattr).
    pub fn extattr_list_file<P>(
        path: P,
        attrnamespace: AttrNamespace,
    ) -> Result<Vec<OsString>>
    where
        P: AsRef<Path>,
    {
        let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
            Ok(p) => p,
            _ => return Err(Errno(libc::EINVAL)),
        };
        let namespace = attrnamespace as libc::c_int;

        // query the buffer size
        let buffer_size = match unsafe {
            super::bindings::extattr_list_file(
                path.as_ptr() as *mut libc::c_char,
                namespace,
                null_mut() as *mut libc::c_void,
                0,
            )
        } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            size => size,
        };

        let mut buffer = Vec::with_capacity(buffer_size as usize);

        let res = unsafe {
            super::bindings::extattr_list_file(
                path.as_ptr() as *mut libc::c_char,
                namespace,
                buffer.as_ptr() as *mut libc::c_void,
                buffer_size as libc::size_t,
            )
        };

        match res {
            -1 => Err(errno()),
            len => {
                unsafe { buffer.set_len(len as usize) };
                Ok(parse_ea_entries(&buffer))
            }
        }
    }

    /// Returns a list of attribute names present in the requested namespace for
    /// the file specified in `path`. If `path` is a symlink, EA names for the link
    /// **itself** will be returned.
    ///
    /// For more information, see [extattr](https://www.freebsd.org/cgi/man.cgi?extattr).
    pub fn extattr_list_link<P>(
        path: P,
        attrnamespace: AttrNamespace,
    ) -> Result<Vec<OsString>>
    where
        P: AsRef<Path>,
    {
        let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
            Ok(p) => p,
            _ => return Err(Errno(libc::EINVAL)),
        };
        let namespace = attrnamespace as libc::c_int;

        // query the buffer size
        let buffer_size = match unsafe {
            super::bindings::extattr_list_link(
                path.as_ptr() as *mut libc::c_char,
                namespace,
                null_mut() as *mut libc::c_void,
                0,
            )
        } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            size => size,
        };

        let mut buffer = Vec::with_capacity(buffer_size as usize);

        let res = unsafe {
            super::bindings::extattr_list_link(
                path.as_ptr() as *mut libc::c_char,
                namespace,
                buffer.as_ptr() as *mut libc::c_void,
                buffer_size as libc::size_t,
            )
        };

        match res {
            -1 => Err(errno()),
            len => {
                unsafe { buffer.set_len(len as usize) };
                Ok(parse_ea_entries(&buffer))
            }
        }
    }

    /// Retrieves the value of the specified extended attribute for the file specified
    /// in the open file descriptor `fd`.
    ///
    /// For more information, see [extattr](https://www.freebsd.org/cgi/man.cgi?extattr).
    pub fn extattr_get_fd<S: AsRef<OsStr>>(
        fd: RawFd,
        attrnamespace: AttrNamespace,
        attrname: S,
    ) -> Result<Vec<u8>> {
        let namespace = attrnamespace as libc::c_int;
        let attrname = match CString::new(attrname.as_ref().as_bytes()) {
            Ok(n) => n,
            _ => return Err(Errno(libc::EINVAL)),
        };

        // query buffer size
        let buffer_size = match unsafe {
            super::bindings::extattr_get_fd(
                fd,
                namespace,
                attrname.as_ptr() as *mut libc::c_char,
                null_mut() as *mut libc::c_void,
                0,
            )
        } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            size => size,
        };
        let mut buffer = Vec::with_capacity(buffer_size as usize);

        let res = unsafe {
            super::bindings::extattr_get_fd(
                fd,
                namespace,
                attrname.as_ptr() as *mut libc::c_char,
                null_mut() as *mut libc::c_void,
                0,
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

    /// Retrieves the value of the specified extended attribute for the file specified
    /// in `path`. If `path` is a symlink, it will be dereferenced.
    ///
    /// For more information, see [extattr](https://www.freebsd.org/cgi/man.cgi?extattr).
    pub fn extattr_get_file<P, S>(
        path: P,
        attrnamespace: AttrNamespace,
        attrname: S,
    ) -> Result<Vec<u8>>
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
    {
        let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
            Ok(p) => p,
            _ => return Err(Errno(libc::EINVAL)),
        };
        let namespace = attrnamespace as libc::c_int;
        let attrname = match CString::new(attrname.as_ref().as_bytes()) {
            Ok(n) => n,
            _ => return Err(Errno(libc::EINVAL)),
        };

        // query buffer size
        let buffer_size = match unsafe {
            super::bindings::extattr_get_file(
                path.as_ptr() as *mut libc::c_char,
                namespace,
                attrname.as_ptr() as *mut libc::c_char,
                null_mut() as *mut libc::c_void,
                0,
            )
        } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            size => size,
        };
        let mut buffer = Vec::with_capacity(buffer_size as usize);

        let res = unsafe {
            super::bindings::extattr_get_file(
                path.as_ptr() as *mut libc::c_char,
                namespace,
                attrname.as_ptr() as *mut libc::c_char,
                null_mut() as *mut libc::c_void,
                0,
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

    /// Retrieves the value of the specified extended attribute for the file specified
    /// in `path`. If `path` is a symlink, EA value of the link **itself** will be
    /// returned.
    ///
    /// For more information, see [extattr](https://www.freebsd.org/cgi/man.cgi?extattr).
    pub fn extattr_get_link<P, S>(
        path: P,
        attrnamespace: AttrNamespace,
        attrname: S,
    ) -> Result<Vec<u8>>
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
    {
        let path = match CString::new(path.as_ref().as_os_str().as_bytes()) {
            Ok(p) => p,
            _ => return Err(Errno(libc::EINVAL)),
        };
        let namespace = attrnamespace as libc::c_int;
        let attrname = match CString::new(attrname.as_ref().as_bytes()) {
            Ok(n) => n,
            _ => return Err(Errno(libc::EINVAL)),
        };

        // query buffer size
        let buffer_size = match unsafe {
            super::bindings::extattr_get_link(
                path.as_ptr() as *mut libc::c_char,
                namespace,
                attrname.as_ptr() as *mut libc::c_char,
                null_mut() as *mut libc::c_void,
                0,
            )
        } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            size => size,
        };
        let mut buffer = Vec::with_capacity(buffer_size as usize);

        let res = unsafe {
            super::bindings::extattr_get_link(
                path.as_ptr() as *mut libc::c_char,
                namespace,
                attrname.as_ptr() as *mut libc::c_char,
                null_mut() as *mut libc::c_void,
                0,
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

    /// Sets the value of the specified extended attribute to the data described by
    /// `data` for the file referred by the open file descriptor `fd`.
    ///
    /// For more information, see [extattr](https://www.freebsd.org/cgi/man.cgi?extattr).
    pub fn extattr_set_fd<S, B>(
        fd: RawFd,
        attrnamespace: AttrNamespace,
        attrname: S,
        data: B,
    ) -> Result<()>
    where
        S: AsRef<OsStr>,
        B: AsRef<[u8]>,
    {
        let namespace = attrnamespace as libc::c_int;
        let attrname = match CString::new(attrname.as_ref().as_bytes()) {
            Ok(n) => n,
            _ => return Err(Errno(libc::EINVAL)),
        };
        let data_ptr = data.as_ref().as_ptr() as *mut libc::c_void;
        let data_len = data.as_ref().len();

        let res = unsafe {
            super::bindings::extattr_set_fd(
                fd,
                namespace,
                attrname.as_ptr() as *mut libc::c_char,
                data_ptr,
                data_len,
            )
        };

        match res {
            -1 => Err(errno()),
            _ => Ok(()),
        }
    }

    /// Sets the value of the specified extended attribute to the data described by
    /// `data` for the file referred by `path`. If `path` is a symlink, it will be
    /// dereferenced.
    ///
    /// For more information, see [extattr](https://www.freebsd.org/cgi/man.cgi?extattr).
    pub fn extattr_set_file<P, S, B>(
        path: P,
        attrnamespace: AttrNamespace,
        attrname: S,
        data: B,
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
        let namespace = attrnamespace as libc::c_int;
        let attrname = match CString::new(attrname.as_ref().as_bytes()) {
            Ok(n) => n,
            _ => return Err(Errno(libc::EINVAL)),
        };
        let data_ptr = data.as_ref().as_ptr() as *mut libc::c_void;
        let data_len = data.as_ref().len();

        let res = unsafe {
            super::bindings::extattr_set_file(
                path.as_ptr() as *mut libc::c_char,
                namespace,
                attrname.as_ptr() as *mut libc::c_char,
                data_ptr,
                data_len,
            )
        };

        match res {
            -1 => Err(errno()),
            _ => Ok(()),
        }
    }

    /// Sets the value of the specified extended attribute to the data described by
    /// `data` for the file referred by `path`. If `path` is a symlink, EA of the link
    /// **itself** will be set.
    ///
    /// For more information, see [extattr](https://www.freebsd.org/cgi/man.cgi?extattr).
    pub fn extattr_set_link<P, S, B>(
        path: P,
        attrnamespace: AttrNamespace,
        attrname: S,
        data: B,
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
        let namespace = attrnamespace as libc::c_int;
        let attrname = match CString::new(attrname.as_ref().as_bytes()) {
            Ok(n) => n,
            _ => return Err(Errno(libc::EINVAL)),
        };
        let data_ptr = data.as_ref().as_ptr() as *mut libc::c_void;
        let data_len = data.as_ref().len();

        let res = unsafe {
            super::bindings::extattr_set_link(
                path.as_ptr() as *mut libc::c_char,
                namespace,
                attrname.as_ptr() as *mut libc::c_char,
                data_ptr,
                data_len,
            )
        };

        match res {
            -1 => Err(errno()),
            _ => Ok(()),
        }
    }
}

mod linux {
    //! EA syscall bindings for Linux and Android

    use crate::{macros::libc_bitflags, Result};
    use errno::{errno, Errno};
    use std::{
        ffi::{CString, OsStr, OsString},
        os::unix::{ffi::OsStrExt, io::RawFd},
        path::Path,
        ptr::null_mut,
    };

    libc_bitflags! {
        /// `flags` used when setting EAs
        pub struct Flags: libc::c_int {
            /// Perform a pure create, which fails if the named attribute exists
            /// already.
            XATTR_CREATE;
            /// Perform a pure replace operation, which fails if the named attribute
            /// does not already exist.
            XATTR_REPLACE;
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
        let buffer_size = match unsafe {
            super::bindings::listxattr(path.as_ptr(), null_mut(), 0)
        } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            buffer_size => buffer_size,
        };

        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);
        let res = unsafe {
            super::bindings::listxattr(
                path.as_ptr(),
                buffer.as_ptr() as *mut libc::c_char,
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
        let buffer_size = match unsafe {
            super::bindings::llistxattr(path.as_ptr(), null_mut(), 0)
        } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            buffer_size => buffer_size,
        };

        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);
        let res = unsafe {
            super::bindings::listxattr(
                path.as_ptr(),
                buffer.as_ptr() as *mut libc::c_char,
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
        let buffer_size =
            match unsafe { super::bindings::flistxattr(fd, null_mut(), 0) } {
                -1 => return Err(errno()),
                0 => return Ok(Vec::new()),
                buffer_size => buffer_size,
            };

        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);
        let res = unsafe {
            super::bindings::flistxattr(
                fd,
                buffer.as_ptr() as *mut libc::c_char,
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
            super::bindings::getxattr(
                path.as_ptr(),
                name.as_ptr() as *mut libc::c_char,
                null_mut(),
                0,
            )
        } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            buffer_size => buffer_size,
        };

        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);

        let res = unsafe {
            super::bindings::getxattr(
                path.as_ptr() as *mut libc::c_char,
                name.as_ptr() as *mut libc::c_char,
                buffer.as_ptr() as *mut libc::c_void,
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
            super::bindings::lgetxattr(
                path.as_ptr(),
                name.as_ptr() as *mut libc::c_char,
                null_mut(),
                0,
            )
        } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            buffer_size => buffer_size,
        };

        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);

        let res = unsafe {
            super::bindings::lgetxattr(
                path.as_ptr() as *mut libc::c_char,
                name.as_ptr() as *mut libc::c_char,
                buffer.as_ptr() as *mut libc::c_void,
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
        let buffer_size = match unsafe {
            super::bindings::fgetxattr(
                fd,
                name.as_ptr() as *mut libc::c_char,
                null_mut(),
                0,
            )
        } {
            -1 => return Err(errno()),
            0 => return Ok(Vec::new()),
            buffer_size => buffer_size,
        };

        let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size as usize);

        let res = unsafe {
            super::bindings::fgetxattr(
                fd,
                name.as_ptr() as *mut libc::c_char,
                buffer.as_ptr() as *mut libc::c_void,
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

        let res = unsafe {
            super::bindings::removexattr(
                path.as_ptr() as *mut libc::c_char,
                name.as_ptr(),
            )
        };

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

        let res = unsafe {
            super::bindings::lremovexattr(
                path.as_ptr() as *mut libc::c_char,
                name.as_ptr(),
            )
        };

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
        let res = unsafe { super::bindings::fremovexattr(fd, name.as_ptr()) };

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
    pub fn setxattr<P, S, B>(
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

        let value_ptr = value.as_ref().as_ptr() as *mut libc::c_void;
        let value_len = value.as_ref().len();

        let res = unsafe {
            super::bindings::setxattr(
                path.as_ptr() as *mut libc::c_char,
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

        let value_ptr = value.as_ref().as_ptr() as *mut libc::c_void;
        let value_len = value.as_ref().len();

        let res = unsafe {
            super::bindings::setxattr(
                path.as_ptr() as *mut libc::c_char,
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
    pub fn fsetxattr<S, B>(
        fd: RawFd,
        name: S,
        value: B,
        flags: Flags,
    ) -> Result<()>
    where
        S: AsRef<OsStr>,
        B: AsRef<[u8]>,
    {
        let name = match CString::new(name.as_ref().as_bytes()) {
            Ok(name) => name,
            _ => return Err(Errno(libc::EINVAL)),
        };

        let value_ptr = value.as_ref().as_ptr() as *mut libc::c_void;
        let value_len = value.as_ref().len();

        let res = unsafe {
            super::bindings::fsetxattr(
                fd,
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
}
