;;kernel.asm
 
;nasm directive - 32 bit
bits 32
section .text
global start
start:
        ;multiboot spec
        align 4
        dd 0x1BADB002            ;магические числа
        dd 6                  ;флаги
        dd - (0x1BADB002+6 ) ;контрольная сумма. мч+ф+кс должно равняться нулю
        dd 0
        dd 0
        dd 0
        dd 0
        dd 0
        dd 1
        dd 80
        dd 25
        dd 0
 

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

global _out_fn
_out_fn:
  mov edx,[esp+4] ;данные
  mov eax,[esp+8] ;номер порта
  out dx,al
  ret
section .bss
bss_start:
resb 8192 ;8KB на стек
end:
