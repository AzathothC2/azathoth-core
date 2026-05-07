/// Generic access right: Read access.
pub const GENERIC_READ: u32 = 0x8000_0000;

/// Generic access right: Write access.
pub const GENERIC_WRITE: u32 = 0x4000_0000;

/// Standard access right: Synchronize access to an object.
pub const SYNCHRONIZE: u32 = 0x0010_0000;

/// File-specific access right: Write file attributes.
pub const FILE_WRITE_ATTRIBUTES: u32 = 0x0100;

/// File sharing flag: Allow other processes to read the file.
pub const FILE_SHARE_READ: u32 = 0x0000_0001;

/// File sharing flag: Allow other processes to write the file.
pub const FILE_SHARE_WRITE: u32 = 0x0000_0002;

/// NT file disposition: Create a new file; fail if it already exists.
pub const FILE_CREATE: u32 = 0x0000_0002;

/// NT file disposition: Open an existing file; fail if it does not exist.
pub const FILE_OPEN: u32 = 0x0000_0001;

/// NT file disposition: Open if exists, otherwise create; truncate on open.
pub const FILE_OVERWRITE_IF: u32 = 0x05;

/// NT file create option: Synchronous I/O, alertable waits suppressed.
pub const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 0x0000_0020;

/// NT file create option: Delete file when the last handle is closed.
pub const FILE_DELETE_ON_CLOSE: u32 = 0x1000;

/// NT file attribute: Normal file with no special attributes.
pub const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;

/// Named pipe type flag: Byte-stream pipe.
pub const FILE_PIPE_BYTE_STREAM_TYPE: u32 = 0x0000_0000;

/// Named pipe read-mode flag: Byte-stream read mode.
pub const FILE_PIPE_BYTE_STREAM_MODE: u32 = 0x0000_0000;

/// Named pipe completion flag: Queued (blocking) I/O.
pub const FILE_PIPE_QUEUE_OPERATION: u32 = 0x0000_0000;

/// Object attributes flag: Name comparisons are case-insensitive.
pub const OBJ_CASE_INSENSITIVE: u32 = 0x40;

/// Object attributes flag: Handle is inherited by child processes.
pub const OBJ_INHERIT: u32 = 0x02;

/// NT status code: End of file reached during a read operation.
pub const STATUS_END_OF_FILE: i32 = -0x3FFF_FFFF;
