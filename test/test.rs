#[cfg(test)]
#[cfg(any(target_os = "linux", target_os = "android", target_os = "netbsd"))]
mod test_linux_android_netbsd {
    use errno::Errno;
    use extattr::{
        fgetxattr, flistxattr, fremovexattr, fsetxattr, getxattr, lgetxattr,
        listxattr, llistxattr, lremovexattr, lsetxattr, removexattr, setxattr,
        Result, SetxattrFlag,
    };
    use std::{fs::File, os::unix::io::AsRawFd};

    #[test]
    fn test_setxattr_file_exist() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_setxattr_file_exist");
        File::create(temp_file_path.as_path()).unwrap();

        let res = setxattr(
            temp_file_path.as_path(),
            "user.test_setxattr_file_exist",
            "",
            SetxattrFlag::empty(),
        );

        match res {
            // The underlying file system does not support EA, skip this test.
            Err(Errno(libc::ENOTSUP)) => {}
            // If EA is supported, then no error should occur
            _ => assert!(res.is_ok()),
        }
    }

    #[test]
    fn test_setxattr_file_not_exist() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path =
            temp_dir.path().join("test_setxattr_file_not_exist");

        let res = setxattr(
            temp_file_path.as_path(),
            "user.test_setxattr_file_not_exist",
            "",
            SetxattrFlag::empty(),
        );

        assert_eq!(res, Err(Errno(libc::ENOENT)));
    }

    #[test]
    fn test_fsetxattr() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_fsetxattr");
        let temp_file = File::create(temp_file_path.as_path()).unwrap();
        let temp_file_fd = temp_file.as_raw_fd();

        let res = fsetxattr(
            temp_file_fd,
            "user.test_fsetxattr",
            "",
            SetxattrFlag::empty(),
        );

        match res {
            // The underlying file system does not support EA, skip this test.
            Err(Errno(libc::ENOTSUP)) => {}
            // If EA is supported, then no error should occur
            _ => assert!(res.is_ok()),
        }
    }

    #[test]
    fn test_listxattr() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_listxattr");
        File::create(temp_file_path.as_path()).unwrap();

        let res = listxattr(temp_file_path.as_path());

        match res {
            // The underlying file system does not support EA, skip this test.
            Err(Errno(libc::ENOTSUP)) => {}
            // If EA is supported, then no error should occur
            _ => assert!(res.is_ok()),
        }
    }

    #[test]
    fn test_flistxattr() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_flistxattr");
        let temp_file = File::create(temp_file_path.as_path()).unwrap();
        let temp_file_fd = temp_file.as_raw_fd();

        let res = flistxattr(temp_file_fd);

        match res {
            // The underlying file system does not support EA, skip this test.
            Err(Errno(libc::ENOTSUP)) => {}
            // If EA is supported, then no error should occur
            _ => assert!(res.is_ok()),
        }
    }

    #[test]
    fn test_getxattr() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_getxattr");
        File::create(temp_file_path.as_path()).unwrap();

        let res = setxattr(
            temp_file_path.as_path(),
            "user.test_getxattr",
            "",
            SetxattrFlag::empty(),
        );

        // The underlying file system does not support EA, skip this test.
        if let Err(Errno(libc::ENOTSUP)) = res {
            return;
        }

        // If EA is supported, then no error should occur
        assert!(res.is_ok());

        assert_eq!(
            Ok(Vec::new()),
            getxattr(temp_file_path.as_path(), "user.test_getxattr")
        );
    }

    #[test]
    fn test_fgetxattr() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_fgetxattr");
        let temp_file = File::create(temp_file_path).unwrap();
        let temp_file_fd = temp_file.as_raw_fd();

        let res = fsetxattr(
            temp_file_fd,
            "user.test_fgetxattr",
            "",
            SetxattrFlag::empty(),
        );

        // The underlying file system does not support EA, skip this test.
        if let Err(Errno(libc::ENOTSUP)) = res {
            return;
        }

        // If EA is supported, then no error should occur
        assert!(res.is_ok());

        assert_eq!(
            Ok(Vec::new()),
            fgetxattr(temp_file_fd, "user.test_fgetxattr")
        );
    }

    #[test]
    fn test_removexattr_ea_exist() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_removexattr_ea_exist");
        File::create(temp_file_path.as_path()).unwrap();

        let res = setxattr(
            temp_file_path.as_path(),
            "user.test_removexattr_ea_exist",
            "",
            SetxattrFlag::empty(),
        );

        // The underlying file system does not support EA, skip this test.
        if let Err(Errno(libc::ENOTSUP)) = res {
            return;
        }

        // If EA is supported, then no error should occur
        assert!(res.is_ok());

        assert!(removexattr(
            temp_file_path.as_path(),
            "user.test_removexattr_ea_exist",
        )
        .is_ok());
    }

    #[test]
    fn test_removexattr_ea_not_exist() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path =
            temp_dir.path().join("test_removexattr_ea_not_exist");
        File::create(temp_file_path.as_path()).unwrap();

        // Here, we use `setxattr(path, "user.*", value, flags)` instead of `listxattr`
        // to test if EA is supported.
        //
        // This is necessary as we need to know whether `user` namespace EA is supported
        // rather than other categories of EA.
        //
        // For example, on `tmpfs`, `trusted` and `security` namespace EAs are
        // supported, but `user` is not.
        if let Err(Errno(libc::ENOTSUP)) = setxattr(
            temp_file_path.as_path(),
            "user.ea",
            "",
            SetxattrFlag::empty(),
        ) {
            // The underlying file system does not support EA, skip this test.
            return;
        }

        assert_eq!(
            Err(Errno(libc::ENODATA)),
            removexattr(
                temp_file_path.as_path(),
                "user.test_removexattr_ea_not_exist",
            )
        );
    }

    #[test]
    fn test_fremovexattr() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_fremovexattr");
        let temp_file = File::create(temp_file_path.as_path()).unwrap();
        let temp_file_fd = temp_file.as_raw_fd();

        let res = fsetxattr(
            temp_file_fd,
            "user.test_fremovexattr",
            "",
            SetxattrFlag::empty(),
        );

        // The underlying file system does not support EA, skip this test.
        if let Err(Errno(libc::ENOTSUP)) = res {
            return;
        }

        // If EA is supported, then no error should occur
        assert!(res.is_ok());

        assert!(fremovexattr(temp_file_fd, "user.test_fremovexattr").is_ok());
    }
}

#[cfg(test)]
#[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
mod test_freebsd_netbsd {}

#[cfg(test)]
#[cfg(any(target_os = "macos", target_os = "ios"))]
mod test_darwin {}
