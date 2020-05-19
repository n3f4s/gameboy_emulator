#![allow(dead_code)] // FIXME remove
#![allow(unused_variables)] // FIXME remove
#![allow(non_snake_case)]
use z80::memory;
use z80::register::*;
use z80::clock::*;


pub const ZERO: Reg8 = 0x80;
pub const OPERATION: Reg8 = 0x40;
pub const HALFCARRY: Reg8 = 0x20;
pub const CARRY: Reg8 = 0x10;

#[repr(u8)]
pub enum Flags {
    ZERO      = 0x80,
    OPERATION = 0x40,
    HALFCARRY = 0x20,
    CARRY     = 0x10,
}

pub struct CPU {
    pub registers: Register,
    pub clock: Clock,
    pub mmu: memory::MMU, // FIXME doit être une référence
}

impl CPU {
    pub fn new(mmu: memory::MMU) -> CPU {
        CPU {
            registers: Register::default(),
            clock: Clock::default(),
            mmu: mmu
        }
    }
    pub fn update_clock(&mut self) {
        self.clock.m += self.registers.clock.m;
        self.clock.t += self.registers.clock.t;
    }
}
