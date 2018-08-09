;;kernel.asm
 
;nasm directive - 32 bit
section .text
bits 32
extern _kmain         ;kmain определена во внешнем файле
extern kernel_end
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
start:
  mov esp, end ;указатель стека
  push kernel_end
  push hd_start
  push ebx
  call _kmain
  hlt ;остановка процессора

global _out_fn
_out_fn:
  mov edx,[esp+4] ;данные
  mov eax,[esp+8] ;номер порта
  out dx,al
  ret
section .bss
resb 8192 ;8KB на стек
end:
