#![allow(non_camel_case_types)]

use core::ffi::c_void;
use super::structs::Guid;

/// A generic handle to an object (e.g., file, process, thread, or resource).
/// Typically returned by many WinAPI functions and used as an opaque pointer.
pub type HANDLE = *mut c_void;

/// A handle to a loaded module (DLL or EXE).
/// Returned by functions like `LoadLibrary` or `GetModuleHandle`.
pub type HMODULE = *mut c_void;

/// A handle to an application instance (historically used in Win16 applications).
/// In modern Win32, this is the same as `HMODULE`.
pub type HINSTANCE = *mut c_void;

/// A pointer to an unspecified type of data (constant).
/// Often used as a generic pointer parameter in APIs.
pub type PVOID = *const c_void;

/// A pointer to a post-process initialization routine.
/// This is an optional function executed during process startup.
pub type PPS_POST_PROCESS_INIT_ROUTINE = Option<unsafe extern "system" fn()>;

/// A pointer to a TLS (Thread Local Storage) callback function.
/// Signature: `fn(DllHandle: PVOID, Reason: DWORD, Reserved: PVOID)`.
pub type PIMAGE_TLS_CALLBACK = unsafe extern "system" fn(PVOID, DWORD, PVOID);

/// A pointer to a constant ANSI string (null-terminated).
/// Used for passing immutable C-style strings.
pub type PCSTR = *const u8;

/// A pointer to a constant wide string.
/// Used for passing immutable C-style strings.
pub type PCWSTR = *const u16;
/// A generic function pointer type returned by `GetProcAddress`.
/// Represents an exported function's entry point.
pub type FARPROC = Option<unsafe extern "system" fn()>;

/// A handle to an Internet session (e.g., returned by `InternetOpen`).
pub type HINTERNET = *mut c_void;

/// A 32-bit boolean value used by the WinAPI.
/// Nonzero values represent `TRUE`, zero represents `FALSE`.
pub type BOOL = i32;

/// A pointer to any mutable data type.
/// Used for passing output buffers or generic memory addresses.
pub type LPVOID = *mut c_void;

/// A 32-bit unsigned integer.
/// Widely used in WinAPI for sizes, flags, and identifiers.
pub type DWORD = u32;

/// A pointer to a constant ANSI string (null-terminated).
/// Used for passing read-only C-style strings.
pub type LPCSTR = *const u8;

/// A pointer to a `DWORD` value.
/// Used for returning 32-bit integers via function parameters.
pub type LPDWORD = *mut DWORD;

/// A pointer to a mutable ANSI string buffer.
/// Used when APIs need to modify a string.
pub type LPSTR = *mut u8;

/// A pointer to a `DWORD` used for storing 32/64-bit values in certain APIs.
/// Typically used for passing a value by reference.
pub type DWORD_PTR = *mut DWORD;

/// A 64-bit unsigned integer.
/// Used for large numerical values (file offsets, sizes, etc.).
pub type ULONGLONG = u64;

/// A signed 32-bit value representing NTSTATUS codes.
/// Used by native NT system calls to indicate success or failure.
pub type NTSTATUS = i32;

/// A 32-bit unsigned long integer.
/// Often used for counters, sizes, or identifiers.
pub type ULONG = u32;

/// A boolean value represented as an 8-bit unsigned integer.
/// Used in COM and some legacy APIs where `TRUE` = 1, `FALSE` = 0.
pub type BOOLEAN = u8;

/// A pointer to a `GUID` structure representing a CLSID (Class Identifier).
/// Used when passing a reference to a COM class ID.
pub type REFCLSID = *const Guid;

/// A pointer to a `GUID` structure representing an IID (Interface Identifier).
/// Used when passing a reference to a COM interface ID.
pub type REFIID = *const Guid;

/// A 32-bit signed value representing an HRESULT.
/// Used extensively in COM to indicate success (`S_OK`) or error codes.
pub type HRESULT = i32;

/// A pointer to a Unicode string (allocated as a COM BSTR).
/// Used in COM automation to represent strings.
pub type BSTR = *mut u16;

/// A pointer to a mutable wide-character (UTF-16) string buffer.
/// Typically used for writable Unicode strings.
pub type LPWSTR = *mut u16;

/// A pointer to a constant wide-character (UTF-16) string.
/// Used for immutable Unicode strings.
pub type LPCWSTR = *const u16;

/// A signed 32-bit integer.
/// Often used for counts, status codes, or general-purpose integers.
pub type LONG = i32;

/// A 16-bit unsigned integer representing a variant type (COM).
/// Specifies the type of data stored in a VARIANT structure.
pub type VARTYPE = u16;

/// A 16-bit unsigned integer.
/// Used for various identifiers, sizes, or bitfields in WinAPI.
pub type WORD = u16;


pub type USHORT = u16;
pub type UCHAR = u8;