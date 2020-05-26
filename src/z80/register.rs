#![allow(dead_code)] // FIXME remove
#![allow(unused_variables)] // FIXME remove
use z80::clock::Clock;
use z80::cpu::Flags;
use z80::memory::MMU;

pub struct RegWord {
    pub low: u8,
    pub high: u8,
}

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    H = 5,
    L = 6,
}

impl std::convert::Into<usize> for Register {
    fn into(self) -> usize { self as usize }
}

#[derive(Default, Clone)]
pub struct RegisterList {
    reg: [Reg8; 7],

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
               self.get(Register::A), self.get(Register::B), self.get(Register::C),
               self.get(Register::D), self.get(Register::E),
               self.get(Register::H), self.get(Register::L), self.f, self.sp, self.pc,
               self.clock.m, self.clock.t)
    }

    pub fn get(&self, reg: Register) -> Reg8 {
        self.reg[reg as usize]
    }
    pub fn set(&self, reg: Register, value: Reg8) -> RegisterList {
        let mut res = self.clone();
        res.reg[reg as usize] = value;
        res
    }

    /// Compute the flags according to the result of the operation
    pub fn test_and_set(&self, reg: Reg8, carry: bool, op: bool) -> RegisterList {
        let carryf = if carry { // overflow/underflow check
            Flags::CARRY as u8
        } else {
            0
        };
        let opf = if op { // substraction check
            Flags::OPERATION as u8
        } else {
            0
        };
        let zerof = if reg == 0 { // check if the result of the last operation is 0
            Flags::ZERO as u8
        } else {
            0
        };
        let halff = if reg > 15 { // check overflow for the lower part of the byte
            Flags::HALFCARRY as u8
        } else {
            0
        };
        RegisterList {
            reg: self.reg,
            clock: self.clock,
            pc: self.pc,
            sp: self.sp,
            f: carryf | opf | zerof | halff
        }
    }

    /// Do a 8bit increment on a register and set the flags
    pub fn incr(&self, reg: Register) -> RegisterList {
        let (new, carry) = self.get(reg).overflowing_add(1);
        self.set(reg, new).test_and_set(new, carry, false)
    }

    // FIXME utiliser le même pattern fonctionnel pour toutes les fonctions
    // FIXME enlever mmu de CPU et utiliser le paramètre des opcodes
    // FIXME transformer les instructions pour ne plus modifier le CPU mais renvoyer un nouveau
    /// Do a 16bit increment on two registers and set the flags
    pub fn incr_word(&self, low: Register, high: Register) -> RegisterList {
        let (newl, c) = self.get(low).overflowing_add(1);
        let (newh, carry) = if c {
            self.get(high).overflowing_add(1)
        } else {
            (self.get(high), false)
        };
        self.set(low, newl).set(high, newh).test_and_set(newh, carry, false)
    }

    /// Do a 8bit decrement on a register and set the flags
    pub fn decr(&mut self, reg: Register) -> RegisterList {
        let (new, carry) = self.get(reg).overflowing_sub(1);
        self.set(reg, new).test_and_set(new, carry, true)
    }

    /// Do a 16bit decrement on two registers and set the flags
    pub fn decr_word(&self, low: Register, high: Register) -> RegisterList {
        let (newl, c) = self.get(low).overflowing_sub(1);
        let (newh, carry) = if c {
            self.get(high).overflowing_sub(1)
        } else {
            (self.get(high), false)
        };
        self.set(low, newl).set(high, newh).test_and_set(newh, carry, false)
    }

    /// Do an exclusive or on 2 register and set the flags
    pub fn xor(&self, lhs: Register, rhs: Register) -> RegisterList {
        let res = self.get(lhs) ^ self.get(rhs);
        self.set(lhs, res).test_and_set(res, false, false)
    }

    /// Add two register and set the flags
    pub fn add(&self, lhs: Register, rhs: Register) -> RegisterList {
        let (res, carry) = self.get(lhs).overflowing_add(self.get(rhs));
        self.set(lhs, res).test_and_set(res, carry, false)
    }

    /// Compare two registers:
    /// The result is computed by substracting lhs and rhs and setting the flags according to the
    /// result of the substraction
    /// Underflow -> B > A
    /// Zero      -> B = A
    /// Nothing   -> B < A
    pub fn cmp(&self, lhs: Register, rhs: Register) -> RegisterList {
        let (res, carry) = self.get(lhs).overflowing_sub(self.get(rhs));
        self.test_and_set(res, carry, false)
    }

    /// Substract two register and set the flags
    pub fn sub(&mut self, lhs: Register, rhs: Register) -> RegisterList {
        let (res, carry) = self.get(lhs).overflowing_sub(self.get(rhs));
        self.set(lhs, res).test_and_set(res, carry, true)
    }

    pub fn set_clock(&self, clock: Clock) -> RegisterList {
        RegisterList {
            reg: self.reg,

            f: self.f, /// Flags

            pc: self.pc, /// Program Counter
            sp: self.sp, /// Stack pointer

            clock: clock,
        }
    }

    pub fn incr_pc(&self, pc_i: Reg16) -> RegisterList {
        RegisterList {
            reg: self.reg,

            f: self.f, /// Flags

            pc: self.pc + pc_i, /// Program Counter
            sp: self.sp, /// Stack pointer

            clock: self.clock,
        }
    }

    /// Load 16 bit into SP
    pub fn load_sp(&self, mmu: &mut MMU) -> RegisterList {
        RegisterList {
            reg: self.reg,
            f: self.f,
            pc: self.pc,
            sp: mmu.read_word(self.pc, &self),
            clock: self.clock,
        }
    }
    pub fn push_sp(&self) -> RegisterList {
        RegisterList {
            reg: self.reg,
            f: self.f,
            pc: self.pc,
            sp: self.sp-1,
            clock: self.clock,
        }
    }
    pub fn pop_sp(&self) -> RegisterList {
        RegisterList {
            reg: self.reg,
            f: self.f,
            pc: self.pc,
            sp: self.sp+1,
            clock: self.clock,
        }
    }

    pub fn push_stach(&self, mmu: &mut MMU, val: Reg8) -> RegisterList {
        let cpu = self.push_sp();
        mmu.write_byte(cpu.sp, val);
        cpu
    }
    pub fn pop_stack(&self, mmu: &mut MMU, reg: Register) -> RegisterList {
        let res = mmu.read_byte(self.sp, &self);
        self.pop_sp().set(reg, res)
    }

    pub fn new() -> RegisterList {
        RegisterList::default()
    }
}

pub type Reg8 = u8; // 8 bit Register
pub type Reg16 = u16; // 16 bit Register
