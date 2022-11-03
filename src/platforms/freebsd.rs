#![allow(non_camel_case_types)]

use crate::{macros::libc_enum, Result};
use errno::{errno, Errno};
use std::{
    ffi::{CString, OsStr, OsString},
    os::unix::{ffi::OsStrExt, io::RawFd},
    path::Path,
    ptr::null_mut,
};

libc_enum! {
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
        libc::extattr_delete_fd(
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
        libc::extattr_delete_file(
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
        libc::extattr_delete_link(
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
            OsStr::from_bytes(&bytes[idx + 1..idx + entry_len + 1]).to_owned(),
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
        libc::extattr_list_fd(fd, namespace, null_mut() as *mut libc::c_void, 0)
    } {
        -1 => return Err(errno()),
        0 => return Ok(Vec::new()),
        size => size,
    };

    let mut buffer = Vec::with_capacity(buffer_size as usize);

    let res = unsafe {
        libc::extattr_list_fd(
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
        libc::extattr_list_file(
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
        libc::extattr_list_file(
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
        libc::extattr_list_link(
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
        libc::extattr_list_link(
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
        libc::extattr_get_fd(
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
        libc::extattr_get_fd(
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
        libc::extattr_get_file(
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
        libc::extattr_get_file(
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
        libc::extattr_get_link(
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
        libc::extattr_get_link(
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
        libc::extattr_set_fd(
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
        libc::extattr_set_file(
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
        libc::extattr_set_link(
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

#[cfg(test)]
mod test {
    use std::ffi::{OsStr, OsString};

    #[test]
    fn test_parse_ea_entries() {
        let list = "\x08attrname\x0fanotherattrname";
        let ret = super::parse_ea_entries(list.as_bytes());

        assert_eq!(
            vec![
                OsStr::new("attrname").to_owned(),
                OsStr::new("anotherattrname").to_owned()
            ],
            ret
        );
    }
}
