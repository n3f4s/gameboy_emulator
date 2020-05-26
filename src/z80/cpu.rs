#![allow(dead_code)] // FIXME remove
#![allow(unused_variables)] // FIXME remove
#![allow(non_snake_case)]

#[macro_use]

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

impl std::convert::Into<u8> for Flags {
    fn into(self) -> u8 { self as u8 }
}

// macro_rules! CPU {
//     ( reg => $reg:expr ) => {
//         {
//             CPU {
//                 clock: cpu.clock,
//                 registers: $reg
//             }
//         }
//     };
//     ( clock => $clock:expr ) => {
//         {
//             CPU {
//                 clock: $clock,
//                 registers: cpu.registers,
//             }
//         }
//     };
// }

#[derive(Clone)]
pub struct CPU {
    pub registers: RegisterList,
    pub clock: Clock,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: RegisterList::new(),
            clock: Clock::default(),
        }
    }
    pub fn update_clock(&mut self) {
        self.clock.m += self.registers.clock.m;
        self.clock.t += self.registers.clock.t;
    }
    // pub fn push_stach(&self, mmu: &mut memory::MMU, val: Reg8) -> CPU {
    //     let cpu = CPU!( reg => self.registers.push_sp() );
    //     mmu.write_byte(cpu.registers.sp, val);
    //     cpu
    // }
    // pub fn pop_stack(&self, mmu: &mut memory::MMU, reg: Register) -> CPU {
    //     let res = mmu.read_byte(self.registers.sp, &self.registers);
    //     CPU!( reg => self.registers.pop_sp().set(reg, res) )
    // }
}
