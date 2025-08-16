#![allow(non_snake_case)]

use core::ffi::c_void;
use crate::os::windows::types::{LONG, VARTYPE, WORD};

/// Represents the underlying data storage of a COM `VARIANT`.
///
/// The `VariantData` union holds different types of values that a `VARIANT`
/// can represent, including integers, floating-point numbers, strings, COM
/// interface pointers, and arrays.
///
/// Only one field is valid at a time, determined by the `vt` field
/// in the enclosing [`VARIANT`] struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub union VariantData {
    llVal: i64,
    lVal: LONG,
    bVal: u8,
    iVal: i16,
    fltVal: f32,
    dblVal: f64,
    boolVal: i16,
    scode: i32,
    cyVal: i64,
    date: f64,
    bstrVal: *mut u16,
    punkVal: *mut c_void,
    pdispVal: *mut c_void,
    /// Pointer to a [`SAFEARRAY`] value, used for storing array data in a VARIANT.
    pub parray: *mut SAFEARRAY,
    byref: *mut c_void,
}


/// Represents a COM `VARIANT` type, used to store different data types in a
/// generic format.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VARIANT {
    /// Specifies the type of value stored in this VARIANT.
    pub vt: VARTYPE,
    /// Reserved for internal use.
    pub w_reserved1: WORD,
    /// Reserved for internal use.
    pub w_reserved2: WORD,
    /// Reserved for internal use.
    pub w_reserved3: WORD,
    /// Contains the value of this VARIANT. Its active field depends on `vt`.
    pub data: VariantData,
}

/// Defines the bounds of a single dimension in a `SAFEARRAY`.
///
/// Each `SAFEARRAYBOUND` structure describes the size and lower bound
/// of one array dimension.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SAFEARRAYBOUND {
    /// Number of elements in the array dimension.
    pub cElements: u32,

    /// The lower bound (starting index) of the array dimension.
    pub lLbound: i32,
}


/// Represents a self-describing array used in COM interoperability.
///
/// Although this struct does not expose any `pub` fields directly,
/// it is often used in FFI calls to COM functions and manipulated via
/// Windows API helper functions (e.g., `SafeArrayCreate`, `SafeArrayAccessData`).
#[repr(C)]
pub struct SAFEARRAY {
    cDims: u16,
    fFeatures: u16,
    cbElements: u32,
    cLocks: u32,
    pvData: *mut u8,
    rgsabound: [SAFEARRAYBOUND; 1],
}

