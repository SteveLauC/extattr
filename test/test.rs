#[cfg(test)]
#[cfg(any(target_os = "linux", target_os = "android", target_os = "netbsd"))]
mod test_linux_android_netbsd {
    use errno::Errno;
    use extattr::{
        fgetxattr, flistxattr, fremovexattr, fsetxattr, getxattr, listxattr,
        removexattr, setxattr, SetxattrFlag,
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

    /// Test flag `XATTR_CREATE`
    #[test]
    fn test_setxattr_xattr_create() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_setxattr_xattr_create");
        File::create(temp_file_path.as_path()).unwrap();

        // set it first
        let res_set = setxattr(
            temp_file_path.as_path(),
            "user.test_setxattr_xattr_create",
            "",
            SetxattrFlag::empty(),
        );

        // EA is not supported on the underlying file system, skip the test.
        if let Err(Errno(libc::ENOTSUP)) = res_set {
            return;
        }

        // Then try to set it again, with `XATTR_CREATE` flag
        let res = setxattr(
            temp_file_path.as_path(),
            "user.test_setxattr_xattr_create",
            "",
            SetxattrFlag::XATTR_CREATE,
        );

        assert_eq!(res, Err(Errno(libc::EEXIST)));
    }

    /// Test flag `XATTR_REPLACE`
    #[test]
    fn test_setxattr_xattr_replace() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path =
            temp_dir.path().join("test_setxattr_xattr_replace");
        File::create(temp_file_path.as_path()).unwrap();

        let res = setxattr(
            temp_file_path.as_path(),
            "user.test_setxattr_xattr_replace",
            "",
            SetxattrFlag::XATTR_REPLACE,
        );

        match res {
            // The underlying file system does not support EA, skip this test.
            Err(Errno(libc::ENOTSUP)) => {}
            _ => assert_eq!(res, Err(Errno(libc::ENODATA))),
        }
    }

    #[test]
    fn test_setxattr_invalid_namespace() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path =
            temp_dir.path().join("test_setxattr_invalid_namespace");
        File::create(temp_file_path.as_path()).unwrap();

        let res = setxattr(
            temp_file_path.as_path(),
            "xxxx.test_setxattr_invalid_namespace",
            "",
            SetxattrFlag::empty(),
        );

        assert_eq!(res, Err(Errno(libc::ENOTSUP)));
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
    fn test_getxattr_attribute_does_not_exist() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir
            .path()
            .join("test_getxattr_attribute_does_not_exist");
        File::create(temp_file_path.as_path()).unwrap();

        let res = getxattr(
            temp_file_path.as_path(),
            "user.test_getxattr_attribute_does_not_exist",
        );

        match res {
            // EA is not supported, skip the test.
            Err(Errno(libc::ENOTSUP)) => {}
            _ => assert_eq!(res, Err(Errno(libc::ENODATA))),
        }
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
        // to test if EA is supported because on some file system (e.g., tmpfs), `user`
        // EA is not supported but `trusted` and `security` EA are. Since we test
        // `removexattr` using `user` EA, we need to know if `user` EA is supported on
        // the underlying file system.
        if let Err(Errno(libc::ENOTSUP)) = setxattr(
            temp_file_path.as_path(),
            "user.ea",
            "",
            SetxattrFlag::empty(),
        ) {
            // The underlying file system does not support user EA, skip this test.
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
mod test_freebsd_netbsd {
    use errno::Errno;
    use extattr::{
        extattr_delete_fd, extattr_delete_file, extattr_get_fd,
        extattr_get_file, extattr_list_fd, extattr_list_file, extattr_set_fd,
        extattr_set_file, AttrNamespace,
    };
    use std::{fs::File, os::unix::io::AsRawFd};

    #[test]
    fn test_extattr_set_file_file_exist() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path =
            temp_dir.path().join("test_extattr_set_file_file_exist");
        File::create(temp_file_path.as_path()).unwrap();

        extattr_set_file(
            temp_file_path.as_path(),
            AttrNamespace::EXTATTR_NAMESPACE_USER,
            "test_extattr_set_file_file_exist",
            "",
        )
        .unwrap();
    }

    #[test]
    fn test_extattr_set_file_file_not_exist() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path =
            temp_dir.path().join("test_extattr_set_file_file_not_exist");

        assert_eq!(
            extattr_set_file(
                temp_file_path.as_path(),
                AttrNamespace::EXTATTR_NAMESPACE_USER,
                "test_extattr_set_file_file_not_exist",
                "",
            ),
            Err(Errno(libc::ENOENT))
        );
    }

    #[test]
    fn test_extattr_set_fd() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_extattr_set_fd");
        let temp_file = File::create(temp_file_path.as_path()).unwrap();
        let temp_file_fd = temp_file.as_raw_fd();

        extattr_set_fd(
            temp_file_fd,
            AttrNamespace::EXTATTR_NAMESPACE_USER,
            "test_extattr_set_fd",
            "",
        )
        .unwrap();
    }

    #[test]
    fn test_extattr_list_file() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_extattr_list_file");
        File::create(temp_file_path.as_path()).unwrap();

        extattr_list_file(
            temp_file_path.as_path(),
            AttrNamespace::EXTATTR_NAMESPACE_USER,
        )
        .unwrap();
    }

    #[test]
    fn test_extattr_list_fd() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_extattr_list_fd");
        let temp_file = File::create(temp_file_path.as_path()).unwrap();
        let temp_file_fd = temp_file.as_raw_fd();

        extattr_list_fd(temp_file_fd, AttrNamespace::EXTATTR_NAMESPACE_USER)
            .unwrap();
    }

    #[test]
    fn test_extattr_get_file() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_extattr_get_file");
        File::create(temp_file_path.as_path()).unwrap();

        extattr_set_file(
            temp_file_path.as_path(),
            AttrNamespace::EXTATTR_NAMESPACE_USER,
            "test_extattr_get_file",
            "",
        )
        .unwrap();

        assert_eq!(
            Ok(Vec::new()),
            extattr_get_file(
                temp_file_path.as_path(),
                AttrNamespace::EXTATTR_NAMESPACE_USER,
                "test_extattr_get_file",
            )
        );
    }

    #[test]
    fn test_extattr_get_fd() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_extattr_get_fd");
        let temp_file = File::create(temp_file_path.as_path()).unwrap();
        let temp_file_fd = temp_file.as_raw_fd();

        extattr_set_fd(
            temp_file_fd,
            AttrNamespace::EXTATTR_NAMESPACE_USER,
            "test_extattr_get_fd",
            "",
        )
        .unwrap();

        assert_eq!(
            Ok(Vec::new()),
            extattr_get_fd(
                temp_file_fd,
                AttrNamespace::EXTATTR_NAMESPACE_USER,
                "test_extattr_get_fd",
            )
        );
    }

    #[test]
    fn test_extattr_get_file_ea_not_exist() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path =
            temp_dir.path().join("test_extattr_get_file_ea_not_exist");
        File::create(temp_file_path.as_path()).unwrap();

        assert_eq!(
            extattr_get_file(
                temp_file_path,
                AttrNamespace::EXTATTR_NAMESPACE_USER,
                "test_extattr_get_file_ea_not_exist",
            ),
            Err(Errno(libc::ENOATTR))
        );
    }

    #[test]
    fn test_extattr_delete_file_ea_exist() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path =
            temp_dir.path().join("test_extattr_delete_file_ea_exist");
        File::create(temp_file_path.as_path()).unwrap();

        extattr_set_file(
            temp_file_path.as_path(),
            AttrNamespace::EXTATTR_NAMESPACE_USER,
            "test_extattr_delete_file_ea_exist",
            "",
        )
        .unwrap();

        extattr_delete_file(
            temp_file_path.as_path(),
            AttrNamespace::EXTATTR_NAMESPACE_USER,
            "test_extattr_delete_file_ea_exist",
        )
        .unwrap();
    }

    #[test]
    fn test_extattr_delete_file_ea_not_exist() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir
            .path()
            .join("test_extattr_delete_file_ea_not_exist");
        File::create(temp_file_path.as_path()).unwrap();

        assert_eq!(
            extattr_delete_file(
                temp_file_path.as_path(),
                AttrNamespace::EXTATTR_NAMESPACE_USER,
                "test_extattr_delete_file_ea_not_exist",
            ),
            Err(Errno(libc::ENOATTR))
        );
    }

    #[test]
    fn test_extattr_delete_fd() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_extattr_delete_fd");
        let temp_file = File::create(temp_file_path.as_path()).unwrap();
        let temp_file_fd = temp_file.as_raw_fd();

        extattr_set_fd(
            temp_file_fd,
            AttrNamespace::EXTATTR_NAMESPACE_USER,
            "test_extattr_delete_fd",
            "",
        )
        .unwrap();

        assert!(extattr_delete_fd(
            temp_file_fd,
            AttrNamespace::EXTATTR_NAMESPACE_USER,
            "test_extattr_delete_fd",
        )
        .is_ok());
    }
}

#[cfg(test)]
#[cfg(any(target_os = "macos", target_os = "ios"))]
mod test_darwin {
    use errno::Errno;
    use extattr::{
        fgetxattr, flistxattr, fremovexattr, fsetxattr, getxattr, listxattr,
        removexattr, setxattr, Options,
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
            0,
            Options::empty(),
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
            0,
            Options::empty(),
        );

        assert_eq!(res, Err(Errno(libc::ENOENT)));
    }

    /// Test flag `XATTR_CREATE`
    #[test]
    fn test_setxattr_xattr_create() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir.path().join("test_setxattr_xattr_create");
        File::create(temp_file_path.as_path()).unwrap();

        // set it first
        let res_set = setxattr(
            temp_file_path.as_path(),
            "user.test_setxattr_xattr_create",
            "",
            0,
            Options::empty(),
        );

        // EA is not supported on the underlying file system, skip the test.
        if let Err(Errno(libc::ENOTSUP)) = res_set {
            return;
        }

        // Then try to set it again, with `XATTR_CREATE` flag
        let res = setxattr(
            temp_file_path.as_path(),
            "user.test_setxattr_xattr_create",
            "",
            0,
            Options::XATTR_CREATE,
        );

        assert_eq!(res, Err(Errno(libc::EEXIST)));
    }

    /// Test flag `XATTR_REPLACE`
    #[test]
    fn test_setxattr_xattr_replace() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path =
            temp_dir.path().join("test_setxattr_xattr_replace");
        File::create(temp_file_path.as_path()).unwrap();

        let res = setxattr(
            temp_file_path.as_path(),
            "user.test_setxattr_xattr_replace",
            "",
            0,
            Options::XATTR_REPLACE,
        );

        match res {
            // The underlying file system does not support EA, skip this test.
            Err(Errno(libc::ENOTSUP)) => {}
            _ => assert_eq!(res, Err(Errno(libc::ENOATTR))),
        }
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
            0,
            Options::empty(),
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

        let res = listxattr(temp_file_path.as_path(), Options::empty());

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

        let res = flistxattr(temp_file_fd, Options::empty());

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
            0,
            Options::empty(),
        );

        // The underlying file system does not support EA, skip this test.
        if let Err(Errno(libc::ENOTSUP)) = res {
            return;
        }

        // If EA is supported, then no error should occur
        assert!(res.is_ok());

        assert_eq!(
            Ok(Vec::new()),
            getxattr(
                temp_file_path.as_path(),
                "user.test_getxattr",
                0,
                Options::empty()
            )
        );
    }

    #[test]
    fn test_getxattr_attribute_does_not_exist() {
        let temp_dir = tempfile::tempdir_in("./").unwrap();
        let temp_file_path = temp_dir
            .path()
            .join("test_getxattr_attribute_does_not_exist");
        File::create(temp_file_path.as_path()).unwrap();

        let res = getxattr(
            temp_file_path.as_path(),
            "user.test_getxattr_attribute_does_not_exist",
            0,
            Options::empty(),
        );

        match res {
            // EA is not supported, skip the test.
            Err(Errno(libc::ENOTSUP)) => {}
            _ => assert_eq!(res, Err(Errno(libc::ENOATTR))),
        }
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
            0,
            Options::empty(),
        );

        // The underlying file system does not support EA, skip this test.
        if let Err(Errno(libc::ENOTSUP)) = res {
            return;
        }

        // If EA is supported, then no error should occur
        assert!(res.is_ok());

        assert_eq!(
            Ok(Vec::new()),
            fgetxattr(temp_file_fd, "user.test_fgetxattr", 0, Options::empty())
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
            0,
            Options::empty(),
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
            Options::empty(),
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
        // to test if EA is supported because on some file system (e.g., tmpfs), `user`
        // EA is not supported but `trusted` and `security` EA are. Since we test
        // `removexattr` using `user` EA, we need to know if `user` EA is supported on
        // the underlying file system.
        if let Err(Errno(libc::ENOTSUP)) = setxattr(
            temp_file_path.as_path(),
            "user.ea",
            "",
            0,
            Options::empty(),
        ) {
            // The underlying file system does not support user EA, skip this test.
            return;
        }

        assert_eq!(
            Err(Errno(libc::ENOATTR)),
            removexattr(
                temp_file_path.as_path(),
                "user.test_removexattr_ea_not_exist",
                Options::empty(),
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
            0,
            Options::empty(),
        );

        // The underlying file system does not support EA, skip this test.
        if let Err(Errno(libc::ENOTSUP)) = res {
            return;
        }

        // If EA is supported, then no error should occur
        assert!(res.is_ok());

        assert!(fremovexattr(
            temp_file_fd,
            "user.test_fremovexattr",
            Options::empty()
        )
        .is_ok());
    }
}
