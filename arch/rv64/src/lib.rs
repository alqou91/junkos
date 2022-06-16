#![no_std]
#![feature(naked_functions)]
#![feature(asm_sym)]

use core::arch::asm;
use csr::{Mode, Stvec};
use sbi::Sbi;

extern "C" {
	static _stack: usize;
}

fn trap_handler(cause: usize) {
}

pub fn init_irq() {
	Sbi::new().set_timer(10000);

	Stvec::read()
		.mode(Mode::Direct)
		.address(_trap as usize)
		.write();
}

#[naked]
extern "C" fn _trap() {
	unsafe {
		asm!("csrr a0, scause",
			 "jal {handler}",
			 "sret",
			 handler = sym trap_handler,
			 options(noreturn));
	}
}

#[naked]
#[link_section = ".start"]
#[export_name = "_start"]
extern "C" fn _start() {
	unsafe {
		asm!(
			"la sp, {stack}",
			// time sie
			"csrr a3, sie",
			"addi a3, a3, 0x20",
			"csrw sie, a3",
			// sstatus
			"csrr a3, sstatus",
			"addi a3, a3, 0x2",
			"csrw sstatus, a3",
			"j main",
			stack = sym _stack,
			options(noreturn));
	}
}
