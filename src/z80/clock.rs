#![allow(dead_code)] // FIXME remove
#![allow(unused_variables)] // FIXME remove
// FIXME: see what the clock members represents
use z80::register::Reg16;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Clock {
    pub m: Reg16,
    pub t: Reg16,
}

impl Clock {
    pub fn tick(cycle: u16) -> Clock {
        Clock {
            m: cycle,
            t: 4 * cycle,
        }
    }
}

