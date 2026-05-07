/// Internet open type: Use system preconfiguration.
pub const INTERNET_OPEN_TYPE_PRECONFIG: u32 = 0;

/// Internet option: Security flags.
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
