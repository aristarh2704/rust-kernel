;;kernel.asm
 
;nasm directive - 32 bit
section .text
bits 32
extern _kmain         ;kmain определена во внешнем файле
extern code_end
global start
hd_start:
        ;multiboot spec
        align 4
        dd 0xE85250D6            ;магические числа
        dd 0
        dd header_end-hd_start
        dd 0x100000000-(0xE85250D6+header_end-hd_start)   ;контрольная сумма. мч+ф+кс должно равняться нулю

mbi_tag:dw 1
        dw 0
        dd 28
        dd 1
        dd 6
        dd 2
        dd 8
        dd 11
	dd 0
mbi_end:
        ;dd 0
fb_tag: dw 5
        dw 0
        dd 20
        dd 800
	dd 600
	dd 24
	dd 0
fb_end:
end_tag:dw 0
        dw 0
        dd 8
header_end:
VIDEO_MEMORY equ 0xb8000
WHITE_ON_BLACK equ 0x0f ; the color byte for each character

print_string_pm:
    pusha
    mov edx, VIDEO_MEMORY

print_string_pm_loop:
    mov al, [ebx] ; [ebx] is the address of our character
    mov ah, WHITE_ON_BLACK

    cmp al, 0 ; check if end of string
    je print_string_pm_done

    mov [edx], ax ; store character + attribute in video memory
    add ebx, 1 ; next char
    add edx, 2 ; next video memory position

    jmp print_string_pm_loop

print_string_pm_done:
    popa
    ret
start:
  cli ;блокировка прерываний
  mov esp, end ;указатель стека
  push end
  push bss_start
  push code_end
  push start
  push ebx
  push eax
  call _kmain
  hlt ;остановка процессора

global _out_fn
_out_fn:
  mov edx,[esp+4] ;данные
  mov eax,[esp+8] ;номер порта
  out dx,al
  ret
global _rust_begin_unwind2
_rust_begin_unwind2: 
	cli
	mov ebx,msg
	call print_string_pm
	hlt
msg: db 'RUST_BEGIN_UNWIND!',0
hello: db 'HELLO WORLD!',0
section .bss
bss_start:
resb 8192 ;8KB на стек
end:
