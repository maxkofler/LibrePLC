# Memory Protection

Use `mprotect()` to set everything to read-only (except the stack and the process data area).
The memory maps can be obtained through `/proc/self/maps`
