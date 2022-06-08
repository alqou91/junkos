MEMORY
{
	rom(rx)  : ORIGIN = 0x20000000, LENGTH = 64m
	ram(rwx) : ORIGIN = 0x81000000, LENGTH = 1024m
}

INCLUDE boards/linker.ld
