#![allow(non_snake_case, non_camel_case_types)]

/// Represents an opaque cURL handle type.
///
/// Used as a placeholder for a `CURL` object obtained from the cURL library.
/// This struct is never directly accessed in Rust; it only serves as a type marker
/// for FFI function pointers.
#[repr(C)]
pub struct CURL {
    _private: [u8; 0],
}

/// A raw pointer to a cURL handle (`CURL*`).
///
/// Returned by functions such as `curl_easy_init` and passed to other cURL APIs.
pub type CURLPtr = *mut CURL;

/// Type alias for cURL option identifiers.
///
/// Used with `curl_easy_setopt` and similar functions to specify configuration options.
pub type CURLOption = u32;

/// Represents possible return codes from cURL operations.
///
/// Maps directly to the C `CURLcode` enum returned by cURL library functions.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CURLCode {
    /// Operation completed successfully.
    CURLE_OK = 0,
    /// Unsupported protocol was specified.
    CURLE_UNSUPPORTED_PROTOCOL = 1,
    /// Initialization of a cURL handle failed.
    CURLE_FAILED_INIT = 2,
    /// Malformed URL passed to a cURL function.
    CURLE_URL_MALFORMAT = 3,
    /// Unknown or unspecified cURL error.
    CURLE_UNKNOWN = 9999,
}
