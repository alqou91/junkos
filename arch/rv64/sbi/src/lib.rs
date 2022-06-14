#![no_std]
use core::arch::asm;

pub enum EID {
	//BASE = 0x10,
	TIMER = 0x54494D45,
}

pub enum Error {
	Success = 0,
	Failed = -1,
	NotSupported = -2,
	InvalidParam = -3,
	Denied = -4,
	InvalidAddress = -5,
	AlreadyAvailable = -6,
	AlreadyStarted = -7,
	AlreadyStopped = -8,
}

pub struct Sbi {
}

impl Sbi {
	pub fn new() -> Self {
		Sbi {}
	}

	fn call(&self, eid: usize, fid: usize, arg0: usize, arg1: usize) -> (usize, usize) {
		let mut error: usize;
		let mut value: usize;

		unsafe {
			#[cfg(any(target_arch = "riscv64"))]
			asm!("ecall",
				 in("a0") arg0,
				 in("a1") arg1,
				 in("a6") fid,
				 in("a7") eid,
				 lateout("a0") error,
				 lateout("a1") value,
			);
		}

		(error, value)
	}

	pub fn set_timer(&self, time: usize) {
		let (error, _) = self.call(EID::TIMER as usize, 0 as usize, time, 0);

		match error {
			0 => (),
			_ => loop {},
		}
	}
}
