/// Performs a raw system call with 1 argument.
///
/// This function directly invokes the Linux `syscall` instruction with the
/// specified system call number (`n`) and one argument (`a1`). It returns the
/// result of the syscall as an `isize`, which may be a negative error code
/// (following Linux conventions).
///
/// # Safety
///
/// - The caller must ensure that the provided syscall number and arguments
///   are valid for the target platform.
/// - Passing invalid pointers or arguments may cause undefined behavior,
///   including crashes or data corruption.
/// - This function does not perform any error translation. Negative return
///   values correspond to `-errno`.
///
/// # Examples
/// ```
/// use azathoth_utils::platform::linux::syscalls::syscall1;
///
/// // Example: `getpid` (syscall number 39 on x86_64 Linux)
/// let pid = unsafe { syscall1(39, 0) };
/// assert!(pid > 0);
/// ```
#[inline(always)]
#[unsafe(link_section = ".text")]
pub fn syscall1(n: usize, a1: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
        "syscall",
        inout("rax") n => ret,
        in("rdi") a1
        )
    }
    ret
}

/// Performs a raw system call with 2 arguments.
///
/// Similar to [`syscall1`], but accepts
/// two arguments (`a1`, `a2`). Returns the raw result of the syscall.
///
/// # Safety
/// Same as `syscall1`.
///
/// # Examples
/// ```
/// use azathoth_utils::platform::linux::{syscalls::syscall2, consts::SYS_OPEN};
///
/// let fd = unsafe { syscall2(SYS_OPEN, b"/etc/passwd\0".as_ptr() as usize, 0) };
/// assert!(fd >= 0);
/// ```
#[inline(always)]
#[unsafe(link_section = ".text")]
pub fn syscall2(n: usize, a1: usize, a2: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
        "syscall",
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags),
        )
    }
    ret
}

/// Performs a raw system call with 3 arguments.
///
/// This is commonly used for syscalls such as `read` or `write`.
///
/// # Safety
/// Same as `syscall1`.
///
/// # Examples
/// ```
/// use azathoth_utils::platform::linux::syscalls::syscall3;
///
/// let buf = [0u8; 64];
/// let n = unsafe { syscall3(0, 0, buf.as_ptr() as usize, buf.len()) }; // read(stdin)
/// assert!(n >= 0);
/// ```
#[inline(always)]
#[unsafe(link_section = ".text")]
pub fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
        "syscall",
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        );
    }
    ret
}

/// Performs a raw system call with 4 arguments.
///
/// # Safety
/// Same as `syscall1`.
///
/// # Examples
/// ```
/// use azathoth_utils::platform::linux::syscalls::syscall4;
///
/// let fd = unsafe { syscall4(17, 1, 0, 0, 0) }; // Example syscall
/// ```
#[allow(unused)]
#[inline(always)]
#[unsafe(link_section = ".text")]
pub fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
        "syscall",
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        )
    }
    ret
}

/// Performs a raw system call with 5 arguments.
///
/// # Safety
/// Same as `syscall1`.
///
/// # Examples
/// ```
/// use azathoth_utils::platform::linux::syscalls::syscall5;
///
/// let fd = unsafe { syscall5(5, 0, 0, 0, 0, 0) };
/// ```
#[inline(always)]
#[unsafe(link_section = ".text")]
pub fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
        "syscall",
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        )
    }
    ret
}


/// Performs a raw system call with 6 arguments.
///
/// This is used for syscalls that require up to six parameters, such as `mmap`.
///
/// # Safety
/// Same as `syscall1`.
///
/// # Examples
/// ```
/// use azathoth_utils::platform::linux::{syscalls::syscall6, consts::SYS_MMAP};
///
/// let ptr = unsafe { syscall6(SYS_MMAP, 0, 4096, 3, 0x22, -1isize as usize, 0) };
/// assert!(ptr >= 0);
/// ```
#[inline(always)]
#[unsafe(link_section = ".text")]
pub fn syscall6(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> isize {
    let ret: isize;
    unsafe {
        core::arch::asm!(
        "syscall",
        inlateout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        in("r9") a6,
        lateout("rcx") _, lateout("r11") _,
        );
    }
    ret
}
