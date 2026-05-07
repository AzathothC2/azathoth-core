#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::c_int;
use crate::os::windows::types::{DWORD, LPSTR, USHORT};

/// Components of a parsed URL string, used with `InternetCrackUrlA`.
#[repr(C)]
#[derive(Debug)]
pub struct URL_COMPONENTSA {
    /// Size of this structure, in bytes.
    pub dwStructSize: DWORD,
    /// Pointer to a buffer that receives the URL scheme (e.g., "http").
    pub lpszScheme: LPSTR,
    /// Length of the scheme string, in characters.
    pub dwSchemeLength: DWORD,
    /// Scheme type value as determined by the API.
    pub nScheme: c_int,
    /// Pointer to a buffer that receives the host name.
    pub lpszHostName: LPSTR,
    /// Length of the host name string, in characters.
    pub dwHostNameLength: DWORD,
    /// Port number extracted from the URL.
    pub nPort: USHORT,
    /// Pointer to a buffer that receives the username, if present.
    pub lpszUserName: LPSTR,
    /// Length of the username string, in characters.
    pub dwUserNameLength: DWORD,
    /// Pointer to a buffer that receives the password, if present.
    pub lpszPassword: LPSTR,
    /// Length of the password string, in characters.
    pub dwPasswordLength: DWORD,
    /// Pointer to a buffer that receives the URL path.
    pub lpszUrlPath: LPSTR,
    /// Length of the URL path string, in characters.
    pub dwUrlPathLength: DWORD,
    /// Pointer to a buffer that receives extra info or query parameters.
    pub lpszExtraInfo: LPSTR,
    /// Length of the extra info string, in characters.
    pub dwExtraInfoLength: DWORD,
}
