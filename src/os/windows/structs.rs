#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::{c_int, c_void};
use crate::os::windows::types::USHORT;

use super::types::{DWORD, LPSTR, PPS_POST_PROCESS_INIT_ROUTINE, ULONGLONG};

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
    pub const fn new(
        data1: u32,
        data2: u16,
        data3: u16,
        data4: [u8; 8],
    ) -> Self {
        Guid {
            data1,
            data2,
            data3,
            data4,
        }
    }

    /// Converts a 128-bit integer to a GUID (big-endian order).
    pub const fn from_u128(data: u128) -> Self {
        let bytes = data.to_be_bytes();

        Guid {
            data1: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            data2: u16::from_be_bytes([bytes[4], bytes[5]]),
            data3: u16::from_be_bytes([bytes[6], bytes[7]]),
            data4: [
                bytes[8], bytes[9], bytes[10], bytes[11],
                bytes[12], bytes[13], bytes[14], bytes[15],
            ],
        }
    }
}


/// Thread Environment Block
#[repr(C)]
pub struct TEB {
    Reserved1: [*mut c_void; 12],
    /// Pointer to the Process Environment Block (PEB) of the current process.
    ///
    /// This field provides access to the process-wide data structure
    /// that holds information such as loaded modules, process parameters,
    /// and environment variables.
    pub ProcessEnvironmentBlock: *mut PEB,
    Reserved2: [*mut c_void; 399],
    Reserved3: [u8; 1952],
    TlsSlots: [*mut c_void; 64],
    Reserved4: [u8; 8],
    Reserved5: [*mut c_void; 26],
    ReservedForOle: *mut c_void,
    Reserved6: [*mut c_void; 4],
    TlsExpansionSlots: *mut c_void,
}

/// Minimal Process Environment Block struct
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PEB {
    Reserved1: [u8; 2],
    BeingDebugged: u8,
    Reserved2: [u8; 1],
    Reserved3: [*mut c_void; 2],
    /// Pointer to the loader data for this process.
    ///
    /// This points to a [`PEB_LDR_DATA`] structure containing a linked list
    /// of all loaded modules for the process.
    pub Ldr: *mut PEB_LDR_DATA,
    /// User process parameters
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    Reserved4: [*mut c_void; 3],
    AtlThunkSListPtr: *mut c_void,
    Reserved5: *mut c_void,
    Reserved6: u32,
    Reserved7: *mut c_void,
    Reserved8: u32,
    AtlThunkSListPtr32: u32,
    Reserved9: [*mut c_void; 45],
    Reserved10: [u8; 96],
    PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    Reserved11: [u8; 128],
    Reserved12: [*mut c_void; 1],
    SessionId: u32,
}

/// User process parameters struct
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    Reserved1: [u8; 16],
    Reserved2: [*mut c_void; 10],
    /// Unicode string containing the full path to the process's image file.
    ///
    /// This value represents the absolute file system path of the executable
    /// that started the process.
    pub ImagePathName: UNICODE_STRING,

    /// Unicode string containing the command line arguments of the process.
    ///
    /// This value points to the full command line, excluding the executable
    /// name in some cases, as passed to the process on creation.
    pub CommandLine: UNICODE_STRING,

    /// Pointer to the process's environment variables block.
    ///
    /// The environment is a sequence of null-terminated strings
    /// (`KEY=VALUE` format) followed by an additional null terminator.
    pub Environment: *mut u16,
}
impl Default for RTL_USER_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

/// Represents a counted, UTF-16 encoded Unicode string buffer
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNICODE_STRING {
    /// Length, in bytes, of the string stored in `Buffer`.
    ///
    /// This does **not** include the terminating null character.
    pub Length: u16,

    /// Maximum size, in bytes, allocated for `Buffer`.
    ///
    /// This value typically includes space for the terminating null character.
    pub MaximumLength: u16,

    /// Pointer to a wide-character string buffer (UTF-16 encoded).
    pub Buffer: *mut u16,
}

/// Double linked list used internally by Windows to link list entries
#[repr(C)]
#[derive(Debug)]
pub struct LIST_ENTRY {
    /// Pointer to the next entry in the list (forward link).
    pub flink: *mut LIST_ENTRY,
    blink: *mut LIST_ENTRY,
}
/// Represents an entry in the process loader's module list.
///
/// Each loaded module (DLL or EXE) is represented by an `LDR_DATA_TABLE_ENTRY`
/// in the loader's linked lists. This structure contains metadata about
/// the loaded image, such as its base address, entry point, and name.
#[repr(C)]
#[derive(Debug)]
pub struct LDR_DATA_TABLE_ENTRY {
    InLoadOrderLinks: LIST_ENTRY,
    InMemoryOrderLinks: LIST_ENTRY,
    InInitializationOrderLinks: LIST_ENTRY,

    /// Base address where the module is loaded in memory.
    pub DllBase: *mut c_void,

    /// Entry point address of the module.
    pub EntryPoint: *mut c_void,

    /// Size, in bytes, of the loaded module image.
    pub SizeOfImage: u32,

    FullDllName: UNICODE_STRING,

    /// Unicode string containing the base name of the loaded module.
    ///
    /// This usually corresponds to the file name of the DLL or EXE
    /// without the full path.
    pub BaseDllName: UNICODE_STRING,
}

/// Contains information about the loaded modules for a process.
///
/// The `PEB_LDR_DATA` structure is referenced by the [`PEB`] and maintains
/// multiple linked lists representing modules in their load, memory, and
/// initialization order.
#[repr(C)]
#[derive(Debug)]
pub struct PEB_LDR_DATA {
    Length: u32,
    Initialized: u8,
    SsHandle: *mut c_void,

    /// Head of a doubly linked list of [`LDR_DATA_TABLE_ENTRY`] structures
    /// representing all loaded modules in their load order.
    pub InLoadOrderModuleList: LIST_ENTRY,

    InMemoryOrderModuleList: LIST_ENTRY,
    InInitializationOrderModuleList: LIST_ENTRY,
}

/// Represents the MS-DOS header at the start of a PE (Portable Executable) file.
///
/// This is the first structure in a PE file, followed by a DOS stub and then the
/// NT headers located at `e_lfanew`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_DOS_HEADER {
    /// Magic number identifying a DOS MZ executable ("MZ").
    pub e_magic: u16,
    e_cblp: u16,
    e_cp: u16,
    e_crlc: u16,
    e_cparhdr: u16,
    e_minalloc: u16,
    e_maxalloc: u16,
    e_ss: u16,
    e_sp: u16,
    e_csum: u16,
    e_ip: u16,
    e_cs: u16,
    e_lfarlc: u16,
    e_ovno: u16,
    e_res: [u16; 4],
    e_oemid: u16,
    e_oeminfo: u16,
    e_res2: [u16; 10],
    /// Offset, in bytes, from the start of the file to the [`IMAGE_NT_HEADERS64`] structure.
    pub e_lfanew: u32,
}

/// Represents the NT headers of a 64-bit PE image.
///
/// This structure contains the PE signature, file header, and optional header,
/// describing the layout and characteristics of the executable image.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_NT_HEADERS64 {
    /// PE file signature ("PE\0\0").
    pub Signature: u32,

    /// COFF file header containing general image characteristics.
    pub FileHeader: IMAGE_FILE_HEADER,

    /// Optional header containing additional information about the image,
    /// including entry point, image base, and section alignment.
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER64,
}

/// Represents the COFF file header in a PE image.
///
/// This structure provides general information about the target machine,
/// number of sections, symbol table, and characteristics of the executable.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_FILE_HEADER {
    Machine: u16,

    /// Number of sections in the PE image.
    pub NumberOfSections: u16,

    TimeDateStamp: u32,

    /// File pointer to the COFF symbol table (deprecated in modern PE files).
    pub PointerToSymbolTable: u32,

    /// Number of symbols in the COFF symbol table (deprecated).
    pub NumberOfSymbols: u32,

    /// Size, in bytes, of the optional header following this structure.
    pub SizeOfOptionalHeader: u16,

    Characteristics: u16,
}

/// Represents the 64-bit PE Optional Header structure.
///
/// This structure provides essential information for loading and executing
/// a PE image in memory, including entry point, image base, and data directories
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_OPTIONAL_HEADER64 {
    Magic: u16,
    MajorLinkerVersion: u8,
    MinorLinkerVersion: u8,
    SizeOfCode: u32,
    SizeOfInitializedData: u32,
    SizeOfUninitializedData: u32,

    /// RVA (Relative Virtual Address) of the image's entry point.
    ///
    /// This value represents the starting address where execution begins
    /// relative to the image's base address.
    pub AddressOfEntryPoint: u32,
    BaseOfCode: u32,

    /// Preferred base address of the image when loaded into memory.
    pub ImageBase: u64,
    SectionAlignment: u32,
    FileAlignment: u32,
    MajorOperatingSystemVersion: u16,
    MinorOperatingSystemVersion: u16,
    MajorImageVersion: u16,
    MinorImageVersion: u16,
    MajorSubsystemVersion: u16,
    MinorSubsystemVersion: u16,
    Win32VersionValue: u32,
    /// Total size, in bytes, of the loaded image, including all headers
    /// and sections, aligned to the specified section alignment.
    pub SizeOfImage: u32,

    /// Combined size, in bytes, of all headers in the PE file,
    /// aligned to the specified file alignment.
    pub SizeOfHeaders: u32,
    CheckSum: u32,
    Subsystem: u16,
    DllCharacteristics: u16,
    SizeOfStackReserve: u64,
    SizeOfStackCommit: u64,
    SizeOfHeapReserve: u64,
    SizeOfHeapCommit: u64,
    LoaderFlags: u32,
    NumberOfRvaAndSizes: u32,

    /// Array of data directory entries describing the locations
    /// and sizes of various tables and resources within the image.
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}

/// Represents a single data directory entry in the PE Optional Header.
///
/// Each entry specifies the RVA and size of a specific table or data structure
/// (e.g., export table, import table, resource table).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_DATA_DIRECTORY {
    /// Relative Virtual Address (RVA) of the data directory.
    pub VirtualAddress: u32,

    /// Size, in bytes, of the data directory.
    pub Size: u32,
}


/// Contains information about exported functions of a PE image.
///
/// The `IMAGE_EXPORT_DIRECTORY` structure describes the export table,
/// which holds function names, ordinals, and addresses available for
/// external modules to import.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_EXPORT_DIRECTORY {
    Characteristics: u32,
    TimeDateStamp: u32,
    MajorVersion: u16,
    MinorVersion: u16,
    Name: u32,
    Base: u32,

    /// Number of exported functions in the table.
    pub NumberOfFunctions: u32,

    /// Number of named entries in the export table.
    pub NumberOfNames: u32,

    /// RVA of the export address table, containing function entry points.
    pub AddressOfFunctions: u32,

    /// RVA of the name pointer table, containing function names.
    pub AddressOfNames: u32,

    /// RVA of the ordinal table, mapping names to function ordinals.
    pub AddressOfNameOrdinals: u32,
}

/// Describes a single section of a PE image.
///
/// Each section contains code, data, or other resources loaded into memory.
/// The `IMAGE_SECTION_HEADER` structure defines the size, location,
/// and characteristics of each section.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_SECTION_HEADER {
    /// ASCII name of the section (up to 8 bytes, null-padded).
    pub Name: [u8; 8],

    /// Total size of the section when loaded into memory.
    pub VirtualSize: u32,

    /// RVA of the first byte of the section when loaded.
    pub VirtualAddress: u32,

    /// Size, in bytes, of the section's data on disk.
    pub SizeOfRawData: u32,

    /// File pointer to the section's first byte of data.
    pub PointerToRawData: u32,

    PointerToRelocations: u32,
    PointerToLinenumbers: u32,
    NumberOfRelocations: u16,
    NumberOfLinenumbers: u16,

    /// Section flags describing memory characteristics
    /// (e.g., executable, writable, readable).
    pub Characteristics: u32,
}

/// Represents a single base relocation block in the PE image.
///
/// Each `IMAGE_BASE_RELOCATION` structure describes a block of addresses
/// that require relocation when the image is loaded at a base address
/// different from its preferred base.
#[repr(C)]
pub struct IMAGE_BASE_RELOCATION {
    /// RVA (Relative Virtual Address) of the page where the relocations apply.
    pub VirtualAddress: u32,

    /// Total size of the relocation block, including the header and all entries.
    pub SizeOfBlock: u32,
}

/// Describes a single import descriptor entry in the PE import table.
///
/// Each `IMAGE_IMPORT_DESCRIPTOR` structure provides information about
/// an imported DLL and the functions or data imported from it.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    /// Anonymous union containing the `OriginalFirstThunk` value.
    pub Anonymous: IMAGE_IMPORT_DESCRIPTOR_0,

    TimeDateStamp: u32,
    ForwarderChain: u32,

    /// RVA of the null-terminated ASCII string containing the name of the DLL.
    pub Name: u32,

    /// RVA of the Import Address Table (IAT) that holds
    /// the resolved function pointers for this DLL.
    pub FirstThunk: u32,
}

/// Anonymous union within `IMAGE_IMPORT_DESCRIPTOR`.
///
/// Holds either the `OriginalFirstThunk` value or other characteristics.
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_IMPORT_DESCRIPTOR_0 {
    Characteristics: u32,

    /// RVA of the Import Name Table (INT) containing
    /// pointers to `IMAGE_IMPORT_BY_NAME` structures.
    pub OriginalFirstThunk: u32,
}

/// Represents a single thunk entry in the import table for 64-bit images.
///
/// Each thunk entry can store an ordinal, a pointer to a hint/name structure,
/// or a resolved function address.
#[repr(C)]
pub struct IMAGE_THUNK_DATA64 {
    /// Anonymous union containing different possible thunk data representations.
    pub u1: IMAGE_THUNK_DATA64_0,
}

/// Union representing different types of data that can be stored
/// in an import thunk entry.
#[repr(C)]
pub union IMAGE_THUNK_DATA64_0 {
    ForwarderString: u64,

    /// Address of the imported function once resolved by the loader.
    pub Function: u64,

    /// Ordinal value of the imported function (if imported by ordinal).
    pub Ordinal: u64,

    /// RVA of an `IMAGE_IMPORT_BY_NAME` structure describing the imported symbol.
    pub AddressOfData: u64,
}

/// Represents an import-by-name entry in the import table.
///
/// Each `IMAGE_IMPORT_BY_NAME` structure contains a hint and a symbol name
/// for a function or variable imported by name from a DLL.
#[repr(C)]
pub struct IMAGE_IMPORT_BY_NAME {
    Hint: u16,

    /// Null-terminated ASCII string containing the name of the imported function or symbol.
    pub Name: [i8; 1],
}

/// Represents the Thread Local Storage (TLS) directory for 64-bit images.
///
/// The `IMAGE_TLS_DIRECTORY64` structure defines the TLS template and callback information
/// that the Windows loader uses to initialize TLS data for threads in a PE image.
#[repr(C)]
pub struct IMAGE_TLS_DIRECTORY64 {
    /// Starting virtual address of the TLS template raw data.
    pub StartAddressOfRawData: ULONGLONG,

    /// Ending virtual address of the TLS template raw data.
    pub EndAddressOfRawData: ULONGLONG,

    /// Virtual address of the index variable used by the TLS runtime to access thread data.
    pub AddressOfIndex: ULONGLONG,

    /// Virtual address of an array of TLS callback functions executed by the loader
    /// during process and thread initialization or termination.
    pub AddressOfCallbacks: ULONGLONG,

    /// Size of the zero-initialized TLS memory section, in bytes.
    pub SizeOfZeroFill: DWORD,

    Reserved0: DWORD,
    Alignment: DWORD,
    Reserved1: DWORD,
}

/// Describes the components of a parsed URL string.
///
/// The `URL_COMPONENTSA` structure is used with the Windows API function
/// `InternetCrackUrlA` to retrieve individual parts of a URL.
#[repr(C)]
#[derive(Debug)]
pub struct URL_COMPONENTSA {
    /// Size of this structure, in bytes.
    pub dwStructSize: DWORD,

    /// Pointer to a buffer that receives the URL scheme (e.g., "http").
    pub lpszScheme: LPSTR,

    /// Length of the scheme string, in characters.
    pub dwSchemeLength: DWORD,

    /// Scheme type value as determined by the API.
    pub nScheme: c_int,

    /// Pointer to a buffer that receives the host name (e.g., "example.com").
    pub lpszHostName: LPSTR,

    /// Length of the host name string, in characters.
    pub dwHostNameLength: DWORD,

    /// Port number extracted from the URL.
    pub nPort: USHORT,

    /// Pointer to a buffer that receives the username, if present in the URL.
    pub lpszUserName: LPSTR,

    /// Length of the username string, in characters.
    pub dwUserNameLength: DWORD,

    /// Pointer to a buffer that receives the password, if present in the URL.
    pub lpszPassword: LPSTR,

    /// Length of the password string, in characters.
    pub dwPasswordLength: DWORD,

    /// Pointer to a buffer that receives the URL path (e.g., "/index.html").
    pub lpszUrlPath: LPSTR,

    /// Length of the URL path string, in characters.
    pub dwUrlPathLength: DWORD,

    /// Pointer to a buffer that receives any extra information or query parameters (e.g., "?id=1").
    pub lpszExtraInfo: LPSTR,

    /// Length of the extra info string, in characters.
    pub dwExtraInfoLength: DWORD,
}

/// Represents an entry in the function table on 64-bit Windows.
#[repr(C)]
#[derive(Debug)]
pub struct RUNTIME_FUNCTION {
    /// The address of the start of the function.
    pub BeginAddress: DWORD,
    /// The address of the end of the function.
    pub EndAddress: DWORD,
    /// The address of the unwind information for the function.
    pub UnwindData: DWORD,
}