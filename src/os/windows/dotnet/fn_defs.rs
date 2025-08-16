#![allow(non_snake_case, non_camel_case_types)]

use core::ffi::c_void;
use super::structs::{SAFEARRAY, SAFEARRAYBOUND};
use crate::os::windows::types::{BSTR, DWORD, HRESULT, LPVOID, REFCLSID, REFIID};

/// Windows CLRCreateInstance typedef: \
/// <https://learn.microsoft.com/en-us/dotnet/core/unmanaged-api/debugging/clrcreateinstance-function>
pub type CLRCreateInstance_t =
unsafe extern "system" fn(clsid: REFCLSID, riid: REFIID, ppInterface: *mut LPVOID) -> HRESULT;

/// Windows CorBindToRuntime typedef: \
/// <https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/hosting/corbindtoruntime-function>
pub type CorBindToRuntime_t = unsafe extern "system" fn(
    pwszVersion: *const u16,
    pwszBuildFlavor: *const u16,
    rclsid: REFCLSID,
    riid: REFIID,
    ppv: *mut LPVOID,
) -> HRESULT;

/// Windows CoCreateInstance typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-cocreateinstance>
pub type CoCreateInstance_t = unsafe extern "system" fn(
    rclsid: *const REFCLSID,
    pUnkOuter: *mut c_void,
    dwClsContext: u32,
    riid: REFIID,
    ppv: *mut *mut c_void,
) -> HRESULT;

/// Windows CoInitializeEx typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex>
pub type CoInitializeEx_t =
unsafe extern "system" fn(pvReserved: LPVOID, dwCoInit: DWORD) -> HRESULT;

/// Windows CoUninitialize typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-couninitialize>
pub type CoUninitialize_t = unsafe extern "system" fn();

/// Windows SysAllocString typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysallocstring>
pub type SysAllocString_t = unsafe extern "system" fn(psz: *const u16) -> BSTR;

/// Windows SysFreeString typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-sysfreestring>
pub type SysFreeString_t = unsafe extern "system" fn(bstr: BSTR);

/// Windows SafeArrayCreate typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-safearraycreate>
pub type SafeArrayCreate_t = unsafe extern "system" fn(
    vt: u16,
    cDims: u32,
    rgsabound: *const SAFEARRAYBOUND,
) -> *mut SAFEARRAY;

/// Windows SafeArrayDestroy typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-safearraydestroy>
pub type SafeArrayDestroy_t = unsafe extern "system" fn(psa: *mut SAFEARRAY) -> HRESULT;

/// Windows SafeArrayCreateVector typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-safearraycreatevector>
pub type SafeArrayCreateVector_t =
unsafe extern "system" fn(vt: u16, lLbound: i32, cElements: u32) -> *mut SAFEARRAY;

/// Windows SafeArrayPutElement typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-safearrayputelement>
pub type SafeArrayPutElement_t = unsafe extern "system" fn(
    psa: *mut SAFEARRAY,
    rgIndices: *const i32,
    pv: *const c_void,
) -> HRESULT;

/// Windows SafeArrayGetLBound typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-safearraygetlbound>
pub type SafeArrayGetLBound_t =
unsafe extern "system" fn(psa: *mut SAFEARRAY, nDim: u32, plLbound: *mut i32) -> HRESULT;

/// Windows SafeArrayGetUBound typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-safearraygetubound>
pub type SafeArrayGetUBound_t =
unsafe extern "system" fn(psa: *mut SAFEARRAY, nDim: u32, plUbound: *mut i32) -> HRESULT;

/// Windows SafeArrayAccessData typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-safearrayaccessdata>
pub type SafeArrayAccessData_t =
unsafe extern "system" fn(psa: *mut SAFEARRAY, ppvData: *mut *mut c_void) -> HRESULT;

/// Windows SafeArrayUnAccessData typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-safearrayunaccessdata>
pub type SafeArrayUnAccessData_t = unsafe extern "system" fn(psa: *mut SAFEARRAY) -> HRESULT;

/// Windows SafeArrayGetElement typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/oleauto/nf-oleauto-safearraygetelement>
pub type SafeArrayGetElement_t = unsafe extern "system" fn(
    psa: *mut SAFEARRAY,
    rgIndices: *const i32,
    pv: *mut c_void,
) -> HRESULT;

/// Windows CreateInterface typedef: \
/// <https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/hosting/clrcreateinstance-function>
pub type CreateInterface_t =
unsafe extern "system" fn(clsid: REFCLSID, iid: REFIID, interface: *mut *mut c_void) -> HRESULT;
