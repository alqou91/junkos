#![no_std]

use core::arch::asm;

pub enum Mode {
    Direct = 0,
    Vectored = 1,
}

pub struct Stvec {
    val: usize,
    mask: usize,
}

impl Stvec {
    pub fn address(mut self, address: usize) -> Self {
		let mask = 0xFFFFFFFF_FFFFFFFC;
		self.val |= address & mask;
		self.mask |= mask;
		self
	}

    pub fn mode(mut self, mode: Mode) -> Self {
		let mask = 0x00000000_00000003;
		self.val &= mask;
		self.val |= mode as usize;
		self.mask |= mask;
		self
    }

    pub fn read() -> Self {
		Stvec {
    		val: Stvec::get(),
			mask: 0,
		}
    }

	pub fn val() -> Self {
		Stvec {
			val: 0,
			mask: 0,
		}
	}

    pub fn get_address(&self) -> usize {
		self.val >> 2 << 2
    }

    pub fn get_mode(&self) -> Option<Mode> {
		match self.val & 0x00000000_00000003 {
			0 => Some(Mode::Direct),
			1 => Some(Mode::Vectored),
			_ => None
		}
    }

    pub fn write(&self) {
        Stvec::set(self.val);
    }

    fn set(val: usize) {
        unsafe {
            asm!("csrw stvec, {reg}", reg = in(reg) val);
        }
    }

    fn get() -> usize {
        let reg: usize;
        unsafe {
            asm!("csrr {reg}, stvec", reg = out(reg) reg);
        }
        reg
    }
}
