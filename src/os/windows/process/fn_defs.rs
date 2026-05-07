#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::c_void;
use crate::os::windows::types::{BOOL, DWORD, HANDLE, LPCWSTR, LPDWORD, LPVOID, LPWSTR};
use super::structs::{ProcessInformation, StartupInfoW};

/// Windows CreateThread typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread>
pub type CreateThread_t = unsafe extern "system" fn(
    lpThreadSecurityAttributes: *mut c_void,
    dwStackSize: usize,
    lpStartAddress: unsafe extern "system" fn(*mut c_void) -> u32,
    lpParameter: *mut c_void,
    dwCreationflags: u32,
    lpThreadId: *mut u32,
) -> HANDLE;

/// Windows ExitThread typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread>
pub type ExitThread_t = unsafe extern "system" fn(code: u32) -> !;

/// Windows ExitProcess typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitprocess>
pub type ExitProcess_t = unsafe extern "system" fn(code: u32) -> !;

/// Windows TlsAlloc typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-tlsalloc>
pub type TlsAlloc_t = unsafe extern "system" fn() -> DWORD;

/// Windows TlsSetValue typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-tlssetvalue>
pub type TlsSetValue_t = unsafe extern "system" fn(dwTlsIndex: DWORD, lpTlsValue: LPVOID) -> BOOL;

/// Windows TlsGetValue typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-tlsgetvalue>
pub type TlsGetValue_t = unsafe extern "system" fn(dwTlsIndex: DWORD) -> LPVOID;

/// Windows GetLastError typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror>
pub type GetLastError_t = unsafe extern "system" fn() -> DWORD;

/// Windows OpenProcess typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess>
pub type OpenProcess_t = unsafe extern "system" fn(
    dwDesiredAccess: u32,
    bInheritHandle: i32,
    dwProcessId: u32,
) -> HANDLE;

/// Windows InitializeProcThreadAttributeList typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-initializeprocthreadattributelist>
pub type InitializeProcThreadAttributeList_t = unsafe extern "system" fn(
    lpAttributeList: *mut u8,
    dwAttributeCount: u32,
    dwFlags: u32,
    lpSize: *mut usize,
) -> i32;

/// Windows UpdateProcThreadAttribute typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute>
pub type UpdateProcThreadAttribute_t = unsafe extern "system" fn(
    lpAttributeList: *mut u8,
    dwFlags: u32,
    Attribute: usize,
    lpValue: HANDLE,
    cbSize: usize,
    lpPreviousValue: HANDLE,
    lpReturnSize: *mut usize,
) -> i32;

/// Windows ResumeThread typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-resumethread>
pub type ResumeThread_t = unsafe extern "system" fn(hThread: HANDLE) -> u32;

/// Windows CloseHandle typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle>
pub type CloseHandle_t = unsafe extern "system" fn(hHandle: HANDLE) -> i32;

/// Windows WaitForSingleObject typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-waitforsingleobject>
pub type WaitForSingleObject_t = unsafe extern "system" fn(hHandle: HANDLE, dwMilliseconds: u32) -> u32;

/// Windows GetExitCodeProcess typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getexitcodeprocess>
pub type GetExitCodeProcess_t = unsafe extern "system" fn(hHandle: HANDLE, lpExitCode: *mut u32) -> i32;

/// Windows GetStdHandle typedef:
/// <https://learn.microsoft.com/en-us/windows/console/getstdhandle>
pub type GetStdHandle_t = unsafe extern "system" fn(nStdHandle: DWORD) -> HANDLE;

/// Windows WriteConsoleA typedef:
/// <https://learn.microsoft.com/en-us/windows/console/writeconsole>
pub type WriteConsoleA_t = unsafe extern "system" fn(
    hConsoleOutput: HANDLE,
    lpBuffer: *const c_void,
    nNumberOfCharsToWrite: DWORD,
    lpNumberOfCharsWritten: LPDWORD,
    lpReserved: LPVOID,
) -> BOOL;

/// Windows AllocConsole typedef:
/// <https://learn.microsoft.com/en-us/windows/console/allocconsole>
pub type AllocConsole_t = unsafe extern "system" fn() -> BOOL;

/// Windows GetCommandLineW typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processenv/nf-processenv-getcommandlinew>
#[allow(unused)]
pub type GetCommandLineW_t = unsafe extern "system" fn() -> LPWSTR;

/// Windows CommandLineToArgvW typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw>
#[allow(unused)]
pub type CommandLineToArgvW_t = unsafe extern "system" fn(lpCmdLine: LPCWSTR, pNumArgs: *mut i32) -> LPWSTR;

/// Windows CreateProcess typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw>
pub type CreateProcess_t = unsafe extern "system" fn(
    lpApplicationName: *const u16,
    lpCommandLine: *mut u16,
    lpProcessAttributes: HANDLE,
    lpThreadAttributes: HANDLE,
    bInheritHandles: i32,
    dwCreationFlags: u32,
    lpEnvironment: HANDLE,
    lpCurrentDirectory: *mut u16,
    lpStartupInfo: *mut StartupInfoW,
    lpProcessInformation: *mut ProcessInformation,
) -> i32;
