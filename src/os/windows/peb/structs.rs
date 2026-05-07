#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::c_void;
use crate::os::windows::types::PPS_POST_PROCESS_INIT_ROUTINE;

/// Represents a 128-bit globally unique identifier (GUID).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "PascalCase"))]
pub struct Guid {
    /// First 32 bits of the GUID.
    pub data1: u32,
    /// Next 16 bits of the GUID.
    pub data2: u16,
    /// Next 16 bits of the GUID.
    pub data3: u16,
    /// Final 64 bits of the GUID as bytes.
    pub data4: [u8; 8],
}

impl Guid {
    /// Creates a new GUID from its four components.
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Guid { data1, data2, data3, data4 }
    }

    /// Converts a 128-bit integer to a GUID (big-endian order).
    pub const fn from_u128(data: u128) -> Self {
        let bytes = data.to_be_bytes();
        Guid {
            data1: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            data2: u16::from_be_bytes([bytes[4], bytes[5]]),
            data3: u16::from_be_bytes([bytes[6], bytes[7]]),
            data4: [bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]],
        }
    }
}

/// Thread Environment Block.
#[repr(C)]
pub struct TEB {
    Reserved1: [*mut c_void; 12],
    /// Pointer to the Process Environment Block (PEB) of the current process.
    pub ProcessEnvironmentBlock: *mut PEB,
    Reserved2: [*mut c_void; 399],
    Reserved3: [u8; 1952],
    pub TlsSlots: [*mut c_void; 64],
    Reserved4: [u8; 8],
    Reserved5: [*mut c_void; 26],
    ReservedForOle: *mut c_void,
    Reserved6: [*mut c_void; 4],
    pub TlsExpansionSlots: *mut c_void,
}

/// Minimal Process Environment Block.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PEB {
    Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    Reserved2: [u8; 1],
    Reserved3: [*mut c_void; 2],
    /// Pointer to the loader data for this process.
    pub Ldr: *mut PEB_LDR_DATA,
    /// User process parameters.
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    Reserved4: [*mut c_void; 3],
    pub AtlThunkSListPtr: *mut c_void,
    Reserved5: *mut c_void,
    Reserved6: u32,
    Reserved7: *mut c_void,
    Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    Reserved9: [*mut c_void; 45],
    Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    Reserved11: [u8; 128],
    Reserved12: [*mut c_void; 1],
    pub SessionId: u32,
}

/// User process parameters.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    Reserved1: [u8; 16],
    Reserved2: [*mut c_void; 10],
    /// Full path to the process's image file.
    pub ImagePathName: UNICODE_STRING,
    /// Command line arguments of the process.
    pub CommandLine: UNICODE_STRING,
    /// Pointer to the process's environment variables block.
    pub Environment: *mut u16,
}

impl Default for RTL_USER_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/// Counted, UTF-16 encoded Unicode string buffer.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNICODE_STRING {
    /// Length in bytes of the string (not including null terminator).
    pub Length: u16,
    /// Maximum size in bytes allocated for `Buffer`.
    pub MaximumLength: u16,
    /// Pointer to the wide-character string buffer.
    pub Buffer: *mut u16,
}

impl UNICODE_STRING {
    pub fn from_wide(wide: &[u16]) -> Self {
        Self {
            Length: (wide.len() * 2) as u16,
            MaximumLength: (wide.len() * 2) as u16,
            Buffer: wide.as_ptr() as *mut u16,
        }
    }
}

/// Doubly-linked list entry used internally by Windows.
#[repr(C)]
#[derive(Debug)]
pub struct LIST_ENTRY {
    /// Forward link to the next entry.
    pub flink: *mut LIST_ENTRY,
    /// Backward link to the previous entry.
    pub blink: *mut LIST_ENTRY,
}

/// Entry in the process loader's module list.
///
/// Each loaded module (DLL or EXE) is represented by an `LDR_DATA_TABLE_ENTRY`
/// in the loader's linked lists.
#[repr(C)]
#[derive(Debug)]
pub struct LDR_DATA_TABLE_ENTRY {
    pub InLoadOrderLinks: LIST_ENTRY,
    pub InMemoryOrderLinks: LIST_ENTRY,
    pub InInitializationOrderLinks: LIST_ENTRY,
    /// Base address where the module is loaded in memory.
    pub DllBase: *mut c_void,
    /// Entry point address of the module.
    pub EntryPoint: *mut c_void,
    /// Size in bytes of the loaded module image.
    pub SizeOfImage: u32,
    pub FullDllName: UNICODE_STRING,
    /// Base name of the loaded module (without path).
    pub BaseDllName: UNICODE_STRING,
}

/// Loader data for a process; referenced by [`PEB`].
///
/// Maintains linked lists of loaded modules in load, memory, and initialization order.
#[repr(C)]
#[derive(Debug)]
pub struct PEB_LDR_DATA {
    pub Length: u32,
    pub Initialized: u8,
    pub SsHandle: *mut c_void,
    /// Head of the load-order module list.
    pub InLoadOrderModuleList: LIST_ENTRY,
    pub InMemoryOrderModuleList: LIST_ENTRY,
    pub InInitializationOrderModuleList: LIST_ENTRY,
}
