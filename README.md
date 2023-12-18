# DebugInfo

This is a tool to get the base debug info in object files.

​         

Supported file formats:
* PE
* PDB
* MachO
* ELF

​      

Extracted Basic Information:
* FileFormat
* Arch
* BuildId
* BreakpadUUID
* PdbFileName (only for PE files)
* HasSymbolTable
* HasDebugInfo
* HasUnwindInfo

​       


# Usage

```
$ debugid <input_file>
FileFormat: elf
Arch: arm64
SymbolTable: true
DebugInfo: true
UnwindInfo: true
BuildId: 433bfdc95e137f439e209d8ee8f07ffe92d9af4c
BreakpadUUID: C9FD3B43135E437F9E209D8EE8F07FFE0
```