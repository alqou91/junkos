.option norvc
.section .start, "ax", @progbits
.global _start
_start:
	csrw mie, zero
	csrw mip, zero
	la sp, _stack
	la gp, _global_pointer
	li a0, -1
	csrw pmpaddr0, a0
	li a0, 0x1f
	csrw pmpcfg0, a0
	la a1, trap
	csrw mtvec, a1
	li t2, 0x800
	csrw mie, t2
	csrsi mstatus, 0x8
	csrw mscratch, zero
	j main
