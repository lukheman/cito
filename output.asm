section .data
    string db '299991', 0xa
    stringLen equ $ - string
section .text
    global _start
_start:
    mov eax, 4
    mov ebx, 1
    mov ecx, string
    mov edx, stringLen
    int 0x80
    mov eax, 1
    int 0x80
