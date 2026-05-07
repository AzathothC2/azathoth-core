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

/// AMD64 relocation: no relocation.
pub const IMAGE_REL_AMD64_ABSOLUTE: u16 = 0x0000;
/// AMD64 relocation: 64-bit VA of the relocation target.
pub const IMAGE_REL_AMD64_ADDR64: u16 = 0x0001;
/// AMD64 relocation: 32-bit VA of the relocation target.
pub const IMAGE_REL_AMD64_ADDR32: u16 = 0x0002;
/// AMD64 relocation: 32-bit signed VA of the relocation target (no image base added).
pub const IMAGE_REL_AMD64_ADDR32NB: u16 = 0x0003;
/// AMD64 relocation: 32-bit relative displacement from the next instruction.
pub const IMAGE_REL_AMD64_REL32: u16 = 0x0004;
/// AMD64 relocation: 32-bit relative with -1 addend.
pub const IMAGE_REL_AMD64_REL32_1: u16 = 0x0005;
/// AMD64 relocation: 32-bit relative with -2 addend.
pub const IMAGE_REL_AMD64_REL32_2: u16 = 0x0006;
/// AMD64 relocation: 32-bit relative with -3 addend.
pub const IMAGE_REL_AMD64_REL32_3: u16 = 0x0007;
/// AMD64 relocation: 32-bit relative with -4 addend.
pub const IMAGE_REL_AMD64_REL32_4: u16 = 0x0008;
/// AMD64 relocation: 32-bit relative with -5 addend.
pub const IMAGE_REL_AMD64_REL32_5: u16 = 0x0009;
/// AMD64 relocation: 16-bit section index.
pub const IMAGE_REL_AMD64_SECTION: u16 = 0x000A;
/// AMD64 relocation: 32-bit offset from section start.
pub const IMAGE_REL_AMD64_SECREL: u16 = 0x000B;
/// AMD64 relocation: 7-bit offset from section start (CLR token).
pub const IMAGE_REL_AMD64_SECREL7: u16 = 0x000C;
/// AMD64 relocation: 32-bit CLR token.
pub const IMAGE_REL_AMD64_TOKEN: u16 = 0x000D;
/// AMD64 relocation: 32-bit signed relative displacement from SFP (stack frame pointer).
pub const IMAGE_REL_AMD64_SREL32: u16 = 0x000E;
/// AMD64 relocation: CLR pair — must precede SREL32.
pub const IMAGE_REL_AMD64_PAIR: u16 = 0x000F;
/// AMD64 relocation: 32-bit signed span-dependent value emitted in object.
pub const IMAGE_REL_AMD64_SSPAN32: u16 = 0x0010;

/// Relocation type: 64-bit address relocation.
pub const IMAGE_REL_BASED_DIR64: u16 = 10;

/// Relocation type: 32-bit high/low relocation.
pub const IMAGE_REL_BASED_HIGHLOW: u16 = 3;

/// Ordinal flag for 64-bit imports (high bit set).
pub const IMAGE_ORDINAL_FLAG64: u64 = 0x8000000000000000;

/// DLL entrypoint reason: Process attach.
pub const DLL_PROCESS_ATTACH: u32 = 1;
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 0x00000080;
pub const IMAGE_SCN_CNT_CODE: u32 = 0x00000020;