use super::*;

const FULL: u32 = 0x001F_01FF;
const READ_AND_EXECUTE: u32 = 0x0012_00A9;

fn allow(principal: Principal, mask: u32) -> Ace {
    Ace {
        kind: ACCESS_ALLOWED,
        flags: 0,
        mask,
        principal,
    }
}

fn of_kind(kind: u8) -> Ace {
    Ace {
        kind,
        flags: 0,
        mask: 0,
        principal: Principal::Other,
    }
}

/// What a file saved in a user's own profile folder carries by default.
fn profile_default() -> Vec<Ace> {
    vec![
        allow(Principal::ThisUser, FULL),
        allow(Principal::System, FULL),
        allow(Principal::Administrators, FULL),
    ]
}

#[test]
fn a_file_owned_by_this_user_or_administrators_with_the_profile_default_is_private() {
    for owner in [Principal::ThisUser, Principal::Administrators] {
        assert_eq!(admit(owner, Some(&profile_default())), Ok(()), "{owner:?}");
    }
    // An empty list grants no one anything.
    assert_eq!(admit(Principal::ThisUser, Some(&[])), Ok(()));
    let mut owner_rights = profile_default();
    owner_rights.push(allow(Principal::OwnerRights, FULL));
    assert_eq!(admit(Principal::ThisUser, Some(&owner_rights)), Ok(()));
}

#[test]
fn a_file_another_principal_owns_is_not_private() {
    for owner in [Principal::Other, Principal::System, Principal::OwnerRights] {
        assert_eq!(
            admit(owner, Some(&profile_default())),
            Err(NotPrivate::Owner),
            "{owner:?}"
        );
    }
}

#[test]
fn a_file_with_no_access_list_is_open_to_everyone() {
    assert_eq!(
        admit(Principal::ThisUser, None),
        Err(NotPrivate::NoAccessList)
    );
}

#[test]
fn any_right_but_the_harmless_ones_for_another_principal_refuses() {
    // ProgramData's default: Users may read and execute.
    let mut shared = profile_default();
    shared.push(allow(Principal::Other, READ_AND_EXECUTE));
    assert_eq!(
        admit(Principal::ThisUser, Some(&shared)),
        Err(NotPrivate::Shared)
    );
    for mask in [
        0x0000_0001, // FILE_READ_DATA
        0x0000_0002, // FILE_WRITE_DATA
        0x0000_0004, // FILE_APPEND_DATA
        0x0000_0020, // FILE_EXECUTE
        0x0001_0000, // DELETE
        0x0004_0000, // WRITE_DAC
        0x0008_0000, // WRITE_OWNER
        0x0200_0000, // MAXIMUM_ALLOWED
        0x1000_0000, // GENERIC_ALL
        0x8000_0000, // GENERIC_READ
        0x0000_4000, // a bit with no name here
    ] {
        let entries = [allow(Principal::Other, mask)];
        assert_eq!(
            admit(Principal::ThisUser, Some(&entries)),
            Err(NotPrivate::Shared),
            "{mask:#x}"
        );
    }
    let harmless = [allow(Principal::Other, HARMLESS_RIGHTS)];
    assert_eq!(admit(Principal::ThisUser, Some(&harmless)), Ok(()));
}

#[test]
fn an_entry_that_applies_only_to_what_a_folder_creates_is_ignored() {
    let inherited_only = [Ace {
        flags: INHERIT_ONLY,
        ..allow(Principal::Other, FULL)
    }];
    assert_eq!(admit(Principal::ThisUser, Some(&inherited_only)), Ok(()));
    let unknown_inherited_only = [Ace {
        flags: INHERIT_ONLY,
        ..of_kind(17)
    }];
    assert_eq!(
        admit(Principal::ThisUser, Some(&unknown_inherited_only)),
        Ok(())
    );
}

#[test]
fn deny_entries_are_ignored_and_any_other_kind_refuses() {
    for kind in ACCESS_DENIED_KINDS {
        let entries = [of_kind(kind), allow(Principal::ThisUser, FULL)];
        assert_eq!(admit(Principal::ThisUser, Some(&entries)), Ok(()), "{kind}");
    }
    // Compound, object, callback and callback-object allow entries, an audit
    // entry and a label: none is interpreted, so each refuses.
    for kind in [2, 4, 5, 9, 11, 17] {
        let entries = [of_kind(kind)];
        assert_eq!(
            admit(Principal::ThisUser, Some(&entries)),
            Err(NotPrivate::UnknownEntry),
            "{kind}"
        );
    }
}

#[cfg(windows)]
mod on_windows {
    use super::super::windows::{descriptor, owner_of, read_entries, with_user_sid};
    use super::super::*;
    use std::fs::{self, File};
    use std::path::{Path, PathBuf};
    use windows_sys::Win32::Foundation::LocalFree;
    use windows_sys::Win32::Security::Authorization::ConvertSecurityDescriptorToStringSecurityDescriptorW;
    use windows_sys::Win32::Security::{DACL_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION};

    /// The file's owner and access list as SDDL, printed so each run records the
    /// runner's real state rather than this test's assumption about it.
    fn sddl(path: &Path) -> String {
        let file = File::open(path).unwrap();
        let (held, _, _) = descriptor(&file).unwrap();
        let mut text = std::ptr::null_mut();
        let mut length = 0u32;
        // SAFETY: the descriptor is live; both out pointers are writable.
        let converted = unsafe {
            ConvertSecurityDescriptorToStringSecurityDescriptorW(
                held.0,
                1,
                OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
                &mut text,
                &mut length,
            )
        };
        assert_ne!(converted, 0, "{}", path.display());
        // SAFETY: the call wrote `length` UTF-16 units, its terminator included.
        let units = unsafe { std::slice::from_raw_parts(text, length as usize) };
        let sddl = String::from_utf16_lossy(units)
            .trim_end_matches('\0')
            .to_string();
        // SAFETY: the string was allocated with LocalAlloc by the call above.
        unsafe { LocalFree(text.cast()) };
        println!("{}: {sddl}", path.display());
        sddl
    }

    #[test]
    fn the_constants_are_windows_own() {
        use windows_sys::Win32::Security::INHERIT_ONLY_ACE;
        use windows_sys::Win32::Storage::FileSystem::{
            FILE_READ_ATTRIBUTES, FILE_READ_EA, READ_CONTROL, SYNCHRONIZE,
        };
        use windows_sys::Win32::System::SystemServices::{
            ACCESS_ALLOWED_ACE_TYPE, ACCESS_DENIED_ACE_TYPE, ACCESS_DENIED_CALLBACK_ACE_TYPE,
            ACCESS_DENIED_CALLBACK_OBJECT_ACE_TYPE, ACCESS_DENIED_OBJECT_ACE_TYPE,
        };
        assert_eq!(u32::from(ACCESS_ALLOWED), ACCESS_ALLOWED_ACE_TYPE);
        assert_eq!(
            ACCESS_DENIED_KINDS.map(u32::from),
            [
                ACCESS_DENIED_ACE_TYPE,
                ACCESS_DENIED_OBJECT_ACE_TYPE,
                ACCESS_DENIED_CALLBACK_ACE_TYPE,
                ACCESS_DENIED_CALLBACK_OBJECT_ACE_TYPE,
            ]
        );
        assert_eq!(u32::from(INHERIT_ONLY), INHERIT_ONLY_ACE);
        assert_eq!(
            HARMLESS_RIGHTS,
            SYNCHRONIZE | READ_CONTROL | FILE_READ_ATTRIBUTES | FILE_READ_EA
        );
    }

    /// An allow entry whose SID claims more bytes than the entry holds is never
    /// compared: the whole list is unreadable. Built by hand, as no file holds one.
    #[test]
    fn an_entry_too_short_for_its_sid_is_not_read() {
        // ACL header: revision 2, size, one entry. Then one allow entry of 12
        // bytes (header, mask, the SID's first 4 bytes), whose SID claims one
        // sub-authority, so 12 bytes of SID where the entry has room for 4.
        // Zero padding past the list's 20 bytes keeps any read of the claimed SID
        // inside this array.
        let mut words = [0u32; 16];
        let bytes: [u8; 20] = [
            2, 0, 20, 0, 1, 0, 0, 0, // ACL: revision, sbz1, size 20, count 1, sbz2
            0, 0, 12, 0, // ACE header: allow, no flags, size 12
            0x01, 0, 0, 0, // mask: FILE_READ_DATA
            1, 1, 0, 0, // SID: revision 1, one sub-authority, authority begins
        ];
        for (word, chunk) in words.iter_mut().zip(bytes.chunks_exact(4)) {
            *word = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        let list = words.as_mut_ptr().cast();
        let read = with_user_sid(|user| read_entries(list, user)).unwrap();
        assert_eq!(
            read.map(|_| ()).map_err(|error| error.kind()),
            Err(std::io::ErrorKind::InvalidData)
        );
    }

    #[test]
    fn a_file_this_test_saves_in_its_temp_folder_is_private() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("password.txt");
        fs::write(&path, b"x").unwrap();
        sddl(&path);
        let file = File::open(&path).unwrap();
        assert_eq!(private_to_this_user(&file).unwrap(), Ok(()));
    }

    /// ProgramData lets Users create folders, and a file created in one inherits
    /// Users' read: no access list is changed by the test.
    #[test]
    fn a_file_saved_where_users_may_read_is_shared() {
        let root = PathBuf::from(std::env::var_os("ProgramData").expect("ProgramData is set"));
        let directory = tempfile::Builder::new()
            .prefix("complyeaze-bridge-access-test-")
            .tempdir_in(&root)
            .unwrap();
        let path = directory.path().join("password.txt");
        fs::write(&path, b"x").unwrap();
        sddl(&path);
        let file = File::open(&path).unwrap();
        // Ours: this user, or Administrators when the runner stamps that.
        let owner = owner_of(&file).unwrap();
        assert!(
            matches!(owner, Principal::ThisUser | Principal::Administrators),
            "{owner:?}"
        );
        assert_eq!(
            private_to_this_user(&file).unwrap(),
            Err(NotPrivate::Shared)
        );
    }

    /// A system file owned by neither this user nor Administrators, picked at
    /// run time; the test fails, naming each candidate's owner, if none is.
    #[test]
    fn a_file_another_principal_owns_is_refused_for_its_owner() {
        let root = PathBuf::from(std::env::var_os("SystemRoot").expect("SystemRoot is set"));
        let candidates = [
            root.join(r"System32\drivers\etc\hosts"),
            root.join("win.ini"),
        ];
        let mut seen = Vec::new();
        for path in &candidates {
            let Ok(file) = File::open(path) else {
                seen.push(format!("{}: not opened", path.display()));
                continue;
            };
            let owner = owner_of(&file).unwrap();
            seen.push(format!("{}: {owner:?} {}", path.display(), sddl(path)));
            if owner == Principal::Other {
                assert_eq!(private_to_this_user(&file).unwrap(), Err(NotPrivate::Owner));
                return;
            }
        }
        panic!("no candidate is owned by another principal: {seen:?}");
    }
}
