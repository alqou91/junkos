#![no_std]

use csr::{Mode, Stvec};

pub fn init_irq() {
    Stvec::read()
        .mode(Mode::Direct)
        .address(0x9908)
        .write();
}
