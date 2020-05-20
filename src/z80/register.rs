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

impl Register {
    pub fn format(&self) -> String {
        format!(
"--------------------------------------------------------------------------
|  a  |  b  |  c  |  d  |  e  |  h  |  l  |  f  ||  sp  ||  pc  || clock |
| {:03X} | {:03X} | {:03X} | {:03X} | {:03X} | {:03X} | {:03X} | {:03X} || {:04X} || {:04X} || {:02}:{:02} |
--------------------------------------------------------------------------",
               self.a, self.b, self.c, self.d, self.e,
               self.h, self.l, self.f, self.sp, self.pc,
               self.clock.m, self.clock.t)
    }
}

pub type Reg8 = u8; // 8 bit Register
pub type Reg16 = u16; // 16 bit Register
