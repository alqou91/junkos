#![no_std]
#![feature(naked_functions)]

use core::arch::asm;

extern "C" {
	static _stack: usize;
	static _global_pointer: usize;
}

#[link_section = ".start"]
#[no_mangle]
pub extern "C" fn trap() {
	loop {}
}


#[link_section = ".start"]
#[no_mangle]
#[naked]
pub extern "C" fn _start()  -> !
{
	unsafe {
		asm!(
			"csrw mie, zero",
			"csrw mip, zero",
			"la sp, _stack",
			"la gp, _global_pointer",
			"li a0, -1",
			"csrw pmpaddr0, a0",
			"li a0, 0x1f",
			"csrw pmpcfg0, a0",
			"la a1, trap",
			"csrw mtvec, a1",
			"li t2, 0x800",
			"csrw mie, t2",
			"csrsi mstatus, 0x8",
			"csrw mscratch, zero",
			"j main",
			options(noreturn));
	}
}
