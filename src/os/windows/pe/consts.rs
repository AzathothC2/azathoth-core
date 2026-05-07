/// Section flag: Memory is executable.
pub const IMAGE_SCN_MEM_EXECUTE: u32 = 0x20000000;

/// Section flag: Memory is readable.
pub const IMAGE_SCN_MEM_READ: u32 = 0x40000000;

/// Section flag: Memory is writable.
pub const IMAGE_SCN_MEM_WRITE: u32 = 0x80000000;

/// DOS header magic value ('MZ').
pub const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D;

/// PE header signature ('PE\0\0').
pub const IMAGE_NT_SIGNATURE: u32 = 0x00004550;

/// Data directory index: Export table.
pub const IMAGE_DIRECTORY_ENTRY_EXPORT: u16 = 0;

/// Data directory index: Import table.
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: u16 = 1;

/// Data directory index: Exception table.
pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: u16 = 3;

/// Data directory index: Base relocations.
pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: u16 = 5;

/// Data directory index: TLS (Thread Local Storage).
pub const IMAGE_DIRECTORY_ENTRY_TLS: u16 = 9;

/// Relocation type: 64-bit address relocation.
pub const IMAGE_REL_BASED_DIR64: u16 = 10;

/// Relocation type: 32-bit high/low relocation.
pub const IMAGE_REL_BASED_HIGHLOW: u16 = 3;

/// Ordinal flag for 64-bit imports (high bit set).
pub const IMAGE_ORDINAL_FLAG64: u64 = 0x8000000000000000;

/// DLL entrypoint reason: Process attach.
pub const DLL_PROCESS_ATTACH: u32 = 1;
