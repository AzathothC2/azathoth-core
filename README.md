# Azathoth_core
![Crates.io Version](https://img.shields.io/crates/v/azathoth-core)
[README.md](README.md)
`azathoth_core` provides low-level, platform-specific definitions and utilities used across the [Azathoth c2 framework](https://github.com/AzathothC2/)

It serves a `no_std`-compatible layer, containing:
* Platform-specific structs, constants and type definitions for Windows and Linux
* A `crc32` lookup table for fast hashing
* The `AzError` trait error interface shared across the rest of the Azathoth Crates

> [!WARNING]
> This project is still in active development and may change at any time!

## Installation
* Manually, via `Cargo.toml`: `azathoth_core = "0.1.2"`
* Using the `cargo` cli: `cargo add azathoth_core`


## Supported Platform Definitions
The following structs (and related types) are included:
* Windows:
    * `Guid` 
    * Windows Internals:
        * `TEB` (Thread Environment Block)
        * `PEB` (Process Environment Block)
        * `RTL_USER_PROCESS_PARAMETERS`
        * `UNICODE_STRING`
        * `LIST_ENTRY`
        * `LDR_DATA_TABLE_ENTRY`
        * `PEB_LDR_DATA`
    * PE/COFF structures:
        * `IMAGE_DOS_HEADER`
        * `IMAGE_NT_HEADERS64`
        * `IMAGE_FILE_HEADER`
        * `IMAGE_OPTIONAL_HEADER64`
        * `IMAGE_DATA_DIRECTORY`
        * `IMAGE_EXPORT_DIRECTORY`
        * `IMAGE_SECTION_HEADER`
        * `IMAGE_BASE_RELOCATION`
        * `IMAGE_IMPORT_DESCRIPTOR`
        * `IMAGE_IMPORT_DESCRIPTOR_0` (union)
        * `IMAGE_THUNK_DATA64`
        * `IMAGE_THUNK_DATA64_0` (union)
        * `IMAGE_IMPORT_BY_NAME`
        * `IMAGE_TLS_DIRECTORY64`
    * `URL_COMPONENTSA`
    * `RUNTIME_FUNCTION`
* Linux:
    * `Elf64Ehdr` (ELF64 File header)
    * `Elf64Dyn` (ELF64 dynamic table entry)
    * `Elf64Rela` (ELF64 Relocation entries)
    * `Elf64Sym` (Symbol table entry)
    * `Elf64Phdr` (ELF64 Program header)
    * `Elf64Shdr` (ELF64 section header)


## Changelog

- 0.1.0: Initial release
- 0.1.1: Re-exported CRC32 hashing table
- 0.1.2: Added Dotnet definitions (locked behind `dotnet` feature. **Windows only**)
- 0.1.3: Made `os` modules feature-dependent. You can now use the `windows` structs on linux and vice versa