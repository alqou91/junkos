.option norvc
.section .start, "ax", @progbits
.global _start
_start:
	la sp, _stack
	j main
