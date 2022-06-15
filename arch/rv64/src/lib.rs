#![no_std]
#![feature(naked_functions)]
#![feature(asm_sym)]

use core::arch::asm;
use csr::{Mode, Stvec};

extern "C" {
	static _stack: usize;
}

fn trap_handler() {
}

pub fn init_irq() {
	Stvec::read()
		.mode(Mode::Direct)
		.address(_trap_handler as usize)
		.write();
}

#[naked]
pub extern "C" fn _trap_handler() {
	unsafe {
		asm!(
			"j {handler}",
			 "sret",
			 handler = sym trap_handler,
			 options(noreturn));
	}
}

#[naked]
#[link_section = ".start"]
#[export_name = "_start"]
pub extern "C" fn _start() {
	unsafe {
		asm!(
			"la sp, {stack}",
			"j main",
			stack = sym _stack,
			options(noreturn));
	}
}
