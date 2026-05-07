#![allow(non_snake_case, non_camel_case_types)]

use crate::os::windows::types::{BOOL, FARPROC, HMODULE, PCSTR};

/// Windows LoadLibraryA typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibrarya>
pub type LoadLibraryA_t = unsafe extern "system" fn(lib: PCSTR) -> HMODULE;

/// Windows GetProcAddress typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress>
pub type GetProcAddress_t = unsafe extern "system" fn(hmod: HMODULE, procname: PCSTR) -> FARPROC;

/// Windows GetModuleHandleA typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlea>
pub type GetModuleHandleA_t = unsafe extern "system" fn(lpModuleName: PCSTR) -> HMODULE;

/// Windows RtlDecompressBuffer typedef:
/// <https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-rtldecompressbuffer>
pub type RtlDecompressBuffer_t = unsafe extern "system" fn(
    CompressionFormat: u16,
    UncompressedBuffer: *mut u8,
    UncompressedBufferSize: usize,
    CompressedBuffer: *const u8,
    CompressedBufferSize: usize,
    FinalUncompressedSize: *mut usize,
) -> u32;

/// Windows DLL entry point typedef.
pub type DllMain_t = unsafe extern "system" fn(module: HMODULE, reason: u32, reserved: *mut core::ffi::c_void) -> BOOL;

/// Equivalent of `int main(int argc, char* argv[])`.
pub type IntMainArgs = unsafe extern "C" fn(argc: i32, argv: *mut *mut u8) -> i32;

/// Equivalent of `int main()`.
pub type IntMainVoid = unsafe extern "system" fn() -> i32;
