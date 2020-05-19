#![allow(dead_code)] // FIXME remove
#![allow(unused_variables)] // FIXME remove
use z80::clock::Clock;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Register {
    pub a: Reg8,
    pub b: Reg8,
    pub c: Reg8,
    pub d: Reg8,
    pub e: Reg8,
    pub h: Reg8,
    pub l: Reg8,
    pub f: Reg8,

    pub pc: Reg16,
    pub sp: Reg16,

    pub clock: Clock,
}

pub type Reg8 = u8; // 8 bit Register
pub type Reg16 = u16; // 16 bit Register
