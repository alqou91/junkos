.option norvc
.section .start, "ax", @progbits
.global trap
trap:
	j trap
