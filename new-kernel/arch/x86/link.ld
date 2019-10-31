ENTRY(start)
SECTIONS
 {
   . = 0x100000;
   .text : { *(.text*)  
	code_e = .;
	}
	. = ALIGN(4096);
   .data : { 
	data_s = .;
	   *(.data*) 
	data_e = .;
	}
   .eh_frame : {
        _eh_frame = .;
        KEEP(*(.eh_frame*))
   }
   .rodata : { *(.rodata*) }
   .bss  : { 
	*(.bss*) 
	}
   kernel_end = .;
 }
