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

/// Memory protection: Read-only.
pub const PAGE_READONLY: u32 = 0x02;

/// Memory protection: Read/write access.
pub const PAGE_READWRITE: u32 = 0x04;

/// Memory protection: Write-copy access.
pub const PAGE_WRITECOPY: u32 = 0x08;

/// Memory protection: Execute only.
pub const PAGE_EXECUTE: u32 = 0x10;

/// Memory protection: Execute + read access.
pub const PAGE_EXECUTE_READ: u32 = 0x20;

/// Memory protection: Execute + read/write access.
pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;

/// Memory protection: Execute + write-copy access.
pub const PAGE_EXECUTE_WRITECOPY: u32 = 0x80;

/// Memory allocation flag: Commit memory pages.
pub const MEM_COMMIT: u32 = 0x00001000;

/// Memory allocation flag: Reserve memory pages.
pub const MEM_RESERVE: u32 = 0x00002000;

/// Relocation type: 64-bit address relocation.
pub const IMAGE_REL_BASED_DIR64: u16 = 10;

/// Relocation type: 32-bit high/low relocation.
pub const IMAGE_REL_BASED_HIGHLOW: u16 = 3;

/// Ordinal flag for 64-bit imports (high bit set).
pub const IMAGE_ORDINAL_FLAG64: u64 = 0x8000000000000000;

/// DLL entrypoint reason: Process attach.
pub const DLL_PROCESS_ATTACH: u32 = 1;

/// Internet open type: Use system preconfiguration.
pub const INTERNET_OPEN_TYPE_PRECONFIG: u32 = 0;

/// Internet option: Security flags
pub const INTERNET_OPTION_SECURITY_FLAGS: u32 = 31;

/// Internet flag: Open asynchronously.
pub const INTERNET_FLAG_ASYNC: u32 = 0;

/// Internet service type: HTTP.
pub const INTERNET_SERVICE_HTTP: u32 = 3;

/// Security flag: Ignore unknown Certificate Authority errors.
pub const SECURITY_FLAG_IGNORE_UNKNOWN_CA: u32 = 0x00000100;

/// Security flag: Ignore invalid certificate date.
pub const SECURITY_FLAG_IGNORE_CERT_DATE_INVALID: u32 = 0x00002000;

/// Security flag: Ignore certificate Common Name mismatch.
pub const SECURITY_FLAG_IGNORE_CERT_CN_INVALID: u32 = 0x00001000;

/// Security flag: Ignore wrong certificate usage.
pub const SECURITY_FLAG_IGNORE_WRONG_USAGE: u32 = 0x00000200;

/// Security flag: Ignore certificate revocation errors.
pub const SECURITY_FLAG_IGNORE_REVOCATION: u32 = 0x00000080;

/// HTTP query flag: Expect a numeric value.
pub const HTTP_QUERY_FLAG_NUMBER: u32 = 0x20000000;

/// HTTP query option: Status code of the response.
pub const HTTP_QUERY_STATUS_CODE: u32 = 19;

/// HTTP query option: Content length of the response.
pub const HTTP_QUERY_CONTENT_LENGTH: u32 = 5;

/// InternetCrackUrlA decode flag: Decode percent-encoded characters.
pub const ICU_DECODE: u32 = 0x10000000;
