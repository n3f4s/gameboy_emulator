#![allow(dead_code)] // FIXME remove
#![allow(unused_variables)] // FIXME remove
use z80::clock::Clock;
use z80::cpu::Flags;
use z80::memory::MMU;

pub struct RegWord {
    pub low: u8,
    pub high: u8,
}

// FIXME impl add and sub for 16bit


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

    pub fn get_flag(&self, flag: Flags) -> bool {
        self.f & (flag as u8) != 0
    }

    /// Set a flag if needed
    pub fn set_flag(&self, flag: Flags, need_set: bool) -> RegisterList {
        RegisterList {
            reg: self.reg,
            clock: self.clock,
            pc: self.pc,
            sp: self.sp,
            f: if need_set { self.f | flag as u8} else { self.f }
        }
    }
    /// Reset a flag
    pub fn reset_flag(&self, flag: Flags) -> RegisterList {
        RegisterList {
            reg: self.reg,
            clock: self.clock,
            pc: self.pc,
            sp: self.sp,
            f: self.f & !(flag as u8)
        }
    }

    fn test_add_hc(lhs: Reg8, rhs: Reg8) -> bool {
        (lhs & 0xf)  + (rhs & 0xf) & 0x10 == 0x10
    }
    fn test_sub_hc(lhs: Reg8, rhs: Reg8) -> bool { // FIXME check the maths
        (lhs & 0xf)  - (rhs & 0xf) & 0x10 == 0x10
    }

    /// Do a 8bit increment on a register (b) and set the flags:
    /// zero if value is zero, reset the operation flag, set the half carry flag if needed
    /// doesn't affect the carry flag
    pub fn incr(&self, reg: Register) -> RegisterList {
        let (new, carry) = self.get(reg).overflowing_add(1);
        self.set(reg, new)
            .set_flag(Flags::ZERO, new == 0)
            .reset_flag(Flags::OPERATION)
            .set_flag(Flags::HALFCARRY, new & 0b1000 != 0) // if carry from bit 3
    }

    /// Do a 16bit increment on two registers
    pub fn incr_word(&self, low: Register, high: Register) -> RegisterList {
        let (newl, c) = self.get(low).overflowing_add(1);
        let (newh, carry) = if c {
            self.get(high).overflowing_add(1)
        } else {
            (self.get(high), false)
        };
        self.set(low, newl).set(high, newh)
    }

    /// Do a 8bit decrement on a register and set the flags
    pub fn decr(&self, reg: Register) -> RegisterList {
        let (new, carry) = self.get(reg).overflowing_sub(1);
        self.set(reg, new)
            .set_flag(Flags::ZERO, new == 0)
            .set_flag(Flags::OPERATION, true)
            .set_flag(Flags::HALFCARRY, new & 0b111 == 0) // if no borrow from bit 4
    }

    /// Do a 16bit decrement on two registers
    pub fn decr_word(&self, low: Register, high: Register) -> RegisterList {
        let (newl, c) = self.get(low).overflowing_sub(1);
        let (newh, _carry) = if c {
            self.get(high).overflowing_sub(1)
        } else {
            (self.get(high), false)
        };
        self.set(low, newl).set(high, newh)
    }

    /// Do an exclusive or on 2 register and set the flags
    pub fn xor(&self, lhs: Register, rhs: Register) -> RegisterList {
        let res = self.get(lhs) ^ self.get(rhs);
        self.set(lhs, res)
            .set_flag(Flags::ZERO, res ==0)
            .reset_flag(Flags::CARRY)
            .reset_flag(Flags::HALFCARRY)
            .reset_flag(Flags::OPERATION)
    }

    /// Add two register and set the flags
    pub fn add(&self, lhs: Register, rhs: Reg8) -> RegisterList {
        let (res, carry) = self.get(lhs).overflowing_add(rhs);
        self.reset_flag(Flags::OPERATION)
            .set_flag(Flags::ZERO, res == 0)
            .set_flag(Flags::CARRY, carry)
            .set_flag(Flags::HALFCARRY, RegisterList::test_add_hc(self.get(lhs), rhs))
            .set(lhs, res)
    }

    pub fn add16(&self,
                 left_high: Register, left_low: Register,
                 right_high: Register, right_low: Register) -> RegisterList {
        let (low, hcarry) = self.get(left_low).overflowing_add(self.get(right_low));
        let (high, carry) = self.get(left_high).overflowing_add(self.get(right_high) + (if hcarry {1} else {0}));
        self
            .reset_flag(Flags::OPERATION)
            .set_flag(Flags::HALFCARRY, RegisterList::test_add_hc(self.get(left_high), self.get(right_high)))
            .set_flag(Flags::CARRY, carry)
            .set(left_high, high)
            .set(left_low, low)
    }

    /// Compare two registers:
    /// The result is computed by substracting lhs and rhs and setting the flags according to the
    /// result of the substraction
    /// Underflow -> B > A
    /// Zero      -> B = A
    /// Nothing   -> B < A
    pub fn cmp(&self, lhs: Register, rhs: Register) -> RegisterList {
        let (res, carry) = self.get(lhs).overflowing_sub(self.get(rhs));
        self.set_flag(Flags::OPERATION, true)
            .set_flag(Flags::ZERO, res == 0)
            .set_flag(Flags::HALFCARRY, RegisterList::test_sub_hc(self.get(lhs), self.get(rhs))) // Set if no borrow from bit 4
            .set_flag(Flags::CARRY, !carry)
    }

    /// Substract two register and set the flags
    pub fn sub(&mut self, lhs: Register, rhs: Reg8) -> RegisterList {
        let (res, carry) = self.get(lhs).overflowing_sub(rhs);
        self.set_flag(Flags::OPERATION, true)
            .set_flag(Flags::ZERO, res == 0)
            .set_flag(Flags::HALFCARRY, !RegisterList::test_sub_hc(self.get(lhs), rhs)) // Set if no borrow from bit 4
            .set_flag(Flags::CARRY, !carry) // Set if no borrow
            .set(lhs, res)
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
