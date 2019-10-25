ENTRY(start)
SECTIONS
 {
   . = 0x100000;
   .text : { *(.text*)  
		ce = .;
	}
	. = ALIGN(4096);
   .data : { *(.data*) 
		de = .;
	}
   .eh_frame : {
	   _eh_frame = .;
	   KEEP(*(.eh_frame*))
   }
   .rodata : { *(.rodata*) }
   .bss  : { *(.bss*)  }
   kernel_end = .;
 }
