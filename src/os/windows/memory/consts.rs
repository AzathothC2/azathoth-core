/// Memory protection: Read-only.
pub const PAGE_READONLY: u32 = 0x02;

/// Memory protection: Read/write access.
pub const PAGE_READWRITE: u32 = 0x04;

/// Memory protection: No-access.
pub const PAGE_NOACCESS: u32 = 0x01;

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

/// Memory allocation flag: Release memory pages.
pub const MEM_RELEASE: u32 = 0x00008000;