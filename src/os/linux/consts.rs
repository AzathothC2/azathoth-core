
/// Load all symbols immediately (`dlopen` flag).
pub const RTLD_NOW: i32 = 0x00002;

/// Lazy-load symbols as they are used (`dlopen` flag).
pub const RTLD_LAZY: i32 = 1;
/// Program header type: Loadable segment.
pub const PT_LOAD: u32 = 1;

/// Program header type: Dynamic linking information segment.
pub const PT_DYNAMIC: u32 = 2;

/// Initialize cURL with SSL support.
pub const CURL_GLOBAL_SSL: u64 = 1 << 0;

/// Initialize cURL with Windows-specific socket support (no-op on Linux).
pub const CURL_GLOBAL_WIN32: u64 = 1 << 1;

/// Initialize cURL with all common features enabled (SSL + Win32).
pub const CURL_GLOBAL_ALL: u64 = CURL_GLOBAL_SSL | CURL_GLOBAL_WIN32;

/// cURL option: Specify custom write data pointer.
pub const CURLOPT_WRITEDATA: u32 = 10001;

/// cURL option: Set the target URL for the request.
pub const CURLOPT_URL: u32 = 10002;

/// cURL option: Provide a custom write callback function.
pub const CURLOPT_WRITEFUNCTION: u32 = 20011;

/// cURL option: Set a custom User-Agent string for the request.
pub const CURLOPT_USERAGENT: u32 = 10018;

/// GZIP file magic number (first two bytes).
pub const GZIP_MAGIC: [u8; 2] = [0x1f, 0x8b];

/// Zlib-compressed data magic byte (first byte).
pub const ZLIB_MAGIC: u8 = 0x78;

/// File permission: Read access for the file owner.
pub const S_IRUSR: u16 = 0o400;

/// File permission: Write access for the file owner.
pub const S_IWUSR: u16 = 0o200;

/// File permission: Execute/search access for the file owner.
pub const S_IXUSR: u16 = 0o100;

/// Open file read-only flag.
pub const O_RDONLY: usize = 0;

/// File descriptor for standard output (`stdout`).
pub const STDOUT: usize = 1;

/// File descriptor for standard error (`stderr`).
pub const STDERR: usize = 2;

/// Memory protection flag: Readable memory region.
pub const PROT_READ: i32 = 1;

/// Memory protection flag: Executable memory region.
pub const PROT_EXEC: i32 = 4;

/// Memory protection flag: Writable memory region.
pub const PROT_WRITE: i32 = 2;

/// Mapping flag: Map memory that is not backed by a file (anonymous mapping).
pub const MAP_ANONYMOUS: i32 = 0x20;

/// Mapping flag: Create a private copy-on-write mapping.
pub const MAP_PRIVATE: i32 = 2;

/// Flag for `mremap`: Allow relocation of the memory region if necessary.
pub const MREMAP_MAYMOVE: i32 = 1;

/// Signal number for abnormal process termination (abort).
pub const SIG_ABRT: usize = 134;

/// Linux syscall number for `write`.
pub const SYS_WRITE: usize = 1;

/// Linux syscall number for `mmap`.
pub const SYS_MMAP: usize = 9;

/// Linux syscall number for `munmap`.
pub const SYS_MUNMAP: usize = 11;

/// Linux syscall number for `close`.
pub const SYS_CLOSE: usize = 3;

/// Linux syscall number for `getdents64` (read directory entries).
pub const SYS_GETDENTS64: usize = 217;

/// Linux syscall number for `open`.
pub const SYS_OPEN: usize = 2;

/// Linux syscall number for `read`.
pub const SYS_READ: usize = 0;

/// Linux syscall number for `lseek`.
pub const SYS_LSEEK: usize = 8;

/// Linux syscall number for `pread64` (read at offset).
pub const SYS_PREAD64: usize = 17;

/// Linux syscall number for `mremap` (resize/move memory mapping).
pub const SYS_MREMAP: usize = 25;

/// Linux syscall number for `exit`.
pub const SYS_EXIT: usize = 60;

/// Linux syscall number for `exit_group` (terminate all threads).
pub const SYS_EXIT_GROUP: usize = 231;

/// Move the file offset to the end of the file.
pub const SEEK_END: usize = 2;
