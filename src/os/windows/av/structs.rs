#![allow(non_snake_case, non_camel_case_types)]

use crate::os::windows::types::{DWORD, HANDLE, PCWSTR, UCHAR, ULONGLONG, USHORT};


#[repr(C)]
#[derive(Debug)]
pub struct WLDP_HOST_INFORMATION {
    pub dwRevision: DWORD,
    pub dwHostId: u32,
    pub szSource: PCWSTR,
    pub hSource: HANDLE,
}

#[repr(C)]
#[derive(Debug)]
pub struct EVENT_DESCRIPTOR {
    Id: USHORT,
    Version: UCHAR,
    Channel: UCHAR,
    Level: UCHAR,
    Opcode: UCHAR,
    Task: USHORT,
    Keyword: ULONGLONG
}

#[repr(C)]
#[derive(Debug)]
pub struct EVENT_DATA_DESCRIPTOR {
    pub Ptr: u64,
    pub Size: u32,
    pub Reserved: u32,
}
