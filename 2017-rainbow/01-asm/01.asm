format ELF64 executable

macro open filename, flags, mode
{
	mov rax, 2
	mov rsi, flags
	mov rdx, mode
	syscall
}

segment readable executable
entry main
main:
	open
	mov r10, rax
	mov rax, 1
	mov rdi, 1
	mov rsi, [r10]
	mov rdx, 10
	syscall

	mov rax, 60
	mov rdi, 0
	syscall
 
segment readable writeable
