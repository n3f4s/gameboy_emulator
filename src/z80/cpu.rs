#![allow(dead_code)] // FIXME remove
#![allow(unused_variables)] // FIXME remove
#![allow(non_snake_case)]
use z80::memory;
use z80::register::*;
use z80::clock::*;

#[repr(u8)]
pub enum Flags {
 /// The last operation returned 0
    ZERO      = 0x80,
    /// The last operation was a substraction
    OPERATION = 0x40,
    /// The lower part of the byte overflowed during last operation
    HALFCARRY = 0x20,
    /// The byte overflowed during last operation
    CARRY     = 0x10,
}

pub struct CPU {
    pub registers: RegisterList,
    pub clock: Clock,
    pub mmu: memory::MMU, // FIXME doit être une référence
}

impl CPU {
    pub fn new(mmu: memory::MMU) -> CPU {
        CPU {
            registers: RegisterList::new(),
            clock: Clock::default(),
            mmu: mmu
        }
    }
    pub fn update_clock(&mut self) {
        self.clock.m += self.registers.clock.m;
        self.clock.t += self.registers.clock.t;
    }
}
