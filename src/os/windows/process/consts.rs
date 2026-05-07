/// `STARTUPINFO.dwFlags` bit: Use the `hStdInput`, `hStdOutput`, and `hStdError` members.
pub const STARTF_USESTDHANDLES: u32 = 0x00000100;

/// Process creation flag: Suppress creation of a console window.
pub const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Process creation flag: Create the process in a suspended state.
pub const CREATE_SUSPENDED: u32 = 0x00000004;

/// Process creation flag: Use an extended `STARTUPINFOEX` structure.
pub const EXTENDED_STARTUPINFO_PRESENT: u32 = 0x00080000;

/// Process access right: Required to use the process as a parent.
pub const PROCESS_CREATE_PROCESS: u32 = 0x0080;

/// Proc thread attribute: Specifies the parent process to inherit handles from.
pub const PROC_THREAD_ATTRIBUTE_PARENT_PROCESS: usize = 0x00020000;

/// Infinite timeout value for wait functions.
pub const INFINITE: u32 = 0xFFFF_FFFF;
