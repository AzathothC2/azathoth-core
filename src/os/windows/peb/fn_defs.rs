#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::c_void;
use crate::os::windows::types::NTSTATUS;
use super::structs::LDR_DATA_TABLE_ENTRY;

/// Windows LdrpReleaseTlsEntry typedef (undocumented).
pub type LdrpReleaseTlsEntry_t = unsafe extern "system" fn(entry: *mut LDR_DATA_TABLE_ENTRY, unk: *mut c_void) -> NTSTATUS;

/// Windows LdrpHandleTlsData typedef (undocumented).
pub type LdrpHandleTlsData_t = unsafe extern "system" fn(entry: *mut LDR_DATA_TABLE_ENTRY);
