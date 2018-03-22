;;kernel.asm
 
;nasm directive - 32 bit
bits 32
section .text
global start
start:
        ;multiboot spec
        align 4
        dd 0x1BADB002            ;магические числа
        dd 64                  ;флаги
        dd - (0x1BADB002 + 64) ;контрольная сумма. мч+ф+кс должно равняться нулю
 

extern _kmain         ;kmain определена во внешнем файле
extern code_end

  cli ;блокировка прерываний
  mov esp, end ;указатель стека
  push end
  push bss_start
  push code_end
  push start
  push ebx
  call _kmain
  hlt ;остановка процессора

section .bss
bss_start:
resb 8192 ;8KB на стек
end:
