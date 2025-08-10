#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_void};
use crate::platform::linux::types::{CURLCode, CURLOption, CURLPtr};

/// Type alias for `dlopen`-style function used to load shared libraries.
pub type DlOpen_t = unsafe extern "C" fn(*const u8, i32) -> *mut c_void;
/// Type alias for `dlsym`-style function used to resolve symbols.
pub type DlSym_t = unsafe extern "C" fn(*mut c_void, *const u8) -> *mut c_void;
/// Type alias for `dlerror`-style function used to retrieve dynamic linker errors.
pub type DlError_t = unsafe extern "C" fn() -> *const c_char;

/// Creates an anonymous in-memory file descriptor.
pub type MemFdCreate_t = unsafe extern "C" fn(*const u8, flags: u32) -> i32;
/// Creates a new process by duplicating the current one.
pub type Fork_t = unsafe extern "C" fn() -> i32;
/// Executes a binary with a given argument and environment list.
pub type Execve_t = unsafe extern "C" fn(pathname: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> i32;
/// Retrieves a pointer to the current thread's `errno` value.
pub type ErrnoLocation_t = unsafe extern "C" fn() -> *mut i32;
/// Writes raw bytes to a file descriptor.
pub type Write_t = unsafe extern "C" fn(fd: i32, buf: *const c_void, count: usize) -> isize;


/// Type alias for the `curl_global_init` function.
///
/// Initializes the global cURL environment.
///
/// # Safety
/// Must be called before any other cURL functions are used.
pub type CurlGlobalInit_t = unsafe extern "C" fn(flags: u64);

/// Type alias for the `curl_easy_init` function.
///
/// Creates and returns a new easy handle for cURL requests.
pub type CurlEasyInit_t = unsafe extern "C" fn() -> CURLPtr;

/// Type alias for the variadic `curl_easy_setopt` function.
///
/// Sets options for a cURL easy handle. Variadic arguments depend on the option.
pub type CurlEasySetOpt_t = unsafe extern "C" fn(handle: CURLPtr, option: u32, ...) -> CURLOption;

/// Type alias for the `curl_easy_perform` function.
///
/// Executes a previously configured cURL request.
pub type CurlEasyPerform_t = unsafe extern "C" fn(handle: CURLPtr) -> CURLCode;

/// Type alias for the `curl_easy_cleanup` function.
///
/// Frees resources associated with a cURL easy handle.
pub type CurlEasyCleanup_t = unsafe extern "C" fn(handle: CURLPtr);

/// Type alias for the `curl_global_cleanup` function.
///
/// Cleans up global cURL state previously initialized with `curl_global_init`.
pub type CurlGlobalCleanup_t = unsafe extern "C" fn();

/// Type alias for `curl_easy_setopt` specialized for string parameters.
///
/// Sets an option that accepts a `*const c_char` as value.
pub type CurlEasySetOptStr_t = unsafe extern "C" fn(handle: CURLPtr, option: u32, value: *const c_char) -> CURLOption;

/// Type alias for `curl_easy_setopt` specialized for pointer parameters.
///
/// Sets an option that accepts a generic pointer value.
pub type CurlEasySetOptPtr_t = unsafe extern "C" fn(handle: CURLPtr, option: u32, value: *mut c_void) -> CURLOption;

/// Type alias for `curl_easy_setopt` specialized for callback functions.
///
/// Sets an option that accepts a function pointer, typically for write callbacks.
pub type CurlEasySetOptFn_t = unsafe extern "C" fn(handle: CURLPtr, option: u32, value: CurlWriteFn_t) -> CURLOption;

/// Type alias for the write callback function used with cURL.
///
/// Called when cURL has received data.
///
/// - `ptr`: Pointer to the received data buffer.
/// - `size`: Size of each element (usually 1).
/// - `nmemb`: Number of elements received.
/// - `userdata`: User-defined pointer passed to the callback.
///
/// Returns the number of bytes successfully handled.
pub type CurlWriteFn_t = unsafe extern "C" fn(ptr: *mut c_char, size: usize, nmemb: usize, userdata: *mut c_void) -> usize;
