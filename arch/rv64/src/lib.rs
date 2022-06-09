#![no_std]

use sbi;

pub fn timer_test() {
	let sbi = sbi::Sbi::new();
	sbi.set_timer(100);
}
