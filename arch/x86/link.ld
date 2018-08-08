ENTRY(start)
SECTIONS
 {
   . = 0x100000;
   //.boot : { *(.boot) }
   .text : { *(.text)   }
   .data : { *(.data) }
   .bss  : { *(.bss)  }
 }
