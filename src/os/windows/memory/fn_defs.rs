#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::c_void;
use crate::os::windows::types::{BOOL, DWORD, HANDLE, LPVOID};

/// Windows VirtualAlloc typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc>
pub type VirtualAlloc_t = unsafe extern "system" fn(
    addr: *const c_void,
    size: usize,
    alloc_type: u32,
    protect: u32,
) -> HANDLE;

/// Windows VirtualProtect typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualprotect>
pub type VirtualProtect_t = unsafe extern "system" fn(
    addr: *const c_void,
    size: usize,
    new_protect: u32,
    old_protect: *mut u32,
) -> BOOL;

/// Windows VirtualFree typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualfree>
pub type VirtualFree_t = unsafe extern "system" fn(addr: *mut c_void, size: usize, free_type: u32) -> BOOL;

/// Windows VirtualLock typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtuallock>
pub type VirtualLock_t = unsafe extern "system" fn(lpaddress: LPVOID, size: usize) -> BOOL;

/// Windows FlushInstructionCache typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushinstructioncache>
pub type FlushInstructionCache_t = unsafe extern "system" fn(
    hproc: HANDLE,
    base_addr: *const c_void,
    size: usize,
) -> BOOL;

/// Windows GetProcessHeap typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap>
pub type GetProcessHeap_t = unsafe extern "system" fn() -> HANDLE;

/// Windows HeapAlloc typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>
pub type HeapAlloc_t = unsafe extern "system" fn(h_heap: HANDLE, flags: DWORD, size: usize) -> LPVOID;

/// Windows HeapReAlloc typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc>
pub type HeapReAlloc_t = unsafe extern "system" fn(h_heap: HANDLE, flags: DWORD, mem: LPVOID, size: usize) -> LPVOID;

/// Windows HeapFree typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree>
pub type HeapFree_t = unsafe extern "system" fn(h_heap: HANDLE, flags: DWORD, mem: LPVOID) -> BOOL;

/// Windows HeapValidate typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapvalidate>
pub type HeapValidate_t = unsafe extern "system" fn(hHeap: HANDLE, dwFlags: u32, lpMem: *mut c_void) -> i32;

/// Windows RtlCompareMemory typedef:
/// <https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/nf-wdm-rtlcomparememory>
pub type RtlCompareMemory_t = unsafe extern "system" fn(
    source1: *const c_void,
    source2: *const c_void,
    length: usize,
) -> u64;
