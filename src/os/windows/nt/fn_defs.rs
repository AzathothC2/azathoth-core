#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::c_void;
use crate::os::windows::types::{DWORD, HANDLE, LPDWORD, LPVOID};
use super::structs::{ClientId, IoStatusBlock, LargeInteger, ObjectAttributes};

/// NT NtClose typedef: Closes an object handle.
pub type NtClose_t = unsafe extern "system" fn(handle: *mut c_void) -> i32;

/// NT NtOpenFile typedef: Opens an existing file.
pub type NtOpenFile_t = unsafe extern "system" fn(
    file_handle: *mut *mut c_void,
    desired_access: u32,
    object_attributes: *mut ObjectAttributes,
    io_status_block: *mut IoStatusBlock,
    share_access: u32,
    open_options: u32,
) -> i32;

/// NT NtCreateFile typedef: Creates or opens a file or I/O device.
pub type NtCreateFile_t = unsafe extern "system" fn(
    file_handle: *mut *mut c_void,
    desired_access: u32,
    object_attributes: *mut ObjectAttributes,
    io_status_block: *mut IoStatusBlock,
    allocation_size: *mut LargeInteger,
    file_attributes: u32,
    share_access: u32,
    create_disposition: u32,
    create_options: u32,
    ea_buffer: *mut c_void,
    ea_length: u32,
) -> i32;

/// NT NtCreateNamedPipeFile typedef: Creates a named pipe and returns a handle to it.
pub type NtCreateNamedPipeFile_t = unsafe extern "system" fn(
    file_handle: *mut *mut c_void,
    desired_access: u32,
    object_attributes: *mut ObjectAttributes,
    io_status_block: *mut IoStatusBlock,
    share_access: u32,
    create_disposition: u32,
    create_options: u32,
    named_pipe_type: u32,
    read_mode: u32,
    completion_mode: u32,
    max_instances: u32,
    in_buffer_size: u32,
    out_buffer_size: u32,
    default_timeout: *mut LargeInteger,
) -> i32;

/// NT NtReadFile typedef: Reads data from a file or I/O device.
pub type NtReadFile_t = unsafe extern "system" fn(
    file_handle: *mut c_void,
    event: *mut c_void,
    apc_routine: *mut c_void,
    apc_context: *mut c_void,
    io_status_block: *mut IoStatusBlock,
    buffer: *mut u8,
    length: u32,
    byte_offset: *mut LargeInteger,
    key: *mut u32,
) -> i32;

/// Windows ReadFile typedef:
/// <https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-readfile>
pub type ReadFile_t = unsafe extern "system" fn(
    hFile: HANDLE,
    lpBuffer: LPVOID,
    nNumberOfBytesToRead: DWORD,
    lpNumberOfBytesRead: LPDWORD,
    lpOverlapped: *mut c_void,
) -> i32;


pub type NtAllocateVirtualMemory_t = unsafe extern "system" fn(
    process: isize,
    base: *mut *mut u8,
    zero_bits: usize,
    size: *mut usize,
    alloc_type: u32,
    protect: u32,
) -> i32;

pub type NtProtectVirtualMemory_t = unsafe extern "system" fn(
    process: isize,
    base: *mut *mut u8,
    size: *mut usize,
    new_protect: u32,
    old_protect: *mut u32,
) -> i32;

pub type NtFreeVirtualMemory_t = unsafe extern "system" fn(
    process: isize,
    base: *mut *mut u8,
    size: *mut usize,
    free_type: u32,
) -> i32;

pub type NtReadVirtualMemory_t = unsafe extern "system" fn(
    process_handle: *mut c_void,
    base_address: *const c_void,
    buffer: *mut c_void,
    buffer_size: usize,
    number_of_bytes_read: *mut usize,
) -> i32;

pub type NtQueryInformationProcess_t = unsafe extern "system" fn(
    process_handle: *mut c_void,
    process_information_class: u32,
    process_information: *const c_void,
    process_information_length: u32,
    return_length: *mut u32,
) -> i32;
pub type NtOpenProcess_t = unsafe extern "system" fn(
    process_handle: *mut *mut c_void,
    desired_access: u32,
    object_attributes: *mut ObjectAttributes,
    client_id: *mut ClientId,
) -> i32;

pub type NtWriteVirtualMemory_t = unsafe extern "system" fn(
    process_handle: *mut c_void,
    base_address: *const c_void,
    buffer: *const c_void,
    buffer_size: usize,
    number_of_bytes_written: *mut usize,
) -> i32;