#![allow(non_snake_case, non_camel_case_types)]

use crate::os::windows::types::{DWORD, ULONGLONG};

/// Represents the MS-DOS header at the start of a PE file.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_DOS_HEADER {
    /// Magic number identifying a DOS MZ executable ("MZ").
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    /// Offset from the start of the file to [`IMAGE_NT_HEADERS64`].
    pub e_lfanew: u32,
}

/// NT headers of a 64-bit PE image.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_NT_HEADERS64 {
    /// PE file signature ("PE\0\0").
    pub Signature: u32,
    /// COFF file header.
    pub FileHeader: IMAGE_FILE_HEADER,
    /// Optional header with entry point, image base, and section alignment.
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER64,
}

/// COFF file header in a PE image.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_FILE_HEADER {
    pub Machine: u16,
    /// Number of sections in the PE image.
    pub NumberOfSections: u16,
    pub TimeDateStamp: u32,
    /// File pointer to the COFF symbol table (deprecated).
    pub PointerToSymbolTable: u32,
    /// Number of symbols in the COFF symbol table (deprecated).
    pub NumberOfSymbols: u32,
    /// Size in bytes of the optional header.
    pub SizeOfOptionalHeader: u16,
    pub Characteristics: u16,
}

/// 64-bit PE Optional Header.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_OPTIONAL_HEADER64 {
    pub Magic: u16,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    /// RVA of the image's entry point.
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    /// Preferred base address of the image when loaded.
    pub ImageBase: u64,
    pub SectionAlignment: u32,
    pub FileAlignment: u32,
    pub MajorOperatingSystemVersion: u16,
    pub MinorOperatingSystemVersion: u16,
    pub MajorImageVersion: u16,
    pub MinorImageVersion: u16,
    pub MajorSubsystemVersion: u16,
    pub MinorSubsystemVersion: u16,
    pub Win32VersionValue: u32,
    /// Total size in bytes of the loaded image.
    pub SizeOfImage: u32,
    /// Combined size in bytes of all headers.
    pub SizeOfHeaders: u32,
    pub CheckSum: u32,
    pub Subsystem: u16,
    pub DllCharacteristics: u16,
    pub SizeOfStackReserve: u64,
    pub SizeOfStackCommit: u64,
    pub SizeOfHeapReserve: u64,
    pub SizeOfHeapCommit: u64,
    pub LoaderFlags: u32,
    pub NumberOfRvaAndSizes: u32,
    /// Data directory entries.
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}

/// Single data directory entry in the PE Optional Header.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_DATA_DIRECTORY {
    /// RVA of the data directory.
    pub VirtualAddress: u32,
    /// Size in bytes of the data directory.
    pub Size: u32,
}

/// Export directory of a PE image.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_EXPORT_DIRECTORY {
    pub Characteristics: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub Name: u32,
    pub Base: u32,
    /// Number of exported functions.
    pub NumberOfFunctions: u32,
    /// Number of named entries in the export table.
    pub NumberOfNames: u32,
    /// RVA of the export address table.
    pub AddressOfFunctions: u32,
    /// RVA of the name pointer table.
    pub AddressOfNames: u32,
    /// RVA of the ordinal table.
    pub AddressOfNameOrdinals: u32,
}

/// Describes a single section of a PE image.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IMAGE_SECTION_HEADER {
    /// ASCII name of the section (up to 8 bytes, null-padded).
    pub Name: [u8; 8],
    /// Total size of the section when loaded into memory.
    pub VirtualSize: u32,
    /// RVA of the first byte of the section.
    pub VirtualAddress: u32,
    /// Size in bytes of the section's data on disk.
    pub SizeOfRawData: u32,
    /// File pointer to the section's data.
    pub PointerToRawData: u32,
    pub PointerToRelocations: u32,
    pub PointerToLinenumbers: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    /// Section flags (executable, writable, readable, etc.).
    pub Characteristics: u32,
}

/// Base relocation block in a PE image.
#[repr(C)]
pub struct IMAGE_BASE_RELOCATION {
    /// RVA of the page where relocations apply.
    pub VirtualAddress: u32,
    /// Total size of the relocation block including the header.
    pub SizeOfBlock: u32,
}

/// Import descriptor entry in the PE import table.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    pub Anonymous: IMAGE_IMPORT_DESCRIPTOR_0,
    pub TimeDateStamp: u32,
    pub ForwarderChain: u32,
    /// RVA of the null-terminated ASCII DLL name.
    pub Name: u32,
    /// RVA of the Import Address Table (IAT).
    pub FirstThunk: u32,
}

/// Anonymous union within [`IMAGE_IMPORT_DESCRIPTOR`].
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_IMPORT_DESCRIPTOR_0 {
    pub Characteristics: u32,
    /// RVA of the Import Name Table (INT).
    pub OriginalFirstThunk: u32,
}

/// Single thunk entry in the 64-bit import table.
#[repr(C)]
pub struct IMAGE_THUNK_DATA64 {
    pub u1: IMAGE_THUNK_DATA64_0,
}

/// Union of possible values in an import thunk entry.
#[repr(C)]
pub union IMAGE_THUNK_DATA64_0 {
    pub ForwarderString: u64,
    /// Resolved function address.
    pub Function: u64,
    /// Import ordinal value.
    pub Ordinal: u64,
    /// RVA of an [`IMAGE_IMPORT_BY_NAME`] structure.
    pub AddressOfData: u64,
}

/// Import-by-name entry in the import table.
#[repr(C)]
pub struct IMAGE_IMPORT_BY_NAME {
    pub Hint: u16,
    /// Null-terminated ASCII name of the imported function.
    pub Name: [i8; 1],
}

/// TLS directory for 64-bit PE images.
#[repr(C)]
pub struct IMAGE_TLS_DIRECTORY64 {
    /// Starting virtual address of the TLS raw data.
    pub StartAddressOfRawData: ULONGLONG,
    /// Ending virtual address of the TLS raw data.
    pub EndAddressOfRawData: ULONGLONG,
    /// Virtual address of the TLS index variable.
    pub AddressOfIndex: ULONGLONG,
    /// Virtual address of the TLS callback array.
    pub AddressOfCallbacks: ULONGLONG,
    /// Size of the zero-initialized TLS section in bytes.
    pub SizeOfZeroFill: DWORD,
    Reserved0: DWORD,
    Alignment: DWORD,
    Reserved1: DWORD,
}

/// Entry in the 64-bit function table used for exception handling.
#[repr(C)]
#[derive(Debug)]
pub struct RUNTIME_FUNCTION {
    /// Start address of the function.
    pub BeginAddress: DWORD,
    /// End address of the function.
    pub EndAddress: DWORD,
    /// Address of the unwind information.
    pub UnwindData: DWORD,
}
