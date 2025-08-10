/// Generic Error enums
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum ErrorClass {
    Unspecified = 0,
    Core = 1,
    Alloc = 2,
    Os = 3,
    Dynload = 4,
    Io = 5,
    Other = 255
}

/// Trait that helps with making sure all errors use the same format
pub trait AzError: Copy + Eq + core::fmt::Display {
    const CLASS: ErrorClass;

    fn code(&self) -> u16;
    fn os_code(&self) -> Option<u32> { None }
    fn is_retryable(&self) -> bool { false }
}