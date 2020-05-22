#![allow(dead_code)] // FIXME remove
#![allow(unused_variables)] // FIXME remove
// FIXME: see what the clock members represents
use z80::register::Reg8;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Clock {
    pub m: Reg8,
    pub t: Reg8,
}

impl Clock {
    pub fn tick(&mut self, cycle: u8) {
        self.m = cycle;
        self.t = 4  * cycle;
    }
}

