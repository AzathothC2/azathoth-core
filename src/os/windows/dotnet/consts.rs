use crate::os::windows::types::VARTYPE;

/// Variant type representing an unknown COM object (`IUnknown*`).
pub const VT_UNKNOWN: VARTYPE = 13;

/// Variant type representing an uninitialized or empty value.
pub const VT_EMPTY: VARTYPE = 0;

/// Variant type representing a `NULL` value (explicitly set).
pub const VT_NULL: VARTYPE = 1;

/// Variant type representing a signed 16-bit integer (`i16`).
pub const VT_I2: VARTYPE = 2;

/// Variant type representing a signed 32-bit integer (`i32`).
pub const VT_I4: VARTYPE = 3;

/// Variant type representing a 32-bit floating-point number (`f32`).
pub const VT_R4: VARTYPE = 4;

/// Variant type representing a 64-bit floating-point number (`f64`).
pub const VT_R8: VARTYPE = 5;

/// Variant type representing a currency value (`CY` structure).
pub const VT_CY: VARTYPE = 6;

/// Variant type representing a date/time value (`DATE` structure).
pub const VT_DATE: VARTYPE = 7;

/// Variant type representing a BSTR (string) pointer (`BSTR`).
pub const VT_BSTR: VARTYPE = 8;

/// Variant type representing an `IDispatch` interface pointer.
pub const VT_DISPATCH: VARTYPE = 9;

/// Variant type representing an error code (`SCODE`).
pub const VT_ERROR: VARTYPE = 10;

/// Variant type representing a Boolean value (`VARIANT_BOOL`).
///
/// `VARIANT_TRUE` = `-1` and `VARIANT_FALSE` = `0`.
pub const VT_BOOL: VARTYPE = 11;

/// Variant type representing another `VARIANT` structure (nested).
pub const VT_VARIANT: VARTYPE = 12;

/// Variant type representing a fixed-point decimal value (`DECIMAL`).
pub const VT_DECIMAL: VARTYPE = 14;

/// Variant type representing a signed 8-bit integer (`i8`).
pub const VT_I1: VARTYPE = 16;

/// Variant type representing an unsigned 8-bit integer (`u8`).
pub const VT_UI1: VARTYPE = 17;

/// Variant type representing an unsigned 16-bit integer (`u16`).
pub const VT_UI2: VARTYPE = 18;

/// Variant type representing an unsigned 32-bit integer (`u32`).
pub const VT_UI4: VARTYPE = 19;

/// Variant type representing a signed 64-bit integer (`i64`).
pub const VT_I8: VARTYPE = 20;

/// Flag modifier indicating that the variant is an array (`SAFEARRAY*`).
///
/// Used in combination with another `VT_*` base type
/// (e.g., `VT_ARRAY | VT_BSTR` for an array of strings).
pub const VT_ARRAY: VARTYPE = 0x2000;
