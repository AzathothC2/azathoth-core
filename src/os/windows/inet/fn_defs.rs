#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::c_ushort;
use crate::os::windows::types::{BOOL, DWORD, HINTERNET, LPCSTR, LPDWORD, LPVOID};
use super::structs::URL_COMPONENTSA;

/// Windows InternetCrackUrlA typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetcrackurla>
pub type InternetCrackUrlA_t = unsafe extern "system" fn(
    lpszUrl: LPCSTR,
    dwUrlLength: DWORD,
    dwFlags: DWORD,
    lpUrlComponents: *mut URL_COMPONENTSA,
) -> BOOL;

/// Windows InternetOpenA typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetopena>
pub type InternetOpenA_t = unsafe extern "system" fn(
    lpszAgent: LPCSTR,
    dwAccessType: DWORD,
    lpszProxy: LPCSTR,
    lpszProxyBypass: LPCSTR,
    dwFlags: DWORD,
) -> HINTERNET;

/// Windows InternetConnectA typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetconnecta>
pub type InternetConnectA_t = unsafe extern "system" fn(
    hInternet: HINTERNET,
    lpszServerName: LPCSTR,
    nServerPort: c_ushort,
    lpszUserName: LPCSTR,
    lpszPassword: LPCSTR,
    dwService: DWORD,
    dwFlags: DWORD,
    dwContext: DWORD,
) -> HINTERNET;

/// Windows InternetSetOptionA typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetsetoptiona>
pub type InternetSetOptionA_t = unsafe extern "system" fn(
    hInternet: HINTERNET,
    dwOption: DWORD,
    lpBuffer: LPVOID,
    dwBufferLength: DWORD,
) -> BOOL;

/// Windows InternetReadFile typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetreadfile>
pub type InternetReadFile_t = unsafe extern "system" fn(
    hFile: HINTERNET,
    lpBuffer: LPVOID,
    dwNumberOfBytesToRead: DWORD,
    lpdwNumberOfBytesRead: LPDWORD,
) -> BOOL;

/// Windows InternetCloseHandle typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetclosehandle>
pub type InternetCloseHandle_t = unsafe extern "system" fn(hInternet: HINTERNET) -> BOOL;

/// Windows InternetQueryDataAvailable typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetquerydataavailable>
pub type InternetQueryDataAvailable_t = unsafe extern "system" fn(
    hFile: HINTERNET,
    lpdwNumberOfBytesAvailable: LPDWORD,
    dwFlags: DWORD,
    dwContext: DWORD,
) -> BOOL;

/// Windows HttpOpenRequestA typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-httpopenrequesta>
pub type HttpOpenRequestA_t = unsafe extern "system" fn(
    hConnect: HINTERNET,
    lpszVerb: LPCSTR,
    lpszObjectName: LPCSTR,
    lpszVersion: LPCSTR,
    lpszReferrer: LPCSTR,
    lplpszAcceptTypes: *const LPCSTR,
    dwFlags: DWORD,
    dwContext: DWORD,
) -> HINTERNET;

/// Windows HttpSendRequestA typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-httpsendrequesta>
pub type HttpSendRequestA_t = unsafe extern "system" fn(
    hRequest: HINTERNET,
    lpszHeaders: LPCSTR,
    dwHeadersLength: DWORD,
    lpOptional: LPVOID,
    dwOptionalLength: DWORD,
) -> BOOL;

/// Windows HttpQueryInfoA typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-httpqueryinfoa>
pub type HttpQueryInfoA_t = unsafe extern "system" fn(
    hRequest: HINTERNET,
    dwInfoLevel: DWORD,
    lpBuffer: LPVOID,
    lpdwBufferLength: LPDWORD,
    lpdwIndex: LPDWORD,
) -> BOOL;
