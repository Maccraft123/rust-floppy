ENTRY(_start)
OUTPUT_FORMAT(binary)

SECTIONS {
	. = 0x7c00;
	.text : {
		_start;
		*(.text .text.*);
	}
	
	.rodata : {
		*(.rodata .rodata.*);
	}
	
	.data : {
		*(.rodata .rodata.*);
		*(.data .data.*);
		*(.got .got.*);
	}

	. = 0x7c00 + 0x1fe;
	.magic : {
		SHORT(0xAA55)
	}
}
