ENTRY(_start)

SECTIONS
{
	.start :
	{
		KEEP(*(.start));
	} > ram # rom

	.text :
	{
		*(.text .text.*);
		*(.rodata .rodata.*);
	} > ram # rom

	.relocate :
	{
		. = ALIGN(4);
		_relocate_start = .;

		*(.data .data.*);
		PROVIDE(_global_pointer = . + 0x800);

		. = ALIGN(4);
		_relocate_end = .;
	} > ram # rom

	.bss (NOLOAD) :
	{
		_bss_start = .;
		*(.bss .bss.*);
		_bss_end = .;
	} > ram

	PROVIDE(_stack = . + 0x8000);
}
