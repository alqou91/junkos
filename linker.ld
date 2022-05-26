SECTIONS
{
	.start :
	{
		*(.start .start.*);
	} > rom

	.text :
	{
		*(.text .text.*);
		*(.rodata .rodata.*);
	} > rom

	.relocate :
	{
		. = ALIGN(4);
		_relocate_start = .;

		*(.data .data.*);

		. = ALIGN(4);
		_relocate_end = .;
	} > rom

	.bss (NOLOAD) :
	{
		_bss_start = .;
		*(.bss .bss.*);
		_bss_end = .;
	} > ram

	.stack (NOLOAD) :
	{
		. = ALIGN(8);
		_stack_start = .;

		KEEP(*(.stack))

		. = ALIGN(8);
		_stack_end = .;
	} > ram
}
