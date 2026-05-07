#![allow(non_snake_case, non_camel_case_types)]

use core::ptr::null_mut;
use crate::os::windows::types::HANDLE;
use crate::os::windows::peb::structs::UNICODE_STRING;

/// Windows `OBJECT_ATTRIBUTES` - passed to NT APIs to name objects and set security.
#[repr(C)]
pub struct ObjectAttributes {
    pub length: u32,
    pub root_directory: HANDLE,
    pub object_name: *mut UNICODE_STRING,
    pub attributes: u32,
    pub security_descriptor: HANDLE,
    pub security_quality_of_service: HANDLE,
}

impl ObjectAttributes {
    pub fn empty() -> Self {
        Self {
            length: size_of::<Self>() as u32,
            root_directory: null_mut(),
            object_name: null_mut(),
            attributes: 0,
            security_descriptor: null_mut(),
            security_quality_of_service: null_mut(),
        }
    }

    pub fn with_attributes(attributes: u32, object_name: *mut UNICODE_STRING) -> Self {
        Self {
            length: size_of::<Self>() as u32,
            root_directory: null_mut(),
            object_name,
            attributes,
            security_descriptor: null_mut(),
            security_quality_of_service: null_mut(),
        }
    }
}

/// Windows `IO_STATUS_BLOCK` - returned by NT I/O functions.
#[repr(C)]
pub struct IoStatusBlock {
    pub status: i32,
    pub information: usize,
}

impl IoStatusBlock {
    pub fn empty() -> Self {
        Self { status: 0, information: 0 }
    }
}

/// Windows `LARGE_INTEGER` - 64-bit signed integer as two 32-bit halves.
#[repr(C)]
pub struct LargeInteger {
    pub low: u32,
    pub high: i32,
}
