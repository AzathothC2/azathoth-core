#![allow(non_snake_case, non_camel_case_types)]


use super::structs::{LDR_DATA_TABLE_ENTRY, URL_COMPONENTSA};
use super::types::{BOOL, DWORD, FARPROC, HANDLE, HINTERNET, HMODULE, LPCSTR, LPCWSTR, LPDWORD, LPVOID, LPWSTR, NTSTATUS, PCSTR};
use core::ffi::{c_ushort, c_void};

/// Windows LoadLibraryA typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibrarya>
pub type LoadLibraryA_t = unsafe extern "system" fn(lib: PCSTR) -> HMODULE;

/// Windows CreateThread typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread>
pub type CreateThread_t = unsafe extern "system" fn(
    lpThreadSecurityAttributes: *mut c_void,
    dwStackSize: usize,
    lpStartAddress: unsafe extern "system" fn(*mut c_void) -> u32,
    lpParameter: *mut c_void,
    dwCreationflags: u32,
    lpThreadId: *mut u32,
) -> HANDLE;

/// Windows GetProcAddress typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress>
pub type GetProcAddress_t = unsafe extern "system" fn(hmod: HMODULE, procname: PCSTR) -> FARPROC;

/// Windows GetModuleHandleA typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress>
pub type GetModuleHandleA_t = unsafe extern "system" fn(lpModuleName: LPCSTR) -> HMODULE;

/// Windows VirtualAlloc typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc>
pub type VirtualAlloc_t = unsafe extern "system" fn(
    addr: *const c_void,
    size: usize,
    alloc_type: u32,
    protect: u32,
) -> HANDLE;

/// Windows VirtualProtect typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualprotect>
pub type VirtualProtect_t = unsafe extern "system" fn(
    addr: *const c_void,
    size: usize,
    new_protect: u32,
    old_protect: *mut u32,
) -> BOOL;

///  Windows VirtualFree typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualfree>
pub type VirtualFree_t =
unsafe extern "system" fn(addr: *mut c_void, size: usize, free_type: u32) -> BOOL;

/// Windows ExitThread typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitthread>
pub type ExitThread_t = unsafe extern "system" fn(code: u32) -> !;

/// Windows ExitProcess typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-exitprocess>
pub type ExitProcess_t = unsafe extern "system" fn(code: u32) -> !;

/// Windows FlushInstructionCache typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-flushinstructioncache>
pub type FlushInstructionCache_t =
unsafe extern "system" fn(hproc: HANDLE, base_addr: *const c_void, size: usize) -> BOOL;

/// Windows RtlDecompressBuffer typedef: \
/// <https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntifs/nf-ntifs-rtldecompressbuffer>
pub type RtlDecompressBuffer_t = unsafe extern "system" fn(
    CompressionFormat: u16,
    UncompressedBuffer: *mut u8,
    UncompressedBufferSize: usize,
    CompressedBuffer: *const u8,
    CompressedBufferSize: usize,
    FinalUncompressedSize: *mut usize,
) -> u32;

/// DllMain function typedef: \
pub type DllMain_t =
unsafe extern "system" fn(module: HMODULE, reason: u32, reserved: *mut c_void) -> BOOL;

/// typedef: \ for the equivalent of `int main(int argc, char* argv[]) { }`
pub type IntMainArgs = unsafe extern "C" fn(argc: i32, argv: *mut *mut u8) -> i32;

/// typedef: \ for the equivalent of `int main() { }`
pub type IntMainVoid = unsafe extern "system" fn() -> i32;

/// Windows GetLastError typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror>
pub type GetLastError_t = unsafe extern "system" fn() -> DWORD;

/// Windows GetProcessHeap typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-getprocessheap>
pub type GetProcessHeap_t = unsafe extern "system" fn() -> HANDLE;

/// Windows HeapAlloc typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapalloc>
pub type HeapAlloc_t =
unsafe extern "system" fn(h_heap: HANDLE, flags: DWORD, size: usize) -> LPVOID;

/// Windows HeapReAlloc typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heaprealloc>
pub type HeapReAlloc_t =
unsafe extern "system" fn(h_heap: HANDLE, flags: DWORD, mem: LPVOID, size: usize) -> LPVOID;

/// Windows HeapFree typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapfree>
pub type HeapFree_t = unsafe extern "system" fn(h_heap: HANDLE, flags: DWORD, mem: LPVOID) -> BOOL;

/// Windows HeapValidate typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/heapapi/nf-heapapi-heapvalidate>
pub type HeapValidate_t =
unsafe extern "system" fn(hHeap: HANDLE, dwFlags: u32, lpMem: *mut c_void) -> i32;

/// Windows InternetCrackUrlA typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetcrackurla>
pub type InternetCrackUrlA_t = unsafe extern "system" fn(
    lpszUrl: LPCSTR,
    dwUrlLength: DWORD,
    dwFlags: DWORD,
    lpUrlComponents: *mut URL_COMPONENTSA,
) -> BOOL;

/// Windows InternetOpenA typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetopena>
pub type InternetOpenA_t = unsafe extern "system" fn(
    lpszAgent: LPCSTR,
    dwAccessType: DWORD,
    lpszProxy: LPCSTR,
    lpszProxyBypass: LPCSTR,
    dwFlags: DWORD,
) -> HINTERNET;

/// Windows InternetConnectA typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetconnecta>
pub type InternetConnectA_t = unsafe extern "system" fn(
    hInternet: HINTERNET,
    lpszServerName: LPCSTR,
    nServerPort: c_ushort,
    lpszUserName: LPCSTR,
    lpszPassword: LPCSTR,
    dwService: DWORD,
    dwFlags: DWORD,
    dwContext: DWORD,
) -> HINTERNET;

/// Windows InternetSetOptionA typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetsetoptiona>
pub type InternetSetOptionA_t = unsafe extern "system" fn(
    hInternet: HINTERNET,
    dwOption: DWORD,
    lpBuffer: LPVOID,
    dwBufferLength: DWORD,
) -> BOOL;

/// Windows InternetReadFile typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetreadfile>
pub type InternetReadFile_t = unsafe extern "system" fn(
    hFile: HINTERNET,
    lpBuffer: LPVOID,
    dwNumberOfBytesToRead: DWORD,
    lpdwNumberOfBytesRead: LPDWORD,
) -> BOOL;

/// Windows InternetCloseHandle typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetclosehandle>
pub type InternetCloseHandle_t = unsafe extern "system" fn(hInternet: HINTERNET) -> BOOL;

/// Windows InternetQueryDataAvailable typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-internetquerydataavailable>
pub type InternetQueryDataAvailable_t = unsafe extern "system" fn(
    hFile: HINTERNET,
    lpdwNumberOfBytesAvailable: LPDWORD,
    dwFlags: DWORD,
    dwContext: DWORD,
) -> BOOL;

/// Windows HttpOpenRequestA typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-httpopenrequesta?redirectedfrom=MSDN>
pub type HttpOpenRequestA_t = unsafe extern "system" fn(
    hConnect: HINTERNET,
    lpszVerb: LPCSTR,
    lpszObjectName: LPCSTR,
    lpszVersion: LPCSTR,
    lpszReferrer: LPCSTR,
    lplpszAcceptTypes: *const LPCSTR,
    dwFlags: DWORD,
    dwContext: DWORD,
) -> HINTERNET;

/// Windows HttpSendRequestA typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-httpsendrequesta>
pub type HttpSendRequestA_t = unsafe extern "system" fn(
    hRequest: HINTERNET,
    lpszHeaders: LPCSTR,
    dwHeadersLength: DWORD,
    lpOptional: LPVOID,
    dwOptionalLength: DWORD,
) -> BOOL;

/// Windows HttpQueryInfoA typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/wininet/nf-wininet-httpqueryinfoa>
pub type HttpQueryInfoA_t = unsafe extern "system" fn(
    hRequest: HINTERNET,
    dwInfoLevel: DWORD,
    lpBuffer: LPVOID,
    lpdwBufferLength: LPDWORD,
    lpdwIndex: LPDWORD,
) -> BOOL;

/// Windows GetStdHandle typedef: \
/// <https://learn.microsoft.com/en-us/windows/console/getstdhandle>
pub type GetStdHandle_t = unsafe extern "system" fn(nStdHandle: DWORD) -> HANDLE;

/// Windows WriteConsole typedef: \
/// <https://learn.microsoft.com/en-us/windows/console/writeconsole>
pub type WriteConsoleA_t = unsafe extern "system" fn(
    hConsoleOutput: HANDLE,
    lpBuffer: *const c_void,
    nNumberOfCharsToWrite: DWORD,
    lpNumberOfCharsWritten: LPDWORD,
    lpReserved: LPVOID,
) -> BOOL;

/// Windows AllocConsole typedef: \
/// <https://learn.microsoft.com/en-us/windows/console/allocconsole>
pub type AllocConsole_t = unsafe extern "system" fn() -> BOOL;
#[allow(unused)]
pub type GetCommandLineW_t = unsafe extern "system" fn() -> LPWSTR;

/// Windows CommandLineToArgvW typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-commandlinetoargvw>
#[allow(unused)]
pub type CommandLineToArgvW_t =
unsafe extern "system" fn(lpCmdLine: LPCWSTR, pNumArgs: *mut i32) -> LPWSTR;

/// Windows TlsAlloc typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-tlsalloc>
pub type TlsAlloc_t = unsafe extern "system" fn() -> DWORD;

/// Windows TlsSetValue typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-tlssetvalue>
pub type TlsSetValue_t = unsafe extern "system" fn(dwTlsIndex: DWORD, lpTlsValue: LPVOID) -> BOOL;

/// Windows TlsGetValue typedef: \
/// <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-tlsgetvalue>
pub type TlsGetValue_t = unsafe extern "system" fn(dwTlsIndex: DWORD) -> LPVOID;

/// Windows LdrpReleaseTlsEntry typedef: \
/// Undocumented
pub type LdrpReleaseTlsEntry_t =
unsafe extern "system" fn(entry: *mut LDR_DATA_TABLE_ENTRY, unk: *mut c_void) -> NTSTATUS;

/// Windows RtlCompareMemory typedef: \
/// <https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/wdm/nf-wdm-rtlcomparememory>
pub type RtlCompareMemory_t = unsafe extern "system" fn(source1: *const c_void, source2: *const c_void, length: usize) -> u64;

/// Windows LdrpHandleTlsData typedef: \
/// Undocumented
pub type LdrpHandleTlsData_t = unsafe extern "system" fn(entry: *mut LDR_DATA_TABLE_ENTRY);
