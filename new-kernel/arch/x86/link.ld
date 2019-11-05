ENTRY(start)
SECTIONS{
	. = 0x100000;
	.text : {
		*(.text*)  
	}
   	.data : { 
		. = ALIGN(4096);
	   	*(.data*) 
	}
   	.eh_frame : {
		. = ALIGN(4096);

        	_eh_frame = .;
        	KEEP(*(.eh_frame*))
   	}
   	.rodata : {
		. = ALIGN(4096);
		*(.rodata*) }
   	.rel.text : {
		. = ALIGN(4096);
		*(.rel.text*)
   	}
   	.bss  : { 
		. = ALIGN(4096);
		*(.bss*) 
	}
 }
