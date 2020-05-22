#![allow(dead_code)] // FIXME remove
#![allow(unused_variables)] // FIXME remove
use z80::clock::Clock;
use z80::cpu::Flags;

pub struct RegWord {
    pub low: u8,
    pub high: u8,
}

#[derive(Default)]
pub struct RegisterList {
    pub a: Reg8,
    pub b: Reg8,
    pub c: Reg8,
    pub d: Reg8,
    pub e: Reg8,
    pub h: Reg8,
    pub l: Reg8,

    pub f: Reg8, /// Flags

    pub pc: Reg16, /// Program Counter
    pub sp: Reg16, /// Stack pointer

    pub clock: Clock,
}

impl RegisterList {
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

    /// Set the flag according to the value of the register
    pub fn test_and_set(&mut self, reg: Reg8, carry: bool, op: bool) {
        self.f = 0;
        if carry { // overflow/underflow check
            self.f |= Flags::CARRY as u8
        }
        if op { // substraction check
            self.f |= Flags::OPERATION as u8
        }
        if reg == 0 { // check if the result of the last operation is 0
            self.f |= Flags::ZERO as u8;
        }
        if reg > 15 { // check overflow for the lower part of the byte
            self.f |= Flags::HALFCARRY as u8;
        }
    }

    /// Do a 8bit increment on a register
    pub fn incr(&mut self, reg: Reg8) -> Reg8 {
        let (new, carry) = reg.overflowing_add(1);
        self.test_and_set(reg, carry, false);
        new
    }

    /// Do a 16bit increment on two registers
    pub fn incr_word(&mut self, low: Reg8, high: Reg8) -> RegWord {
        let (newl, carry) = low.overflowing_add(1);
        let mut newh = high;
        if carry {
            let (new, carry) = high.overflowing_add(1);
            newh = new;
            self.test_and_set(newh, carry, false);
        }
        RegWord {
            high: newh,
            low: newl,
        }
    }

    /// Do a 8bit decrement on a register
    pub fn decr(&mut self, reg: Reg8) -> Reg8 {
        let (new, carry) = reg.overflowing_sub(1);
        self.test_and_set(reg, carry, true);
        new
    }

    /// Do a 16bit decrement on two registers
    pub fn decr_word(&mut self, low: Reg8, high: Reg8) -> RegWord {
        let (newh, carry) = high.overflowing_sub(1);
        let mut newl = low;
        if carry {
            let (new, carry) = low.overflowing_sub(1);
            newl = new;
            self.test_and_set(newl, carry, true);
        }
        RegWord {
            low: newl,
            high: newh,
        }
    }

    pub fn xor(&mut self, lhs: Reg8, rhs: Reg8) -> Reg8 {
        let res = lhs ^ rhs;
        self.test_and_set(res, false, false);
        res
    }

    pub fn add(&mut self, lhs: u8, rhs: u8) -> Reg8 {
        let (new, carry) = lhs.overflowing_add(rhs);
        self.test_and_set(new, carry, false);
        new
    }

    pub fn new() -> RegisterList {
        RegisterList::default()
    }
}

pub type Reg8 = u8; // 8 bit Register
pub type Reg16 = u16; // 16 bit Register
