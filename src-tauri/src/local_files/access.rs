//! Whether a file is private to the user running ComplyEaze Bridge, judged by its
//! owner and access list. The Windows counterpart of the Unix password-file rule
//! (owned by this user, no group or other permission bits). The decision is a
//! pure function of a parsed list, so it is tested on every platform; only the
//! reading of the list is Windows code.

/// Who an owner or an access entry names, as far as this check cares.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Principal {
    ThisUser,
    /// The operating system (S-1-5-18).
    System,
    /// BUILTIN\Administrators (S-1-5-32-544).
    Administrators,
    /// OWNER RIGHTS (S-1-3-4): whoever owns the file, once the owner is admitted.
    OwnerRights,
    Other,
}

/// One entry of a file's access list, as read.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Ace {
    pub(crate) kind: u8,
    pub(crate) flags: u8,
    /// Read only for an allow entry; 0 for any other kind.
    pub(crate) mask: u32,
    pub(crate) principal: Principal,
}

/// Why a file is not private to this user.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NotPrivate {
    /// Owned by neither this user nor Administrators.
    Owner,
    /// No access list at all: everyone has full access.
    NoAccessList,
    /// Another principal may read, change or re-grant the file.
    Shared,
    /// An entry this check does not interpret.
    UnknownEntry,
}

// Values from windows-sys 0.61.2; a Windows test holds them equal to its constants.
pub(crate) const ACCESS_ALLOWED: u8 = 0;
/// Deny entries only narrow access, so ignoring them can only refuse a file
/// that is in fact private: plain, object, callback and callback-object.
pub(crate) const ACCESS_DENIED_KINDS: [u8; 4] = [1, 6, 10, 12];
pub(crate) const INHERIT_ONLY: u8 = 0x08;
/// The rights another principal may hold: SYNCHRONIZE, READ_CONTROL,
/// FILE_READ_ATTRIBUTES and FILE_READ_EA. Every other bit, known or not, refuses.
pub(crate) const HARMLESS_RIGHTS: u32 = 0x0010_0000 | 0x0002_0000 | 0x80 | 0x08;

/// Admit a file owned by this user or by Administrators whose access list lets
/// no one but this user, SYSTEM and Administrators read, change or re-grant it.
/// SYSTEM and Administrators are admitted because no access list keeps them
/// out: an administrator can take ownership or read through the backup
/// privilege, and every file under a user's profile names both by default.
pub(crate) fn admit(owner: Principal, access_list: Option<&[Ace]>) -> Result<(), NotPrivate> {
    if !matches!(owner, Principal::ThisUser | Principal::Administrators) {
        return Err(NotPrivate::Owner);
    }
    let entries = access_list.ok_or(NotPrivate::NoAccessList)?;
    for entry in entries {
        // An inherit-only entry applies to what a folder creates, not to this file.
        if entry.flags & INHERIT_ONLY != 0 {
            continue;
        }
        if entry.kind == ACCESS_ALLOWED {
            if entry.principal == Principal::Other && entry.mask & !HARMLESS_RIGHTS != 0 {
                return Err(NotPrivate::Shared);
            }
        } else if !ACCESS_DENIED_KINDS.contains(&entry.kind) {
            return Err(NotPrivate::UnknownEntry);
        }
    }
    Ok(())
}

#[cfg(windows)]
pub(crate) use windows::private_to_this_user;

#[cfg(windows)]
mod windows {
    use super::{admit, Ace, NotPrivate, Principal};
    use std::ffi::c_void;
    use std::fs::File;
    use std::io;
    use std::os::windows::io::AsRawHandle;
    use std::ptr::null_mut;
    use windows_sys::Win32::Foundation::{CloseHandle, LocalFree, ERROR_SUCCESS, HANDLE};
    use windows_sys::Win32::Security::Authorization::{GetSecurityInfo, SE_FILE_OBJECT};
    use windows_sys::Win32::Security::{
        AclSizeInformation, EqualSid, GetAce, GetAclInformation, GetLengthSid, GetTokenInformation,
        IsValidSid, IsWellKnownSid, TokenUser, WinBuiltinAdministratorsSid,
        WinCreatorOwnerRightsSid, WinLocalSystemSid, ACCESS_ALLOWED_ACE, ACE_HEADER, ACL,
        ACL_SIZE_INFORMATION, DACL_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION,
        PSECURITY_DESCRIPTOR, PSID, TOKEN_QUERY, TOKEN_USER,
    };
    use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    /// A security descriptor `GetSecurityInfo` allocated, freed once.
    pub(super) struct Descriptor(pub(super) PSECURITY_DESCRIPTOR);

    impl Drop for Descriptor {
        fn drop(&mut self) {
            // SAFETY: the pointer came from GetSecurityInfo, which allocates
            // with LocalAlloc, and nothing else frees it.
            unsafe { LocalFree(self.0) };
        }
    }

    struct Token(HANDLE);

    impl Drop for Token {
        fn drop(&mut self) {
            // SAFETY: the handle came from OpenProcessToken and is closed once.
            unsafe { CloseHandle(self.0) };
        }
    }

    /// The `TOKEN_USER` of this process, in a buffer aligned for it; its SID
    /// points into the buffer.
    struct UserSid(Vec<u64>);

    impl UserSid {
        fn current() -> io::Result<Self> {
            let mut token: HANDLE = null_mut();
            // SAFETY: GetCurrentProcess returns a pseudo-handle; token is writable.
            if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
                return Err(io::Error::last_os_error());
            }
            let token = Token(token);
            let mut needed = 0u32;
            // SAFETY: a size query: no buffer, length 0, needed is writable.
            unsafe { GetTokenInformation(token.0, TokenUser, null_mut(), 0, &mut needed) };
            if (needed as usize) < size_of::<TOKEN_USER>() {
                return Err(io::Error::last_os_error());
            }
            let mut buffer = vec![0u64; (needed as usize).div_ceil(size_of::<u64>())];
            // SAFETY: the buffer holds at least `needed` bytes, aligned for TOKEN_USER.
            if unsafe {
                GetTokenInformation(
                    token.0,
                    TokenUser,
                    buffer.as_mut_ptr().cast::<c_void>(),
                    needed,
                    &mut needed,
                )
            } == 0
            {
                return Err(io::Error::last_os_error());
            }
            Ok(Self(buffer))
        }

        fn sid(&self) -> PSID {
            // SAFETY: the buffer holds a TOKEN_USER that GetTokenInformation wrote.
            unsafe { (*self.0.as_ptr().cast::<TOKEN_USER>()).User.Sid }
        }
    }

    fn principal(sid: PSID, user: PSID) -> Principal {
        // SAFETY: both are valid SIDs: the caller checked `sid` with IsValidSid,
        // and `user` is the token's own.
        unsafe {
            if EqualSid(sid, user) != 0 {
                Principal::ThisUser
            } else if IsWellKnownSid(sid, WinLocalSystemSid) != 0 {
                Principal::System
            } else if IsWellKnownSid(sid, WinBuiltinAdministratorsSid) != 0 {
                Principal::Administrators
            } else if IsWellKnownSid(sid, WinCreatorOwnerRightsSid) != 0 {
                Principal::OwnerRights
            } else {
                Principal::Other
            }
        }
    }

    /// Where an allow entry's SID begins: after its header and its mask.
    const SID_START: usize = size_of::<ACE_HEADER>() + size_of::<u32>();

    fn malformed() -> io::Error {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "security descriptor not readable",
        )
    }

    pub(super) fn read_entries(list: *mut ACL, user: PSID) -> io::Result<Vec<Ace>> {
        let mut size = ACL_SIZE_INFORMATION {
            AceCount: 0,
            AclBytesInUse: 0,
            AclBytesFree: 0,
        };
        // SAFETY: list is the descriptor's live DACL; size is writable for its length.
        if unsafe {
            GetAclInformation(
                list,
                (&raw mut size).cast::<c_void>(),
                size_of::<ACL_SIZE_INFORMATION>() as u32,
                AclSizeInformation,
            )
        } == 0
        {
            return Err(io::Error::last_os_error());
        }
        (0..size.AceCount)
            .map(|index| {
                let mut entry: *mut c_void = null_mut();
                // SAFETY: index is below the list's own count; entry is writable.
                if unsafe { GetAce(list, index, &mut entry) } == 0 || entry.is_null() {
                    return Err(io::Error::last_os_error());
                }
                // SAFETY: every entry starts with an ACE_HEADER.
                let header = unsafe { entry.cast::<ACE_HEADER>().read_unaligned() };
                let (mask, principal) = if header.AceType == super::ACCESS_ALLOWED {
                    if usize::from(header.AceSize) < size_of::<ACCESS_ALLOWED_ACE>() {
                        return Err(malformed());
                    }
                    let allowed = entry.cast::<ACCESS_ALLOWED_ACE>();
                    // SAFETY: the header says this is an allow entry of at least that size.
                    let mask = unsafe { (&raw const (*allowed).Mask).read_unaligned() };
                    // SAFETY: the SID starts at SidStart, inside the entry.
                    let sid: PSID = unsafe { (&raw mut (*allowed).SidStart).cast::<c_void>() };
                    // SAFETY: sid points into the live entry, which holds at least the
                    // SID's first four bytes; these two calls read only those.
                    if unsafe { IsValidSid(sid) } == 0
                        || SID_START + unsafe { GetLengthSid(sid) } as usize
                            > usize::from(header.AceSize)
                    {
                        return Err(malformed());
                    }
                    (mask, principal(sid, user))
                } else {
                    (0, Principal::Other)
                };
                Ok(Ace {
                    kind: header.AceType,
                    flags: header.AceFlags,
                    mask,
                    principal,
                })
            })
            .collect()
    }

    /// The owner and access list of the file behind `file`'s own handle, so no
    /// other file can be swapped in between the check and the read.
    pub(super) fn descriptor(file: &File) -> io::Result<(Descriptor, PSID, Option<*mut ACL>)> {
        let mut owner: PSID = null_mut();
        let mut list: *mut ACL = null_mut();
        let mut descriptor: PSECURITY_DESCRIPTOR = null_mut();
        // SAFETY: the handle is live; every out pointer is writable.
        let status = unsafe {
            GetSecurityInfo(
                file.as_raw_handle(),
                SE_FILE_OBJECT,
                OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
                &mut owner,
                null_mut(),
                &mut list,
                null_mut(),
                &mut descriptor,
            )
        };
        if status != ERROR_SUCCESS {
            return Err(io::Error::from_raw_os_error(status as i32));
        }
        let descriptor = Descriptor(descriptor);
        // SAFETY: owner points into the live descriptor when it is not null.
        if owner.is_null() || unsafe { IsValidSid(owner) } == 0 {
            return Err(malformed());
        }
        Ok((descriptor, owner, (!list.is_null()).then_some(list)))
    }

    /// Whether the open file is private to this user. An error is a file whose
    /// owner or access list could not be read, which the caller refuses too.
    pub(crate) fn private_to_this_user(file: &File) -> io::Result<Result<(), NotPrivate>> {
        let user = UserSid::current()?;
        let (_descriptor, owner, list) = descriptor(file)?;
        let entries = list
            .map(|list| read_entries(list, user.sid()))
            .transpose()?;
        Ok(admit(principal(owner, user.sid()), entries.as_deref()))
    }

    /// This process's user SID, for the test that reads a hand-built list.
    #[cfg(test)]
    pub(super) fn with_user_sid<T>(read: impl FnOnce(PSID) -> T) -> io::Result<T> {
        Ok(read(UserSid::current()?.sid()))
    }

    /// The owner's principal alone, for the test that picks an owner probe.
    #[cfg(test)]
    pub(super) fn owner_of(file: &File) -> io::Result<Principal> {
        let user = UserSid::current()?;
        let (_descriptor, owner, _) = descriptor(file)?;
        Ok(principal(owner, user.sid()))
    }
}

#[cfg(test)]
#[path = "access_tests.rs"]
mod tests;
