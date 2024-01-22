section .bss 

buffer: resb 1024

section .text

global _start
_start:
    mov ebx, file
    mov eax, 5  
    mov ecx, 0  
    int 80h     

    mov eax, 3  
    mov ebx, eax
    mov ecx, buffer 
    mov edx, len    
    int 80h     

    mov eax, 4  
    mov ebx, 1
    mov ecx, buffer 
    mov edx, len    
    int 80h     

    mov eax, 6  
    int 80h     

    mov eax, 1
    mov ebx, 0 
    int 80h

section .data

file db "../input/01", 0

len equ 1024
