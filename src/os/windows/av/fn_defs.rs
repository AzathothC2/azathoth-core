#![allow(non_snake_case, non_camel_case_types)]

use crate::os::windows::{av::structs::{EVENT_DATA_DESCRIPTOR, EVENT_DESCRIPTOR, WLDP_HOST_INFORMATION}, types::{BOOL, DWORD, HANDLE, HMODULE, HRESULT, LPCWSTR, PVOID, REFCLSID, ULONG}};


/// Windows AmsiScanBuffer typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/amsi/nf-amsi-amsiscanbuffer>
pub type AmsiScanBuffer_t = unsafe extern "system" fn(amsiContext: HMODULE, buffer: PVOID, length: ULONG, contentName: LPCWSTR, amsiSession: HMODULE, result: &mut u32) -> HRESULT;

/// Windows AmsiScanString typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/amsi/nf-amsi-amsiscanstring>
pub type AmsiScanString_t = unsafe extern "system" fn(amsiContext: HMODULE, string: LPCWSTR, contentName: LPCWSTR, amsiSession: HMODULE, result: &mut u32) -> HRESULT;

/// Windows WldpQueryDynamicCodeTrust typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/wldp/nf-wldp-wldpquerydynamiccodetrust>
pub type WldpQueryDynamicCodeTrust_t = unsafe extern "system" fn(fileHandle: HANDLE, baseImage: PVOID, imageSize: ULONG) -> HRESULT;

/// Windows WldpIsClassInApprovedList typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/wldp/nf-wldp-wldpisclassinapprovedlist>
pub type WldpIsClassInApprovedList_t = unsafe extern "system" fn(classId: REFCLSID, hostInformation: *const WLDP_HOST_INFORMATION, isApproved: *mut BOOL, optionalFlags: DWORD) -> HRESULT;

/// Windows EventWrite typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/evntprov/nf-evntprov-eventwrite>
pub type EventWrite_t = unsafe extern "system" fn(RegHandle: u64, HANDLE, EventDecriptor: *mut EVENT_DESCRIPTOR, UserDataCount: ULONG, UserData: *mut EVENT_DATA_DESCRIPTOR) -> DWORD;